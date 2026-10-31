//! The tables H-46's round pinned from its runs, a row a line (ADR-0227).

use crate::*;

/// H-46's verdict, as read; none until its twelve runs are pinned.
pub(crate) const ZERO_1024: Option<ZeroVerdict> = None;

/// The step of ADR-0226's stopping rule the verdict reaches.
pub(crate) const ZERO_STOP_1024: Option<u8> = None;

/// ADR-0226's four predicted readings, as read.
pub(crate) const ZERO_READINGS_READ_1024: Option<[Option<bool>; 4]> = None;

/// H-42's image on each network by sixteenths of the width, as the first halves read it.
pub(crate) const ZERO_IMAGE_BINS: [Option<Bins>; 3] = [None; 3];

/// The eighth image's CRC-64, `[network][form][arm]`, as each first half printed it.
pub(crate) const ZERO_IMAGES_CRC: [[[Option<u64>; 2]; 2]; 3] = [
    [
        [Some(0xc4a4_465d_974f_b445), Some(0x4d8c_c100_30ff_6f31)],
        [Some(0xedcd_0f2e_2d3f_e331), Some(0x07da_b225_9bdb_a1f9)],
    ],
    [
        [Some(0xdcc1_5576_0462_7cb5), Some(0xab95_a196_c9da_c82f)],
        [Some(0x5bb3_9e3c_7892_2e0e), Some(0xc467_3929_e111_0110)],
    ],
    [
        [Some(0x7c95_780f_087c_b3b0), Some(0x3ee9_c0ff_0a05_014e)],
        [Some(0xde6f_2d87_47b6_692b), Some(0xe7d7_4c85_14e0_2d88)],
    ],
];

/// Each run's rows, `[network][form][arm]`, one a schedule, from its two halves; empty until they ran.
pub(crate) const ZERO_ROWS: [[[&[ZeroRow]; 2]; 2]; 3] = [[[&[]; 2]; 2]; 3];
