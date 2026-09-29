//! Milestone M4's exit test and the image round trip (whitepaper Appendix C; brief 015;
//! ADR-0024): an image written from arenas and opened again is byte-identical and runs
//! identically; a corrupted, truncated or foreign image fails closed; a synapse delay the wheel
//! cannot hold is refused at load; and evict, spike, re-hydrate preserves a unit bit for bit:
//! a run that sweeps and re-hydrates ends in the same image as one that never evicts.

#![deny(clippy::arithmetic_side_effects)]

use cortex_connectome::{
    CortexFileHeader, HeaderError, SECTION_EPISODE, SECTION_HIPPOCAMPUS, SECTION_HOMEOSTASIS,
    SECTION_MODULATOR, SECTION_NEURON, SECTION_PLASTIC_DELTA, SECTION_SYNAPSE, SectionEntry, crc64,
};
use cortex_core::{
    FLAG_FACILITATING, FLAG_INHIBITORY, FLAG_SLOW, ISTDP_PERIOD_MAX_TICKS, ISTDP_PERIOD_MIN_TICKS,
    ISTDP_TARGET_PERIOD_TICKS, MODULATION_ONE_Q16, PlasticDelta, STP_MAX, STP_U, SlowCurrent,
    StpClass, THRESHOLD_BASE, TICK_NS, spike_message, synaptic_efficacy_q16,
};
use cortex_hippocampus::{Episode, HippocampalAttractorState, PATTERN_MAX};
use cortex_homeostasis::{
    ACTIVITY_WINDOW_BINS, CONTROL_STEP_MAX_Q0_16, GAIN_MAX_Q16, GAIN_MIN_Q16, HomeostaticDrivePool,
    PRESSURE_MAX_Q16, REM_WINDOWS, SLEEP_SHIFT_MAX, STAGE_REM, STAGE_SWS, SWS_WINDOWS,
};
use cortex_neuromod::ValueCritic;
use cortex_runtime::{Config, ConfigError, Executor, Image, ImageError, WriteAheadLog};
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
                let target = (next() >> 8)
                    .checked_rem(units as u32)
                    .expect("the arena holds a unit");
                let delay = match next() % 6 {
                    0 => 0,
                    _ => ((next() >> 8) % 300).wrapping_add(1),
                } as u16;
                let weight = (((next() >> 8) % 20_000) as i16).wrapping_add(10_000);
                assert!(block.set_synapse(slot, target, weight, delay, next() % 4 == 0));
            }
        }
    }
    {
        let deltas = exec.deltas_mut();
        for (i, d) in deltas.iter_mut().enumerate() {
            *d = PlasticDelta::new(i as u32, (i % 4) as u8, (i as i16).wrapping_mul(-100), 7)
                .unwrap();
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
    // The header, eight directory entries (neurons, synapses, deltas, the modulation state,
    // the homeostasis state, the hippocampal state, the affect state, the induction
    // record), then the neuron section.
    flipped[64 * 9 + 30] ^= 0x01;
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

/// `small_image` with the dopamine signal off rest, so the modulator section carries a
/// signal to read back.
fn small_image_with_modulator() -> Vec<u8> {
    let mut exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        ..Config::default()
    })
    .unwrap();
    assert_eq!(exec.reward(0x1234), 0x1234);
    Image::encode(&exec).unwrap()
}

/// Edits section `kind` in place and re-seals its checksum, so only the record check can
/// refuse the image.
fn patch_section(img: &mut [u8], kind: u32, patch: impl Fn(&mut [u8])) {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    // The entries start at 64 and are 64 bytes each: an iterator, not a counter.
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == kind {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            patch(&mut img[offset..][..length]);
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
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
    // `[52..54)` was reserved until ADR-0131 made it the value weight, which only the critic
    // writes: refused while the image carries none.
    patch_section(&mut img, SECTION_NEURON, |s| {
        s[52] = 1;
        s[53] = 2;
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ValueWithoutCritic(0))
    ));
    let mut img = small_image_with_modulator();
    patch_section(&mut img, SECTION_MODULATOR, |s| s[63] = 7);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ReservedNotZero {
            section: SECTION_MODULATOR,
            index: 0
        })
    ));
    let mut img = small_image_with_modulator();
    patch_section(&mut img, SECTION_MODULATOR, |s| s[26] = 1);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ReservedNotZero {
            section: SECTION_MODULATOR,
            index: 0
        })
    ));
    // The inhibitory rule's target period at [20..24) (ADR-0053): outside its bounds, the
    // configuration's refusal; within them, the image's outranks the configuration's.
    let mut img = small_image_with_modulator();
    patch_section(&mut img, SECTION_MODULATOR, |s| {
        s[20..24].copy_from_slice(&(ISTDP_PERIOD_MIN_TICKS - 1).to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Config(ConfigError::IstdpPeriodOutOfRange))
    ));
    let mut img = small_image_with_modulator();
    patch_section(&mut img, SECTION_MODULATOR, |s| {
        s[20..24].copy_from_slice(&(ISTDP_PERIOD_MAX_TICKS + 1).to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Config(ConfigError::IstdpPeriodOutOfRange))
    ));
    let mut img = small_image_with_modulator();
    patch_section(&mut img, SECTION_MODULATOR, |s| {
        s[20..24].copy_from_slice(&5_000u32.to_le_bytes())
    });
    assert_eq!(
        Image::decode::<8>(
            &img,
            Config {
                istdp_target_period_ticks: 40_000,
                ..Config::default()
            }
        )
        .unwrap()
        .istdp_target_period_ticks(),
        5_000,
        "the image's period is the engine's"
    );
    assert_eq!(
        Image::decode::<8>(&small_image(), Config::default())
            .unwrap()
            .istdp_target_period_ticks(),
        ISTDP_TARGET_PERIOD_TICKS,
        "written from the configuration's default"
    );
    let img = small_image_with_modulator();
    let loaded = Image::decode::<8>(&img, Config::default()).unwrap();
    assert_eq!(
        loaded.modulator().dopamine_rpe,
        0x1234,
        "the record's own bytes are read"
    );
    // A trace or a compartment on an empty slot: bytes the writer never produces.
    let mut img = small_image();
    patch_section(&mut img, SECTION_SYNAPSE, |s| s[58] = 7);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ReservedNotZero {
            section: SECTION_SYNAPSE,
            index: 0
        })
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_SYNAPSE, |s| s[35] |= 0x20);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::ReservedNotZero {
            section: SECTION_SYNAPSE,
            index: 0
        })
    ));
    // The same bytes on the filled slot are state, and load.
    let mut img = small_image();
    patch_section(&mut img, SECTION_SYNAPSE, |s| {
        s[56] = 7;
        s[35] |= 0x10;
    });
    let loaded = Image::decode::<8>(&img, Config::default()).unwrap();
    assert_eq!(loaded.blocks()[0].eligibility_q1_15[0], 7);
    assert!(loaded.blocks()[0].is_apical(0));
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
        header.tick_ns,
        header.written_tick,
    );
    img[0..64].copy_from_slice(&sealed.encode());
}

/// Edits directory entry `kind` in place (the directory is not sealed).
fn patch_entry(img: &mut [u8], kind: u32, patch: impl Fn(&mut SectionEntry)) {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    // The entries start at 64 and are 64 bytes each: an iterator, not a counter.
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == kind {
            patch(&mut entry);
            img[at..][..64].copy_from_slice(&entry.encode());
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
    // The tick (ADR-0033): a header sealed with another duration, and one with none.
    let mut img = small_image();
    patch_header(&mut img, |h| h.tick_ns = 20_000);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::TickMismatch(20_000))
    ));
    let mut img = small_image();
    patch_header(&mut img, |h| h.tick_ns = 0);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Header(HeaderError::ZeroTick))
    ));
    assert_eq!(TICK_NS, 10_000, "the fine tick of §8.4");
    // The modulator section (ADR-0032): exactly one record.
    let mut img = small_image_with_modulator();
    patch_entry(&mut img, SECTION_MODULATOR, |e| {
        e.length = 0;
        e.crc64 = 0;
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_MODULATOR))
    ));
    let mut img = small_image_with_modulator();
    patch_entry(&mut img, SECTION_MODULATOR, |e| e.record_size = 16);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_MODULATOR))
    ));
    // The modulation state is required: an image without it does not define its run.
    let mut img = small_image();
    patch_entry(&mut img, SECTION_MODULATOR, |e| e.kind = SECTION_NEURON);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::MissingSection(SECTION_MODULATOR))
    ));
    // A baseline outside [0, 1.0] is refused as the configuration's would be.
    let mut img = small_image();
    patch_section(&mut img, SECTION_MODULATOR, |s| {
        s[16..20].copy_from_slice(&0x0001_0001i32.to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Config(ConfigError::ModulationOutOfRange))
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_MODULATOR, |s| {
        s[16..20].copy_from_slice(&(-1i32).to_le_bytes())
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Config(ConfigError::ModulationOutOfRange))
    ));
    let mut img = small_image();
    patch_section(&mut img, SECTION_MODULATOR, |s| {
        s[16..20].copy_from_slice(&0x8000i32.to_le_bytes())
    });
    assert_eq!(
        Image::decode::<8>(&img, Config::default())
            .unwrap()
            .modulation_baseline_q16(),
        0x8000,
        "the image's baseline is the engine's"
    );
    // The homeostasis section (ADR-0036): exactly one 64-byte record, required.
    let mut img = small_image();
    patch_entry(&mut img, SECTION_HOMEOSTASIS, |e| {
        e.length = 0;
        e.crc64 = 0;
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_HOMEOSTASIS))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_HOMEOSTASIS, |e| e.record_size = 16);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_HOMEOSTASIS))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_HOMEOSTASIS, |e| e.kind = SECTION_NEURON);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::MissingSection(SECTION_HOMEOSTASIS))
    ));
    // Its record: the step above its bound is the configuration's refusal; a gain outside its
    // bounds, a reserved byte, a full window and a window that is not the clock's are records
    // the rules never leave.
    let with_pool = |patch: fn(&mut HomeostaticDrivePool)| {
        let mut img = small_image();
        patch_section(&mut img, SECTION_HOMEOSTASIS, |s| {
            let mut pool = HomeostaticDrivePool::decode((&s[..64]).try_into().unwrap());
            patch(&mut pool);
            s.copy_from_slice(&pool.encode());
        });
        img
    };
    assert!(matches!(
        Image::decode::<8>(
            &with_pool(|p| p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16 + 1),
            Config::default()
        ),
        Err(ImageError::Config(ConfigError::ControlStepOutOfRange))
    ));
    assert!(matches!(
        Image::decode::<8>(
            &with_pool(|p| p.sleep_shift = SLEEP_SHIFT_MAX + 1),
            Config::default()
        ),
        Err(ImageError::Config(ConfigError::SleepShiftOutOfRange))
    ));
    for patch in [
        (|p| p.synaptic_gain_q16 = GAIN_MAX_Q16 + 1) as fn(&mut HomeostaticDrivePool),
        |p| p.synaptic_gain_q16 = GAIN_MIN_Q16 - 1,
        |p| p.window_bins = ACTIVITY_WINDOW_BINS,
        |p| p.window_bins = 1,
        |p| p.bin_activity = 0x0100_0000,
        |p| p.sleep_stage = STAGE_REM + 1,
        |p| p.sleep_pressure_q16 = PRESSURE_MAX_Q16 + 1,
        |p| {
            p.sleep_stage = STAGE_SWS;
            p.stage_windows = SWS_WINDOWS;
        },
        |p| {
            p.sleep_stage = STAGE_REM;
            p.stage_windows = REM_WINDOWS;
        },
    ] {
        assert!(matches!(
            Image::decode::<8>(&with_pool(patch), Config::default()),
            Err(ImageError::MalformedHomeostasis)
        ));
    }
    // The image's gain, step, shift, stage and pressure outrank the configuration's.
    let loaded = Image::decode::<8>(
        &with_pool(|p| {
            p.synaptic_gain_q16 = 0x8000;
            p.control_step_q0_16 = CONTROL_STEP_MAX_Q0_16;
            p.bin_activity = 9;
            p.sleep_shift = SLEEP_SHIFT_MAX;
            p.sleep_stage = STAGE_REM;
            p.stage_windows = REM_WINDOWS - 1;
            p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        }),
        Config::default(),
    )
    .unwrap();
    assert_eq!(
        (
            loaded.homeostasis().synaptic_gain_q16,
            loaded.homeostasis().control_step_q0_16,
            loaded.homeostasis().bin_activity,
            loaded.homeostasis().sleep_shift,
            loaded.sleep_stage(),
            loaded.homeostasis().stage_windows,
            loaded.homeostasis().sleep_pressure_q16,
        ),
        (
            0x8000,
            CONTROL_STEP_MAX_Q0_16,
            9,
            SLEEP_SHIFT_MAX,
            STAGE_REM,
            REM_WINDOWS - 1,
            PRESSURE_MAX_Q16
        ),
        "the image's homeostasis state is the engine's"
    );
    // The hippocampal section (ADR-0038): exactly one 64-byte record, required; the ledger
    // section required when the record says the ledger is not empty, its count the record's
    // length; a hand at the length or a reserved byte refused; an episode refused on each
    // clause of its own well-formedness and for a unit outside the arena.
    let mut img = small_image();
    patch_entry(&mut img, SECTION_HIPPOCAMPUS, |e| {
        e.length = 0;
        e.crc64 = 0;
    });
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_HIPPOCAMPUS))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_HIPPOCAMPUS, |e| e.record_size = 16);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_HIPPOCAMPUS))
    ));
    let mut img = small_image();
    patch_entry(&mut img, SECTION_HIPPOCAMPUS, |e| e.kind = SECTION_NEURON);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::MissingSection(SECTION_HIPPOCAMPUS))
    ));
    let with_state = |patch: fn(&mut HippocampalAttractorState)| {
        let mut img = small_image();
        patch_section(&mut img, SECTION_HIPPOCAMPUS, |s| {
            let mut state = HippocampalAttractorState::decode((&s[..64]).try_into().unwrap());
            patch(&mut state);
            s.copy_from_slice(&state.encode());
        });
        img
    };
    assert!(
        matches!(
            Image::decode::<8>(&with_state(|h| h.episodes = 1), Config::default()),
            Err(ImageError::MissingSection(SECTION_EPISODE))
        ),
        "a length without a ledger section"
    );
    for patch in [
        (|h| h.replay_hand = 1) as fn(&mut HippocampalAttractorState),
        |h| h._reserved[35] = 1,
    ] {
        assert!(matches!(
            Image::decode::<8>(&with_state(patch), Config::default()),
            Err(ImageError::MalformedHippocampus)
        ));
    }
    let specified = Image::decode::<8>(
        &with_state(|h| {
            h.dg_sparsity_bits = 5;
            h.place_field_id = 6;
        }),
        Config::default(),
    )
    .unwrap();
    assert_eq!(
        (
            specified.hippocampus().dg_sparsity_bits,
            specified.hippocampus().place_field_id
        ),
        (5, 6),
        "the Specified fields are read back"
    );
    // A ledger of two episodes, then each refusal on its own.
    let ledger = || {
        let mut exec = Executor::<8>::new(Config {
            units: 4,
            blocks: 1,
            episodes: 2,
            ..Config::default()
        })
        .unwrap();
        assert_eq!(exec.tag_episode(&[0, 1, 2], 3), Ok(0));
        assert_eq!(exec.tag_episode(&[3], 1), Ok(1));
        assert_eq!(exec.hippocampus().episodes, 2);
        Image::encode(&exec).unwrap()
    };
    let loaded = Image::decode::<8>(&ledger(), Config::default()).unwrap();
    assert_eq!(loaded.episodes().len(), 2);
    assert_eq!(loaded.episodes()[0].pattern(), &[0, 1, 2]);
    assert_eq!(
        (loaded.episodes()[1].tag, loaded.episodes()[1].pattern()),
        (1, &[3][..])
    );
    assert_eq!(
        loaded.episode_room(),
        0,
        "sized as the image's ledger plus the configuration's room"
    );
    assert_eq!(
        Image::decode::<8>(
            &ledger(),
            Config {
                episodes: 3,
                ..Config::default()
            }
        )
        .unwrap()
        .episode_room(),
        3
    );
    let mut img = ledger();
    patch_entry(&mut img, SECTION_EPISODE, |e| e.record_size = 16);
    assert!(matches!(
        Image::decode::<8>(&img, Config::default()),
        Err(ImageError::Directory(SECTION_EPISODE))
    ));
    let mut img = ledger();
    patch_section(&mut img, SECTION_HIPPOCAMPUS, |s| {
        s[12..16].copy_from_slice(&1u32.to_le_bytes())
    });
    assert!(
        matches!(
            Image::decode::<8>(&img, Config::default()),
            Err(ImageError::MalformedHippocampus)
        ),
        "a length that is not the section's count"
    );
    let mut img = ledger();
    patch_section(&mut img, SECTION_HIPPOCAMPUS, |s| {
        s[24..28].copy_from_slice(&2u32.to_le_bytes())
    });
    assert!(
        matches!(
            Image::decode::<8>(&img, Config::default()),
            Err(ImageError::MalformedHippocampus)
        ),
        "a hand at the length"
    );
    let with_episode = |index: usize, patch: fn(&mut Episode)| {
        let mut img = ledger();
        patch_section(&mut img, SECTION_EPISODE, |s| {
            let at = index * 64;
            let mut e = Episode::decode((&s[at..at + 64]).try_into().unwrap());
            patch(&mut e);
            s[at..at + 64].copy_from_slice(&e.encode());
        });
        img
    };
    for (index, patch) in [
        (0usize, (|e| e.len = 0) as fn(&mut Episode)),
        (1, |e| e.len = PATTERN_MAX as u8 + 1),
        (0, |e| e.pattern[3] = 3),
        (0, |e| e.pattern[2] = 0),
        (1, |e| e._pad = 1),
        (1, |e| e._reserved[0] = 1),
        (1, |e| e.pattern[0] = 4),
        (0, |e| e.pattern[1] = u32::MAX),
    ] {
        let err = Image::decode::<8>(&with_episode(index, patch), Config::default()).err();
        assert!(
            matches!(err, Some(ImageError::MalformedEpisode(i)) if i as usize == index),
            "episode {index}: {err:?}"
        );
    }
    let spent = Image::decode::<8>(&with_episode(1, |e| e.tag = 0), Config::default()).unwrap();
    assert!(
        spent.episodes()[1].is_spent(),
        "a spent episode is a record of the ledger"
    );
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
    let header = CortexFileHeader::new(0, 1, 0, u32::MAX, TICK_NS, 0);
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
    // A header with no sections and nothing after it is not truncated: it lacks its sections.
    let none = CortexFileHeader::new(0, 1, 0, 0, TICK_NS, 0);
    assert!(matches!(
        Image::decode::<8>(&none.encode(), Config::default()),
        Err(ImageError::MissingSection(SECTION_NEURON))
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
    assert!(!log.holds(1) && !log.holds(0), "nothing logged yet");
    assert!(log.append(1, &[7; 64]).is_ok());
    assert!(log.holds(1), "a logged unit is held");
    assert!(!log.holds(0), "and only that one");
    assert_eq!(log.read(1).unwrap(), [7; 64]);
    assert_eq!(log.entries(), 1);
    assert!(log.append(0, &[8; 64]).is_ok());
    assert!(log.holds(0));
    assert_eq!(log.entries(), 2, "one entry per append");
    assert_eq!(log.read(0).unwrap(), [8; 64]);
}

/// The positioned read fills its buffer from repeated reads and reports a file that ends
/// first as `UnexpectedEof`, on every platform (ADR-0062): the entry is cut in half behind
/// the log's back, and the read of it fails closed instead of returning a short record.
#[test]
fn a_log_entry_the_file_no_longer_holds_in_full_is_an_early_end_not_a_short_record() {
    let path = scratch("short.wal");
    let mut log = WriteAheadLog::create(&path, 2).unwrap();
    assert!(log.append(1, &[9; 64]).is_ok());
    assert_eq!(log.read(1).unwrap(), [9; 64]);
    std::fs::OpenOptions::new()
        .write(true)
        .open(&path)
        .unwrap()
        .set_len(40)
        .unwrap();
    assert!(
        matches!(
            log.read(1),
            Err(ImageError::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof
        ),
        "a half entry is an early end"
    );
    assert!(log.holds(1), "the log's own index is unchanged");
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

/// The bytes of section `kind`, copied out.
fn section_bytes(img: &[u8], kind: u32) -> Vec<u8> {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == kind {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            return img[offset..][..length].to_vec();
        }
    }
    panic!("no section {kind}");
}

/// The inhibitory baseline in the modulator section (ADR-0086; format 15): a flag byte at
/// `[24]` and the value at `[28..32)`. Unset, both are zero — the bytes a format-14 writer
/// left there — and read as unset whatever the configuration says; set, the flag is 1 and
/// the value is read back, whatever the configuration says, zero and 1.0 included. Refused:
/// a set value outside [0, 1], as the baseline's is; a flag of zero with a value that is
/// not, a flag that is neither zero nor one, and a reserved byte between or after them,
/// which the writer never produces; and a header stamped with the previous version, as
/// every foreign version is.
#[test]
fn the_inhibitory_baseline_is_written_to_and_read_from_the_image_and_a_record_left_zero_reads_as_unset()
 {
    let unset = small_image_with_modulator();
    let section = section_bytes(&unset, SECTION_MODULATOR);
    assert_eq!(&section[24..64], &[0u8; 40], "unset writes zeros");
    let loaded = Image::decode::<8>(
        &unset,
        Config {
            inhibitory_baseline_q16: Some(0x4000),
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(
        loaded.inhibitory_baseline_q16(),
        None,
        "the image's unset outranks the configuration's"
    );
    let exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        inhibitory_baseline_q16: Some(0x8000),
        ..Config::default()
    })
    .unwrap();
    let set = Image::encode(&exec).unwrap();
    let section = section_bytes(&set, SECTION_MODULATOR);
    assert_eq!(section[24], 1, "the flag");
    assert_eq!(&section[25..28], &[0u8; 3]);
    assert_eq!(&section[28..32], &0x8000i32.to_le_bytes(), "the value");
    assert_eq!(&section[32..64], &[0u8; 32]);
    assert_eq!(
        Image::decode::<8>(&set, Config::default())
            .unwrap()
            .inhibitory_baseline_q16(),
        Some(0x8000),
        "the image's set outranks the configuration's unset"
    );
    let with_value = |flag: u8, value: i32| {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| {
            s[24] = flag;
            s[28..32].copy_from_slice(&value.to_le_bytes());
        });
        img
    };
    for value in [0, 1, MODULATION_ONE_Q16 - 1, MODULATION_ONE_Q16] {
        assert_eq!(
            Image::decode::<8>(&with_value(1, value), Config::default())
                .unwrap()
                .inhibitory_baseline_q16(),
            Some(value),
            "set at {value} is set"
        );
    }
    for value in [-1, MODULATION_ONE_Q16 + 1, i32::MIN, i32::MAX] {
        assert!(
            matches!(
                Image::decode::<8>(&with_value(1, value), Config::default()),
                Err(ImageError::Config(
                    ConfigError::InhibitoryBaselineOutOfRange
                ))
            ),
            "{value}: refused as the baseline's is"
        );
    }
    for (flag, value) in [(0, 1), (0, 0x8000), (0, -1), (2, 0), (2, 0x8000), (0xFF, 0)] {
        assert!(
            matches!(
                Image::decode::<8>(&with_value(flag, value), Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "flag {flag} value {value}: bytes the writer never produces"
        );
    }
    assert_eq!(
        Image::decode::<8>(&with_value(0, 0), Config::default())
            .unwrap()
            .inhibitory_baseline_q16(),
        None,
        "the flag and the value zero, as a format-14 writer left them, read as unset"
    );
    // `[32..36)` is the class of short-term plasticity's since ADR-0114, `[36..48)` but `[39]`
    // the slow current's since ADR-0123, and `[48..51)` the critic's since ADR-0131.
    for at in [26, 27, 39, 51, 63] {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| s[at] = 1);
        assert!(
            matches!(
                Image::decode::<8>(&img, Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "byte {at} is reserved"
        );
    }
    let mut older = set.clone();
    let mut header = CortexFileHeader::decode(older[0..64].try_into().unwrap());
    header.version = CortexFileHeader::FORMAT_VERSION - 1;
    header.crc64 = header.checksum();
    older[0..64].copy_from_slice(&header.encode());
    assert!(
        matches!(
            Image::decode::<8>(&older, Config::default()),
            Err(ImageError::Header(HeaderError::ForeignVersion(18)))
        ),
        "the previous format's header fails closed, as every foreign version does"
    );
}

/// The signed gate in the modulator section (ADR-0094; format 16): a flag byte at `[25]`.
/// Unset, it is zero — the byte a format-15 writer left there — and reads as unset whatever
/// the configuration says; set, it is 1 and reads as set whatever the configuration says,
/// beside the inhibitory baseline's flag and value, which it leaves as they are. Refused: a
/// flag that is neither zero nor one, which the writer never produces, and a reserved byte
/// after it; and a header stamped with format 15, as every foreign version is.
#[test]
fn the_signed_gate_is_written_to_and_read_from_the_image_and_a_byte_left_zero_reads_as_unset() {
    let unset = small_image_with_modulator();
    let section = section_bytes(&unset, SECTION_MODULATOR);
    assert_eq!(section[25], 0, "unset writes zero");
    let loaded = Image::decode::<8>(
        &unset,
        Config {
            signed_gate: true,
            ..Config::default()
        },
    )
    .unwrap();
    assert!(
        !loaded.signed_gate(),
        "the image's unset outranks the configuration's set"
    );
    let exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        inhibitory_baseline_q16: Some(0x8000),
        signed_gate: true,
        ..Config::default()
    })
    .unwrap();
    let set = Image::encode(&exec).unwrap();
    let section = section_bytes(&set, SECTION_MODULATOR);
    assert_eq!(
        (section[24], section[25]),
        (1, 1),
        "the inhibitory flag and the signed gate's"
    );
    assert_eq!(&section[26..28], &[0u8; 2]);
    assert_eq!(
        &section[28..32],
        &0x8000i32.to_le_bytes(),
        "the inhibitory value"
    );
    assert_eq!(&section[32..64], &[0u8; 32]);
    let loaded = Image::decode::<8>(&set, Config::default()).unwrap();
    assert!(
        loaded.signed_gate(),
        "the image's set outranks the configuration's unset"
    );
    assert_eq!(
        loaded.inhibitory_baseline_q16(),
        Some(0x8000),
        "and the two are apart"
    );
    let with_flag = |flag: u8| {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| s[25] = flag);
        img
    };
    assert!(
        !Image::decode::<8>(&with_flag(0), Config::default())
            .unwrap()
            .signed_gate(),
        "a zero, as a format-15 writer left it, reads as unset"
    );
    assert!(
        Image::decode::<8>(&with_flag(1), Config::default())
            .unwrap()
            .signed_gate()
    );
    for flag in [2, 3, 0x80, 0xFF] {
        assert!(
            matches!(
                Image::decode::<8>(&with_flag(flag), Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "flag {flag}: a byte the writer never produces"
        );
    }
    // `[32..36)` is the class of short-term plasticity's since ADR-0114, `[36..48)` but `[39]`
    // the slow current's since ADR-0123, and `[48..51)` the critic's since ADR-0131.
    for at in [26, 27, 39, 51, 63] {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| s[at] = 1);
        assert!(
            matches!(
                Image::decode::<8>(&img, Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "byte {at} is reserved"
        );
    }
    assert_eq!(CortexFileHeader::FORMAT_VERSION, 19);
    let mut older = set.clone();
    let mut header = CortexFileHeader::decode(older[0..64].try_into().unwrap());
    header.version = 15;
    header.crc64 = header.checksum();
    older[0..64].copy_from_slice(&header.encode());
    assert!(
        matches!(
            Image::decode::<8>(&older, Config::default()),
            Err(ImageError::Header(HeaderError::ForeignVersion(15)))
        ),
        "a format-15 header fails closed, as every foreign version does (L-6)"
    );
}

/// The class of short-term plasticity in the modulator section (ADR-0114; format 17): a flag
/// byte at `[32]` and the class's $U$ and two shifts at `[33]`, `[34]` and `[35]`. Unset, all
/// four are zero — the bytes a format-16 writer left there — and read as unset whatever the
/// configuration says; set, the flag is 1 and the class is read back, whatever the
/// configuration says, at the edges the rule resolves, beside the inhibitory baseline and the
/// signed gate, which it leaves as they are. A unit's mark (`FLAG_FACILITATING`) is carried
/// in its record. Refused: a class the rule does not resolve, as the configuration's is; a
/// flag that is neither zero nor one, a byte beside a zero flag and a reserved byte after the
/// class, which the writer never produces; a unit marked while no class is set; and a header
/// stamped with format 16, as every foreign version is.
#[test]
fn the_class_of_short_term_plasticity_is_written_to_and_read_from_the_image_and_a_record_left_zero_reads_as_unset()
 {
    let class = |u, tau_f_shift, tau_d_shift| StpClass {
        u,
        tau_f_shift,
        tau_d_shift,
    };
    let set_ii = class(26, 16, 13);
    let unset = small_image_with_modulator();
    let section = section_bytes(&unset, SECTION_MODULATOR);
    assert_eq!(&section[32..64], &[0u8; 32], "unset writes zeros");
    let loaded = Image::decode::<8>(
        &unset,
        Config {
            stp_class: Some(set_ii),
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(
        loaded.stp_class(),
        None,
        "the image's unset outranks the configuration's set"
    );
    // Set beside the inhibitory baseline and the signed gate, with unit 1 marked.
    let mut exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        inhibitory_baseline_q16: Some(0x8000),
        signed_gate: true,
        stp_class: Some(set_ii),
        ..Config::default()
    })
    .unwrap();
    exec.units_mut()[1].flags = FLAG_FACILITATING | FLAG_INHIBITORY;
    let set = Image::encode(&exec).unwrap();
    let section = section_bytes(&set, SECTION_MODULATOR);
    assert_eq!(
        (section[24], section[25]),
        (1, 1),
        "the inhibitory flag and the signed gate's"
    );
    assert_eq!(&section[28..32], &0x8000i32.to_le_bytes());
    assert_eq!(
        &section[32..36],
        &[1, 26, 16, 13],
        "the flag, U and the shifts"
    );
    assert_eq!(&section[36..64], &[0u8; 28]);
    assert_eq!(
        section_bytes(&set, SECTION_NEURON)[64 + 57],
        FLAG_FACILITATING | FLAG_INHIBITORY,
        "the mark in the unit's record"
    );
    let loaded = Image::decode::<8>(&set, Config::default()).unwrap();
    assert_eq!(
        loaded.stp_class(),
        Some(set_ii),
        "the image's set outranks the configuration's unset"
    );
    assert_eq!(
        (loaded.inhibitory_baseline_q16(), loaded.signed_gate()),
        (Some(0x8000), true),
        "and the three are apart"
    );
    assert_eq!(loaded.units()[1].flags, FLAG_FACILITATING | FLAG_INHIBITORY);
    assert_eq!(loaded.units()[0].flags, 0);
    assert_eq!(Image::encode(&loaded).unwrap(), set, "one image, twice");
    let with_class = |bytes: [u8; 4]| {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| {
            s[32..36].copy_from_slice(&bytes)
        });
        img
    };
    for (u, f, d) in [
        (1, 1, 1),
        (255, 16, 16),
        (1, 16, 1),
        (255, 1, 16),
        (51, 16, 13),
    ] {
        assert_eq!(
            Image::decode::<8>(&with_class([1, u, f, d]), Config::default())
                .unwrap()
                .stp_class(),
            Some(class(u, f, d)),
            "({u}, {f}, {d}) is set"
        );
    }
    for (u, f, d) in [
        (0, 16, 13),
        (51, 0, 13),
        (51, 17, 13),
        (51, 16, 0),
        (51, 16, 17),
        (51, 0xFF, 0xFF),
        (0, 0, 0),
    ] {
        assert!(
            matches!(
                Image::decode::<8>(&with_class([1, u, f, d]), Config::default()),
                Err(ImageError::Config(ConfigError::StpClassOutOfRange))
            ),
            "({u}, {f}, {d}): refused as the configuration's is"
        );
    }
    for bytes in [
        [0, 26, 16, 13],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
        [2, 26, 16, 13],
        [0xFF, 26, 16, 13],
        [2, 0, 0, 0],
    ] {
        assert!(
            matches!(
                Image::decode::<8>(&with_class(bytes), Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "{bytes:?}: bytes the writer never produces"
        );
    }
    // `[36..48)` but `[39]` is the slow current's since ADR-0123, and `[48..51)` the critic's
    // since ADR-0131.
    for at in [26, 27, 39, 51, 63] {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| s[at] = 1);
        assert!(
            matches!(
                Image::decode::<8>(&img, Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "byte {at} is reserved"
        );
    }
    // A mark while no class is set: refused at the marked unit, whatever the configuration
    // says; the inhibitory flag alone is no mark.
    let unmarked = with_class([0; 4]);
    let loaded = Image::decode::<8>(
        &unmarked,
        Config {
            stp_class: Some(set_ii),
            ..Config::default()
        },
    );
    assert!(
        matches!(loaded, Err(ImageError::MarkWithoutClass(1))),
        "the image's unset class, and unit 1 marked"
    );
    let mut inhibitory = unmarked.clone();
    patch_section(&mut inhibitory, SECTION_NEURON, |s| {
        s[64 + 57] = FLAG_INHIBITORY
    });
    assert_eq!(
        Image::decode::<8>(&inhibitory, Config::default())
            .unwrap()
            .stp_class(),
        None
    );
    let mut first = inhibitory.clone();
    patch_section(&mut first, SECTION_NEURON, |s| s[57] = FLAG_FACILITATING);
    assert!(matches!(
        Image::decode::<8>(&first, Config::default()),
        Err(ImageError::MarkWithoutClass(0))
    ));
    // A header stamped with format 16, as every foreign version is (L-6).
    assert_eq!(CortexFileHeader::FORMAT_VERSION, 19);
    let mut older = set.clone();
    let mut header = CortexFileHeader::decode(older[0..64].try_into().unwrap());
    header.version = 16;
    header.crc64 = header.checksum();
    older[0..64].copy_from_slice(&header.encode());
    assert!(
        matches!(
            Image::decode::<8>(&older, Config::default()),
            Err(ImageError::Header(HeaderError::ForeignVersion(16)))
        ),
        "a format-16 header fails closed, as every foreign version does (L-6)"
    );
}

/// The slow current in the modulator section and the unit's record (ADR-0123; format 18): a flag
/// byte at `[36]`, the leak and input shifts at `[37]` and `[38]`, the gate's two voltages at
/// `[40..44)` and `[44..48)`; a unit's mark (`FLAG_SLOW`) and its slow potential at `[20..24)`
/// of its record. Unset, the section's bytes are zero — those a format-17 writer left there — and
/// read as unset whatever the configuration says; set, they are read back whatever the
/// configuration says, at the edges the rule resolves, beside the inhibitory baseline, the signed
/// gate and the class, which it leaves as they are. A marked unit with a slow potential and
/// nothing else is woken on load and leaks it. Refused: constants the rule does not resolve, as
/// the configuration's are; a flag that is neither zero nor one, a byte beside a zero flag and a
/// reserved byte, which the writer never produces; a unit marked while no slow current is set; an
/// unmarked unit with a slow potential; and a header stamped with format 17, as every foreign
/// version is.
#[test]
fn the_slow_current_is_written_to_and_read_from_the_image_and_a_record_left_zero_reads_as_unset() {
    let slow = |leak_shift, input_shift, v_lo_q16, v_hi_q16| SlowCurrent {
        leak_shift,
        input_shift,
        v_lo_q16,
        v_hi_q16,
    };
    let current = slow(13, 1, 28_561, THRESHOLD_BASE);
    let set_ii = StpClass {
        u: 26,
        tau_f_shift: 16,
        tau_d_shift: 13,
    };
    let unset = small_image_with_modulator();
    let section = section_bytes(&unset, SECTION_MODULATOR);
    assert_eq!(&section[36..64], &[0u8; 28], "unset writes zeros");
    let loaded = Image::decode::<8>(
        &unset,
        Config {
            slow_current: Some(current),
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(
        loaded.slow_current(),
        None,
        "the image's unset outranks the configuration's set"
    );
    // Set beside the inhibitory baseline, the signed gate and the class, with unit 1 marked for
    // both and carrying a slow potential.
    let mut exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        inhibitory_baseline_q16: Some(0x8000),
        signed_gate: true,
        stp_class: Some(set_ii),
        slow_current: Some(current),
        ..Config::default()
    })
    .unwrap();
    exec.units_mut()[1].flags = FLAG_SLOW | FLAG_FACILITATING;
    exec.units_mut()[1].v_slow = 0x1_2345;
    let set = Image::encode(&exec).unwrap();
    let section = section_bytes(&set, SECTION_MODULATOR);
    assert_eq!((section[24], section[25]), (1, 1));
    assert_eq!(&section[28..32], &0x8000i32.to_le_bytes());
    assert_eq!(&section[32..36], &[1, 26, 16, 13], "the class");
    assert_eq!(
        &section[36..40],
        &[1, 13, 1, 0],
        "the flag, the two shifts, a reserved byte"
    );
    assert_eq!(
        &section[40..44],
        &28_561i32.to_le_bytes(),
        "the low voltage"
    );
    assert_eq!(
        &section[44..48],
        &THRESHOLD_BASE.to_le_bytes(),
        "the high voltage"
    );
    assert_eq!(&section[48..64], &[0u8; 16]);
    let neurons = section_bytes(&set, SECTION_NEURON);
    assert_eq!(neurons[64 + 57], FLAG_SLOW | FLAG_FACILITATING, "the mark");
    assert_eq!(
        &neurons[64 + 20..64 + 24],
        &0x1_2345i32.to_le_bytes(),
        "the slow potential in the unit's record"
    );
    let loaded = Image::decode::<8>(&set, Config::default()).unwrap();
    assert_eq!(
        loaded.slow_current(),
        Some(current),
        "the image's set outranks the configuration's unset"
    );
    assert_eq!(
        (
            loaded.inhibitory_baseline_q16(),
            loaded.signed_gate(),
            loaded.stp_class()
        ),
        (Some(0x8000), true, Some(set_ii)),
        "and the four are apart"
    );
    assert_eq!(
        (loaded.units()[1].flags, loaded.units()[1].v_slow),
        (FLAG_SLOW | FLAG_FACILITATING, 0x1_2345)
    );
    assert_eq!((loaded.units()[0].flags, loaded.units()[0].v_slow), (0, 0));
    assert_eq!(Image::encode(&loaded).unwrap(), set, "one image, twice");
    // The marked unit, at rest but for its slow potential, is woken on load: its first tick leaks
    // 2^-13 of the slow potential, nine LSB.
    let mut woken = loaded;
    woken.tick();
    assert_eq!(woken.units()[1].v_slow, 0x1_2345 - (0x1_2345 >> 13));
    assert_eq!(woken.units()[0].v_slow, 0);
    let with_current = |flag: u8, bytes: [u8; 11]| {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| {
            s[36] = flag;
            s[37..48].copy_from_slice(&bytes);
        });
        img
    };
    let bytes_of = |c: SlowCurrent| {
        let mut b = [0u8; 11];
        b[0] = c.leak_shift;
        b[1] = c.input_shift;
        b[3..7].copy_from_slice(&c.v_lo_q16.to_le_bytes());
        b[7..11].copy_from_slice(&c.v_hi_q16.to_le_bytes());
        b
    };
    for good in [
        slow(1, 0, 1, 2),
        slow(16, 16, 1, THRESHOLD_BASE),
        slow(13, 3, THRESHOLD_BASE - 1, THRESHOLD_BASE),
        current,
    ] {
        assert_eq!(
            Image::decode::<8>(&with_current(1, bytes_of(good)), Config::default())
                .unwrap()
                .slow_current(),
            Some(good),
            "{good:?} is set"
        );
    }
    for bad in [
        slow(0, 1, 28_561, THRESHOLD_BASE),
        slow(17, 1, 28_561, THRESHOLD_BASE),
        slow(13, 17, 28_561, THRESHOLD_BASE),
        slow(13, 0xFF, 28_561, THRESHOLD_BASE),
        slow(13, 1, 0, THRESHOLD_BASE),
        slow(13, 1, -1, THRESHOLD_BASE),
        slow(13, 1, THRESHOLD_BASE, THRESHOLD_BASE),
        slow(13, 1, 0xC000, 0x4000),
        slow(13, 1, 28_561, THRESHOLD_BASE + 1),
        slow(0, 0, 0, 0),
    ] {
        assert!(
            matches!(
                Image::decode::<8>(&with_current(1, bytes_of(bad)), Config::default()),
                Err(ImageError::Config(ConfigError::SlowCurrentOutOfRange))
            ),
            "{bad:?}: refused as the configuration's is"
        );
    }
    let mut stray = [0u8; 11];
    for at in [0, 1, 3, 6, 7, 10] {
        stray[at] = 1;
        assert!(
            matches!(
                Image::decode::<8>(&with_current(0, stray), Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "byte {} beside a zero flag: one the writer never produces",
            at + 37
        );
        stray[at] = 0;
    }
    for flag in [2, 3, 0xFF] {
        for bytes in [bytes_of(current), [0; 11]] {
            assert!(
                matches!(
                    Image::decode::<8>(&with_current(flag, bytes), Config::default()),
                    Err(ImageError::ReservedNotZero {
                        section: SECTION_MODULATOR,
                        index: 0
                    })
                ),
                "flag {flag}: a byte the writer never produces"
            );
        }
    }
    for at in [39, 51, 63] {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| s[at] = 1);
        assert!(
            matches!(
                Image::decode::<8>(&img, Config::default()),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "byte {at} is reserved"
        );
    }
    // A mark while no slow current is set: refused at the marked unit, whatever the configuration
    // says; the facilitating mark alone is no mark for it.
    let unset_current = with_current(0, [0; 11]);
    assert!(
        matches!(
            Image::decode::<8>(
                &unset_current,
                Config {
                    slow_current: Some(current),
                    ..Config::default()
                }
            ),
            Err(ImageError::MarkWithoutSlowCurrent(1))
        ),
        "the image's unset slow current, and unit 1 marked"
    );
    let mut facilitating = unset_current.clone();
    patch_section(&mut facilitating, SECTION_NEURON, |s| {
        s[64 + 57] = FLAG_FACILITATING;
        s[64 + 20..64 + 24].copy_from_slice(&[0; 4]);
    });
    assert_eq!(
        Image::decode::<8>(&facilitating, Config::default())
            .unwrap()
            .slow_current(),
        None
    );
    let mut first = facilitating.clone();
    patch_section(&mut first, SECTION_NEURON, |s| s[57] = FLAG_SLOW);
    assert!(matches!(
        Image::decode::<8>(&first, Config::default()),
        Err(ImageError::MarkWithoutSlowCurrent(0))
    ));
    // An unmarked unit carries no slow potential: nothing writes one in it.
    let mut unmarked = set.clone();
    patch_section(&mut unmarked, SECTION_NEURON, |s| {
        s[64 + 57] = FLAG_FACILITATING
    });
    assert!(
        matches!(
            Image::decode::<8>(&unmarked, Config::default()),
            Err(ImageError::NotAtRest(1))
        ),
        "a slow potential without the mark"
    );
    // A header stamped with format 17, as every foreign version is (L-6).
    assert_eq!(CortexFileHeader::FORMAT_VERSION, 19);
    let mut older = set.clone();
    let mut header = CortexFileHeader::decode(older[0..64].try_into().unwrap());
    header.version = 17;
    header.crc64 = header.checksum();
    older[0..64].copy_from_slice(&header.encode());
    assert!(
        matches!(
            Image::decode::<8>(&older, Config::default()),
            Err(ImageError::Header(HeaderError::ForeignVersion(17)))
        ),
        "a format-17 header fails closed, as every foreign version does (L-6)"
    );
}

/// The critic in the modulator section and the unit record (ADR-0131; format 19): a flag byte at
/// `[48]`, the step's shift at `[49]` and the weight's scale at `[50]`, and each unit's value
/// weight at its record's `[52..54)`. Unset, the three bytes are zero — the bytes a format-18
/// writer left there — and read as unset whatever the configuration says; set, they are read
/// back whatever the configuration says, beside the slow current, with every weight, and the
/// counts start from zero at the load. Refused: constants the rule does not resolve, as the
/// configuration's are; a critic with no train to count from; a flag of zero with a byte
/// beside it, a flag that is neither zero nor one, and a reserved byte after them, which the
/// writer never produces; a weight while the image carries no critic; and a header stamped
/// with the previous version, as every foreign version is.
#[test]
fn the_critic_is_written_to_and_read_from_the_image_and_a_record_left_zero_reads_as_unset() {
    let critic = ValueCritic { shift: 9, scale: 2 };
    let with_train = |critic: Option<ValueCritic>| Config {
        train_capacity: 8,
        critic,
        ..Config::default()
    };
    let unset = small_image_with_modulator();
    let section = section_bytes(&unset, SECTION_MODULATOR);
    assert_eq!(&section[48..64], &[0u8; 16], "unset writes zeros");
    let loaded = Image::decode::<8>(&unset, with_train(Some(critic))).unwrap();
    assert_eq!(
        (loaded.critic(), loaded.features()),
        (None, &[][..]),
        "the image's unset outranks the configuration's set"
    );
    // Set beside the slow current, with unit 1 carrying a weight.
    let current = SlowCurrent {
        leak_shift: 13,
        input_shift: 1,
        v_lo_q16: 28_561,
        v_hi_q16: THRESHOLD_BASE,
    };
    let mut exec = Executor::<8>::new(Config {
        units: 2,
        blocks: 1,
        slow_current: Some(current),
        ..with_train(Some(critic))
    })
    .unwrap();
    exec.units_mut()[1].value_weight = -1_234;
    let set = Image::encode(&exec).unwrap();
    let section = section_bytes(&set, SECTION_MODULATOR);
    assert_eq!(section[36], 1, "the slow current's flag");
    assert_eq!(
        &section[48..51],
        &[1, 9, 2],
        "the flag, the shift, the scale"
    );
    assert_eq!(&section[51..64], &[0u8; 13]);
    let neurons = section_bytes(&set, SECTION_NEURON);
    assert_eq!(
        &neurons[64 + 52..64 + 54],
        &(-1_234i16).to_le_bytes(),
        "the weight in the unit's record"
    );
    assert_eq!(&neurons[52..54], &[0, 0]);
    let loaded = Image::decode::<8>(&set, with_train(None)).unwrap();
    assert_eq!(
        (loaded.critic(), loaded.slow_current()),
        (Some(critic), Some(current)),
        "the image's set outranks the configuration's unset, and the two are apart"
    );
    assert_eq!(
        (
            loaded.units()[0].value_weight,
            loaded.units()[1].value_weight
        ),
        (0, -1_234)
    );
    assert_eq!(
        (loaded.features(), loaded.prediction()),
        (&[0, 0][..], None),
        "the counts start at the load"
    );
    assert_eq!(Image::encode(&loaded).unwrap(), set, "one image, twice");
    assert!(
        matches!(
            Image::decode::<8>(&set, Config::default()),
            Err(ImageError::Config(ConfigError::CriticWithoutTrain))
        ),
        "the image's critic needs a train the configuration gives"
    );
    let with_critic = |bytes: [u8; 3]| {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| {
            s[48..51].copy_from_slice(&bytes)
        });
        img
    };
    for (shift, scale) in [(0, 0), (30, 14), (30, 0), (0, 14)] {
        assert_eq!(
            Image::decode::<8>(&with_critic([1, shift, scale]), with_train(None))
                .unwrap()
                .critic(),
            Some(ValueCritic { shift, scale }),
            "{shift} {scale} is set"
        );
    }
    for (shift, scale) in [(31, 2), (9, 15), (0xFF, 0), (0, 0xFF)] {
        assert!(
            matches!(
                Image::decode::<8>(&with_critic([1, shift, scale]), with_train(None)),
                Err(ImageError::Config(ConfigError::CriticOutOfRange))
            ),
            "{shift} {scale}: refused as the configuration's is"
        );
    }
    for bytes in [
        [0, 1, 0],
        [0, 0, 1],
        [0, 9, 2],
        [2, 9, 2],
        [3, 0, 0],
        [0xFF, 9, 2],
    ] {
        assert!(
            matches!(
                Image::decode::<8>(&with_critic(bytes), with_train(None)),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "{bytes:?}: bytes the writer never produces"
        );
    }
    for at in [51, 63] {
        let mut img = set.clone();
        patch_section(&mut img, SECTION_MODULATOR, |s| s[at] = 1);
        assert!(
            matches!(
                Image::decode::<8>(&img, with_train(None)),
                Err(ImageError::ReservedNotZero {
                    section: SECTION_MODULATOR,
                    index: 0
                })
            ),
            "byte {at} is reserved"
        );
    }
    // A weight while no critic is set: refused at the unit that carries it, whatever the
    // configuration says; with every weight zero the image loads unset.
    let unset_critic = with_critic([0, 0, 0]);
    assert!(
        matches!(
            Image::decode::<8>(&unset_critic, with_train(Some(critic))),
            Err(ImageError::ValueWithoutCritic(1))
        ),
        "the image's unset critic, and unit 1 carrying a weight"
    );
    let mut first = unset_critic.clone();
    patch_section(&mut first, SECTION_NEURON, |s| {
        s[64 + 52..64 + 54].copy_from_slice(&[0, 0]);
        s[52] = 1;
    });
    assert!(matches!(
        Image::decode::<8>(&first, with_train(None)),
        Err(ImageError::ValueWithoutCritic(0))
    ));
    let mut none = unset_critic.clone();
    patch_section(&mut none, SECTION_NEURON, |s| {
        s[64 + 52..64 + 54].copy_from_slice(&[0, 0])
    });
    assert_eq!(
        Image::decode::<8>(&none, with_train(None))
            .unwrap()
            .critic(),
        None
    );
    // A header stamped with format 18, as every foreign version is (L-6).
    assert_eq!(CortexFileHeader::FORMAT_VERSION, 19);
    let mut older = set.clone();
    let mut header = CortexFileHeader::decode(older[0..64].try_into().unwrap());
    header.version = 18;
    header.crc64 = header.checksum();
    older[0..64].copy_from_slice(&header.encode());
    assert!(
        matches!(
            Image::decode::<8>(&older, with_train(None)),
            Err(ImageError::Header(HeaderError::ForeignVersion(18)))
        ),
        "a format-18 header fails closed, as every foreign version does (L-6)"
    );
}

/// Milestone M4's exit test with the critic set (ADR-0131): a unit that fired since the previous
/// reward is not evicted, so the critic reads and moves its weight in its slot at the next
/// reward, and a run that sweeps, re-hydrates and rewards ends in the same image as one that
/// never evicts, every weight included. A unit quiet and at rest with a spike pending stays
/// through a sweep and goes at the first after a reward.
#[test]
fn with_the_critic_set_a_unit_with_spikes_pending_stays_and_evict_spike_rehydrate_keeps_every_weight()
 {
    let critic = Some(ValueCritic { shift: 9, scale: 2 });
    let valued = || Config {
        train_capacity: 1 << 12,
        critic,
        ..config()
    };
    let log_path = scratch("critic-sweep.wal");
    let mut swept = Executor::<64>::new(valued()).unwrap();
    let mut control = Executor::<64>::new(valued()).unwrap();
    wire(&mut swept);
    wire(&mut control);
    swept.attach_log(&log_path).expect("a log");
    let mut x = 0x9E37_79B9u32;
    let mut next = || {
        x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        x
    };
    let mut pending_kept = 0usize;
    for round in 0..60u32 {
        let unit = (next() >> 8) % 96;
        kick(&swept, unit, 14);
        kick(&control, unit, 14);
        swept.run(100);
        control.run(100);
        if round % 3 == 2 {
            let pending: Vec<usize> = (0..96)
                .filter(|&i| swept.features()[i] != 0 && !swept.is_evicted(i as u32))
                .collect();
            swept.sweep(150, 96).expect("a sweep");
            for &i in &pending {
                assert!(
                    !swept.is_evicted(i as u32),
                    "round {round}: unit {i} pending"
                );
            }
            pending_kept = pending_kept.saturating_add(pending.len());
        }
        for i in 0..96u32 {
            if swept.is_evicted(i) {
                assert_eq!(swept.features()[i as usize], 0, "round {round}: unit {i}");
            }
        }
        if round % 5 == 4 {
            let reward = if round % 2 == 0 { 0x1_0000 } else { -0x8000 };
            assert_eq!(
                swept.reward(reward),
                control.reward(reward),
                "round {round}"
            );
            assert_eq!(swept.prediction(), control.prediction());
        }
    }
    assert!(swept.evictions() > 0 && swept.rehydrations() > 0);
    assert!(pending_kept > 0, "a sweep met a unit with spikes pending");
    assert!(
        control.units().iter().any(|u| u.value_weight != 0),
        "the critic moved weights"
    );
    settle(&mut swept, 40_000);
    settle(&mut control, 40_000);
    assert_eq!(
        Image::encode(&swept).expect("the swept image"),
        Image::encode(&control).expect("the control image"),
        "every weight included"
    );
    let _ = std::fs::remove_file(&log_path);
    // One unit, quiet and at rest with a spike pending: kept, then swept after a reward.
    let mut exec = Executor::<64>::new(Config {
        blocks: 0,
        ..valued()
    })
    .unwrap();
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
    exec.attach_log(&scratch("critic-pending.wal"))
        .expect("a log");
    kick(&exec, 5, 14);
    settle(&mut exec, 40_000);
    exec.run(20_000);
    let five = &exec.units()[5];
    assert!(
        five.v_soma == 0
            && five.v_basal == 0
            && five.refractory_ticks == 0
            && five.v_thresh == THRESHOLD_BASE,
        "unit 5 at rest"
    );
    assert!(exec.features()[5] > 0, "unit 5 fired");
    assert_eq!(
        exec.sweep(0, 96).unwrap(),
        95,
        "every unit but the one pending"
    );
    assert!(!exec.is_evicted(5));
    exec.reward(0x1_0000);
    assert_eq!(exec.sweep(0, 96).unwrap(), 1, "after the reward it goes");
    assert!(exec.is_evicted(5));
}
