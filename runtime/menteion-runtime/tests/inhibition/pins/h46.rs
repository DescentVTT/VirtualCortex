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

/// The eighth image's CRC-64, `[network][form][arm]`, as each first half printed it; none until the commit that adds the images.
pub(crate) const ZERO_IMAGES_CRC: [[[Option<u64>; 2]; 2]; 3] = [[[None; 2]; 2]; 3];

/// Each run's rows, `[network][form][arm]`, one a schedule, from its two halves; empty until they ran.
pub(crate) const ZERO_ROWS: [[[&[ZeroRow]; 2]; 2]; 3] = [[[&[]; 2]; 2]; 3];
