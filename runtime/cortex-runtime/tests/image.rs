//! Milestone M4's exit test and the image round trip (whitepaper Appendix C; brief 015;
//! ADR-0024): an image written from arenas and opened again is byte-identical and runs
//! identically; a corrupted, truncated or foreign image fails closed; a synapse delay the wheel
//! cannot hold is refused at load; and evict, spike, re-hydrate preserves a unit bit for bit:
//! a run that sweeps and re-hydrates ends in the same image as one that never evicts.

use cortex_connectome::{CortexFileHeader, HeaderError, SECTION_NEURON};
use cortex_core::{
    PlasticDelta, STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16,
};
use cortex_runtime::{Config, Executor, Image, ImageError};
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
