//! Milestone M4's exit test and the image round trip (whitepaper Appendix C; brief 015;
//! ADR-0024): an image written from arenas and opened again is byte-identical and runs
//! identically; a corrupted, truncated or foreign image fails closed; a synapse delay the wheel
//! cannot hold is refused at load; and evict, spike, re-hydrate preserves a unit bit for bit:
//! a run that sweeps and re-hydrates ends in the same image as one that never evicts.

use cortex_connectome::{
    CortexFileHeader, HeaderError, SECTION_NEURON, SECTION_PLASTIC_DELTA, SECTION_SYNAPSE,
    SectionEntry, crc64,
};
use cortex_core::{
    PlasticDelta, STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16,
};
use cortex_runtime::{Config, Executor, Image, ImageError, WriteAheadLog};
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("vcortex-image-tests");
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir.join(format!("{}-{name}", std::process::id()))
}

fn config() -> Config {
    Config {
        workers: 2,
        units: 96,
        blocks: 96,
        deltas: 8,
        nodes_per_worker: 4096,
        injector_capacity: 1024,
        trace_capacity: 1 << 14,
        ..Config::default()
    }
}

/// A pseudo-random network of 96 units, one block each, a few deltas.
fn wire(exec: &mut Executor<64>) {
    let mut x = 0x2545_F491u32;
    let mut next = || {
        x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        x
    };
    let units = exec.units().len();
    {
        let blocks = exec.blocks_mut();
        for block in blocks.iter_mut() {
            for slot in 0..4 {
                let target = (next() >> 8) % units as u32;
                let delay = match next() % 6 {
                    0 => 0,
                    _ => 1 + (next() >> 8) % 300,
                } as u16;
                let weight = ((next() >> 8) % 20_000) as i16 + 10_000;
                assert!(block.set_synapse(slot, target, weight, delay, next() % 4 == 0));
            }
        }
    }
    {
        let deltas = exec.deltas_mut();
        for (i, d) in deltas.iter_mut().enumerate() {
            *d = PlasticDelta::new(i as u32, (i % 4) as u8, -100 * i as i16, 7).unwrap();
        }
    }
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
        assert!(unit.set_first_block(i as u32));
        if i < 8 {
            assert!(unit.set_delta_head(i as u32));
        }
    }
}

fn kick(exec: &Executor<64>, unit: u32, count: usize) {
    let inject = exec.injector();
    let one = synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX);
    for _ in 0..count {
        inject.inject(unit, spike_message(one, false)).unwrap();
    }
}

/// Ticks until nothing is in flight and every unit's mailbox is empty, or `limit` ticks.
fn settle(exec: &mut Executor<64>, limit: u64) {
    for _ in 0..limit {
        if exec.is_quiescent() {
            return;
        }
        exec.tick();
    }
    assert!(
        exec.is_quiescent(),
        "the network settled within {limit} ticks"
    );
}

#[test]
fn an_image_round_trips_byte_for_byte_and_the_loaded_network_runs_identically() {
    let path = scratch("roundtrip.cortex");
    let mut original = Executor::<64>::new(config()).unwrap();
    wire(&mut original);
    Image::write(&original, &path).expect("written");
    let first = std::fs::read(&path).unwrap();
    assert_eq!(&first[0..8], b"VCORTEX1");
    assert_eq!(first.len() % 64, 0, "sections are 64-byte aligned");

    let mut loaded: Executor<64> = Image::open(&path, config()).expect("opened");
    assert_eq!(loaded.units().len(), 96);
    assert_eq!(loaded.blocks(), original.blocks());
    assert_eq!(loaded.deltas(), original.deltas());
    for (a, b) in loaded.units().iter().zip(original.units()) {
        assert!(a.same_bytes(b));
    }
    assert_eq!(
        Image::encode(&loaded).unwrap(),
        first,
        "a second write is byte-identical"
    );

    // Both run the same trace and stay identical, including through the sorted batches,
    // STDP and the stored releases.
    for exec in [&mut original, &mut loaded] {
        for unit in [3u32, 17, 40, 88] {
            kick(exec, unit, 14);
        }
        exec.run(4000);
    }
    assert_eq!(loaded.blocks(), original.blocks());
    for (a, b) in loaded.units().iter().zip(original.units()) {
        assert!(a.same_bytes(b));
    }
    let _ = std::fs::remove_file(&path);
}

#[test]
fn a_corrupted_truncated_or_foreign_image_fails_closed() {
    let path = scratch("corrupt.cortex");
    let mut exec = Executor::<64>::new(config()).unwrap();
    wire(&mut exec);
    Image::write(&exec, &path).unwrap();
    let bytes = std::fs::read(&path).unwrap();

    let mut flipped = bytes.clone();
    flipped[64 * 4 + 30] ^= 0x01; // inside the neuron section
    assert!(matches!(
        Image::decode::<64>(&flipped, config()),
        Err(ImageError::SectionCrc(SECTION_NEURON))
    ));

    let truncated = &bytes[..bytes.len() - 64];
    assert!(matches!(
        Image::decode::<64>(truncated, config()),
        Err(ImageError::Truncated)
    ));
    assert!(matches!(
        Image::decode::<64>(&bytes[..40], config()),
        Err(ImageError::Truncated)
    ));

    let mut foreign = bytes.clone();
    let mut header = CortexFileHeader::decode(foreign[0..64].try_into().unwrap());
    header.version = CortexFileHeader::FORMAT_VERSION + 1;
    header.crc64 = header.checksum();
    foreign[0..64].copy_from_slice(&header.encode());
    assert!(matches!(
        Image::decode::<64>(&foreign, config()),
        Err(ImageError::Header(HeaderError::ForeignVersion(_)))
    ));

    let mut bad_header = bytes.clone();
    bad_header[20] ^= 0xFF;
    assert!(matches!(
        Image::decode::<64>(&bad_header, config()),
        Err(ImageError::Header(HeaderError::BadCrc))
    ));

    let mut wrong_magic = bytes.clone();
    wrong_magic[7] = b'9';
    assert!(matches!(
        Image::decode::<64>(&wrong_magic, config()),
        Err(ImageError::Header(HeaderError::BadMagic))
    ));

    assert!(
        Image::decode::<64>(&bytes, config()).is_ok(),
        "the untouched image opens"
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn a_delay_beyond_the_horizon_and_a_dangling_target_are_refused_at_load() {
    let path = scratch("horizon.cortex");
    let mut exec = Executor::<64>::new(config()).unwrap();
    wire(&mut exec);
    let good = Image::encode(&exec).unwrap();
    assert!(exec.blocks_mut()[5].set_synapse(2, 1, 100, 2560, false));
    assert!(matches!(
        Image::decode::<64>(&Image::encode(&exec).unwrap(), config()),
        Err(ImageError::DelayBeyondHorizon { block: 5, slot: 2 })
    ));
    assert!(exec.blocks_mut()[5].set_synapse(2, 96, 100, 10, false));
    assert!(matches!(
        Image::decode::<64>(&Image::encode(&exec).unwrap(), config()),
        Err(ImageError::DanglingIndex { block: 5, slot: 2 })
    ));
    assert!(exec.blocks_mut()[5].set_synapse(2, 1, 100, 10, false));
    assert!(exec.blocks_mut()[5].link(96));
    assert!(matches!(
        Image::decode::<64>(&Image::encode(&exec).unwrap(), config()),
        Err(ImageError::DanglingIndex { block: 5, slot: 4 })
    ));
    assert!(Image::decode::<64>(&good, config()).is_ok());
    let _ = std::fs::remove_file(&path);
}

/// A small image: two units, one synapse, one delta.
fn small_image() -> Vec<u8> {
    let mut exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        deltas: 1,
        ..Config::default()
    })
    .unwrap();
    assert!(exec.blocks_mut()[0].set_synapse(0, 1, 100, 1, false));
    assert!(exec.units_mut()[0].set_first_block(0));
    exec.deltas_mut()[0] = PlasticDelta::new(0, 0, 5, 1).unwrap();
    assert!(exec.units_mut()[0].set_delta_head(0));
    Image::encode(&exec).unwrap()
}

/// Edits section `kind` in place and re-seals its checksum, so only the record check can
/// refuse the image.
fn patch_section(img: &mut [u8], kind: u32, patch: impl Fn(&mut [u8])) {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for i in 0..header.section_count as usize {
        let at = 64 + 64 * i;
        let mut entry = SectionEntry::decode(img[at..at + 64].try_into().unwrap());
        if entry.kind == kind {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            patch(&mut img[offset..offset + length]);
            entry.crc64 = crc64(&img[offset..offset + length]);
            img[at..at + 64].copy_from_slice(&entry.encode());
            return;
        }
    }
    panic!("no section {kind}");
}

#[test]
fn a_record_that_is_not_at_rest_in_its_reserved_bytes_or_its_slot_is_refused_at_load() {
    assert!(Image::decode::<8>(&small_image(), Config::default()).is_ok());
    let mut img = small_image();
    patch_section(&mut img, SECTION_PLASTIC_DELTA, |s| s[4] = 9);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::DanglingIndex { block: 0, slot: 5 })
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_PLASTIC_DELTA, |s| s[5] = 0xFF);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ReservedNotZero {
            section: SECTION_PLASTIC_DELTA,
            index: 0
        })
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_NEURON, |s| {
        s[52] = 1;
        s[53] = 2;
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::NotAtRest(0))
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_SYNAPSE, |s| s[63] = 7);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ReservedNotZero {
            section: SECTION_SYNAPSE,
            index: 0
        })
    ));
}

/// Re-seals the header after `patch` edited its decoded fields.
fn patch_header(img: &mut [u8], patch: impl Fn(&mut CortexFileHeader)) {
    let mut header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    patch(&mut header);
    let sealed = CortexFileHeader::new(
        header.num_columns,
        header.num_neurons,
        header.num_synapses,
        header.section_count,
    );
    img[0..64].copy_from_slice(&sealed.encode());
}

/// Edits directory entry `kind` in place (the directory is not sealed).
fn patch_entry(img: &mut [u8], kind: u32, patch: impl Fn(&mut SectionEntry)) {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for i in 0..header.section_count as usize {
        let at = 64 + 64 * i;
        let mut entry = SectionEntry::decode(img[at..at + 64].try_into().unwrap());
        if entry.kind == kind {
            patch(&mut entry);
            img[at..at + 64].copy_from_slice(&entry.encode());
            return;
        }
    }
    panic!("no section {kind}");
}

#[test]
fn every_clause_of_the_loader_s_checks_refuses_on_its_own() {
    // The directory: a well-formed entry with the wrong record size, and a malformed one with
    // the right size.
    let mut img = small_image();
    patch_entry(&mut img, SECTION_NEURON, |e| e.record_size = 16);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_NEURON))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_SYNAPSE, |e| e.kind = 99);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(99))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_SYNAPSE, |e| e.kind = SECTION_NEURON);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::MissingSection(SECTION_SYNAPSE))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_NEURON, |e| e._reserved[0] = 1);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_NEURON))
    ));
    // The counts: each of the header's two record counts against its section.
    let mut img = small_image();
    patch_header(&mut img, |h| h.num_neurons += 1);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_NEURON))
    ));
    let mut img = small_image();
    patch_header(&mut img, |h| h.num_synapses += 1);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_NEURON))
    ));
    // A delta: its block index, then its next index, each outside the arena on its own.
    let mut img = small_image();
    patch_section(&mut img, SECTION_PLASTIC_DELTA, |s| {
        s[0..4].copy_from_slice(&5u32.to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::DanglingIndex { block: 0, slot: 5 })
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_PLASTIC_DELTA, |s| {
        s[12..16].copy_from_slice(&5u32.to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::DanglingIndex { block: 0, slot: 5 })
    ));
    // A unit: its first block, then its delta head, each outside its arena on its own.
    let mut img = small_image();
    patch_section(&mut img, SECTION_NEURON, |s| {
        s[48..52].copy_from_slice(&9u32.to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::DanglingIndex { block: 0, slot: 6 })
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_NEURON, |s| {
        s[60..64].copy_from_slice(&9u32.to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::DanglingIndex { block: 0, slot: 6 })
    ));
}

#[test]
fn a_unit_the_writer_left_awake_is_woken_by_the_loader_and_fires() {
    let mut img = small_image();
    // Unit 1 above threshold with a threshold set: image-ready (idle, empty mailbox) but not
    // at rest, so the loader wakes it and the first tick integrates it.
    patch_section(&mut img, SECTION_NEURON, |s| {
        s[64 + 24..64 + 28].copy_from_slice(&(2 * THRESHOLD_BASE).to_le_bytes());
        s[64 + 36..64 + 40].copy_from_slice(&THRESHOLD_BASE.to_le_bytes());
    });
    let mut exec = Image::decode::<8>(
        &img,
        Config {
            trace_capacity: 64,
            ..Config::default()
        },
    )
    .unwrap();
    exec.run(2);
    let reports = exec.shutdown();
    let spikes: Vec<(u32, u32)> = reports
        .iter()
        .flat_map(|r| r.spikes.iter().copied())
        .collect();
    assert_eq!(
        spikes,
        vec![(1, 0)],
        "unit 1 fired on the first tick after the load"
    );
}

#[test]
fn the_sweep_evicts_at_exactly_the_quiet_bound_and_leaves_a_unit_with_a_message() {
    let log_path = scratch("bound.wal");
    let mut exec = Executor::<64>::new(config()).unwrap();
    wire(&mut exec);
    exec.attach_log(&log_path).expect("a log");
    assert_eq!((exec.evictions(), exec.rehydrations()), (0, 0));
    exec.run(100);
    assert_eq!(
        exec.sweep(101, 96).unwrap(),
        0,
        "quiet for 100 ticks is not quiet for 101"
    );
    assert_eq!(
        exec.sweep(100, 96).unwrap(),
        96,
        "quiet for exactly the bound is swept"
    );
    assert_eq!(exec.evictions(), 96);
    let mut busy = Executor::<64>::new(config()).unwrap();
    wire(&mut busy);
    busy.attach_log(&scratch("busy.wal")).expect("a log");
    busy.run(100);
    kick(&busy, 7, 1);
    busy.tick();
    assert!(
        !busy.units()[7].mailbox_is_empty(),
        "the message waits in the mailbox"
    );
    let swept = busy.sweep(0, 96).unwrap();
    assert!(
        !busy.is_evicted(7),
        "a unit holding a message is not at rest"
    );
    assert_eq!(swept, 95);
    assert_eq!(busy.log().unwrap().entries(), 95);
}

#[test]
fn a_sealed_header_claiming_more_directory_than_the_file_holds_is_truncated_not_an_allocation() {
    let header = CortexFileHeader::new(0, 1, 0, u32::MAX);
    assert_eq!(header.validate(), Ok(()));
    assert!(matches!(
        Image::decode::<8>(&header.encode(), Config::default()),
        Err(ImageError::Truncated)
    ));
    let mut two = header.encode().to_vec();
    two.extend_from_slice(&[0; 64]);
    assert!(matches!(
        Image::decode::<8>(&two, Config::default()),
        Err(ImageError::Truncated)
    ));
}

#[test]
fn the_log_refuses_a_unit_outside_it_before_writing() {
    let path = scratch("outside.wal");
    let mut log = WriteAheadLog::create(&path, 2).unwrap();
    assert!(!log.holds(5));
    let err = log.append(5, &[0; 64]).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        std::fs::metadata(&path).unwrap().len(),
        0,
        "nothing was written"
    );
    assert!(matches!(log.read(5), Err(ImageError::LogCorrupt(5))));
    assert!(log.append(1, &[7; 64]).is_ok());
    assert_eq!(log.read(1).unwrap(), [7; 64]);
    assert_eq!(log.entries(), 1);
    assert!(log.append(0, &[8; 64]).is_ok());
    assert_eq!(log.entries(), 2, "one entry per append");
    assert_eq!(log.read(0).unwrap(), [8; 64]);
}

#[test]
fn a_pair_still_in_the_injector_ring_is_not_a_quiescent_point() {
    let mut exec = Executor::<8>::new(Config {
        units: 2,
        ..Config::default()
    })
    .unwrap();
    assert!(exec.is_quiescent());
    exec.injector().inject(1, 0x100).unwrap();
    assert!(!exec.is_quiescent(), "the pair is in no record yet");
    assert!(matches!(
        Image::encode(&exec),
        Err(ImageError::NotQuiescent)
    ));
    exec.run(2);
    assert!(exec.is_quiescent(), "drained, delivered and integrated");
    assert!(Image::encode(&exec).is_ok());
}

#[test]
fn an_image_is_written_only_at_a_quiescent_point() {
    let mut exec = Executor::<64>::new(config()).unwrap();
    wire(&mut exec);
    kick(&exec, 3, 14);
    exec.tick(); // the kick is now in unit 3's mailbox
    assert!(!exec.is_quiescent());
    assert!(matches!(
        Image::encode(&exec),
        Err(ImageError::NotQuiescent)
    ));
    settle(&mut exec, 20_000);
    assert!(Image::encode(&exec).is_ok());
}

/// Milestone M4's exit test: evict, spike, re-hydrate preserves state bit for bit.
#[test]
fn evict_spike_rehydrate_preserves_every_unit_bit_for_bit() {
    let log_path = scratch("sweep.wal");
    let mut swept = Executor::<64>::new(config()).unwrap();
    let mut control = Executor::<64>::new(config()).unwrap();
    wire(&mut swept);
    wire(&mut control);
    assert!(matches!(swept.sweep(0, 10), Err(ImageError::NoLog)));
    swept.attach_log(&log_path).expect("a log");

    let mut x = 0x9E37_79B9u32;
    let mut next = || {
        x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        x
    };
    let mut rounds_with_evicted_target = 0;
    for round in 0..60u32 {
        // A kick into a random unit, which on the swept executor is sometimes evicted.
        let unit = (next() >> 8) % 96;
        if swept.is_evicted(unit) {
            rounds_with_evicted_target += 1;
        }
        kick(&swept, unit, 14);
        kick(&control, unit, 14);
        swept.run(100);
        control.run(100);
        // Every few rounds the sweep evicts whatever has been quiet for 150 ticks.
        if round % 3 == 2 {
            swept.sweep(150, 96).expect("a sweep");
        }
        // Evicted units hold only their id until a message re-hydrates them.
        for (i, unit) in swept.units().iter().enumerate() {
            if swept.is_evicted(i as u32) {
                assert_eq!(unit.v_thresh, 0, "an evicted slot is empty");
                assert_eq!(unit.id, i as u64);
            }
        }
    }
    assert!(swept.evictions() > 0, "the sweep evicted");
    assert!(
        swept.rehydrations() > 0 && rounds_with_evicted_target > 0,
        "a spike reached an evicted unit and re-hydrated it"
    );
    settle(&mut swept, 40_000);
    settle(&mut control, 40_000);
    // Evicted units are folded back from the log; the two images are byte-identical.
    let swept_image = Image::encode(&swept).expect("the swept image");
    let control_image = Image::encode(&control).expect("the control image");
    assert_eq!(swept_image, control_image);
    // Live units too.
    for (i, (a, b)) in swept.units().iter().zip(control.units()).enumerate() {
        if !swept.is_evicted(i as u32) {
            assert!(a.same_bytes(b), "unit {i}");
        }
    }
    let _ = std::fs::remove_file(&log_path);
}
