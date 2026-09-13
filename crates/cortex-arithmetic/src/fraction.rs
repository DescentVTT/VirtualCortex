//! Continued fractions through the scratchpad (whitepaper §5.2.31, §6.10, §8.8; ADR-0042;
//! brief 021): the first sequencing of slots into an expression. A polynomial continued
//! fraction is $a_0 + b_1 / (a_1 + b_2 / (a_2 + \dots))$ with $a_n$ and $b_n$ polynomials in
//! $n$ of degree at most two; its convergents $p_n / q_n$ follow the recurrence
//! $p_n = a_n p_{n-1} + b_n p_{n-2}$, $q_n = a_n q_{n-1} + b_n q_{n-2}$ from
//! $(p_{-1}, p_0, q_{-1}, q_0) = (1, a_0, 0, 1)$ (Wall 1948), every multiplication and
//! addition an `execute` of the slot, so that an overflow is the slot's flag and the slot is
//! left showing the operation that failed. [`within`] decides exactly whether a convergent
//! lies within a rational tolerance of a rational target; [`search`] enumerates the
//! coefficient tuples of a bound and reports those whose deepest comparable convergent does.
//! A target is two integers the caller supplies; what a match means is the caller's
//! conjecture, which the discovery path of the runtime can send to the broker for a
//! certificate (§6.10). The meet-in-the-middle search over rational functions of the target
//! (Raayoni et al. 2021) is Specified.

use crate::{
    ArithmeticScratchpadSlot, ERR_DIVIDE_BY_ZERO, ERR_OVERFLOW, OP_ADD, OP_DIV, OP_MUL, OP_SUB,
};

/// A polynomial in the depth: $c_0 + c_1 n + c_2 n^2$.
pub type Poly = [i64; 3];
/// The deepest convergent a walk computes.
pub const MAX_DEPTH: u32 = 64;
/// The highest degree the search enumerates.
pub const MAX_DEGREE: u8 = 2;

/// FNV-1a, 32 bits: the offset basis and the prime.
const FNV_OFFSET: u32 = 0x811C_9DC5;
const FNV_PRIME: u32 = 0x0100_0193;

/// A convergent $p / q$ at a depth.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Convergent {
    pub p: i128,
    pub q: i128,
    pub depth: u32,
}

/// Why a walk stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FractionError {
    /// A depth above `MAX_DEPTH` was asked for.
    DepthExceeded,
    /// The slot flagged an operation (`ERR_*` bits); the slot shows which.
    Arithmetic(u16),
}

/// Why a search did not run, or did not fit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchError {
    /// A degree above `MAX_DEGREE`, a negative bound, a depth above `MAX_DEPTH`, a zero
    /// denominator or a non-positive tolerance.
    Bound,
    /// One more candidate matched than the output slice holds; the slice is full.
    OutFull,
}

/// A coefficient tuple whose deepest comparable convergent lies within the tolerance.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Candidate {
    pub a: Poly,
    pub b: Poly,
    pub convergent: Convergent,
}

impl Candidate {
    /// The hash a frame carries for this candidate as its `param_hash`: FNV-1a over the six
    /// coefficients, little-endian.
    pub fn statement_hash(&self) -> u32 {
        let mut hash = FNV_OFFSET;
        for coefficient in self.a.iter().chain(self.b.iter()) {
            for byte in coefficient.to_le_bytes() {
                hash = (hash ^ u32::from(byte)).wrapping_mul(FNV_PRIME);
            }
        }
        hash
    }
}

/// The polynomial at `n`, exact: with 64-bit coefficients and a 32-bit `n` the magnitude is
/// at most $2^{63} (2^{32} - 1)^2 + 2^{63} (2^{32} - 1) + 2^{63} < 2^{127}$, so nothing here
/// can wrap; the operations are named as §8.1 asks.
pub fn poly_at(c: &Poly, n: u32) -> i128 {
    let n = i128::from(n);
    let linear = i128::from(c[1]).wrapping_mul(n);
    let square = i128::from(c[2]).wrapping_mul(n).wrapping_mul(n);
    i128::from(c[0]).wrapping_add(linear).wrapping_add(square)
}

/// One operation of the slot: `Ok(result)` or the flags it raised.
fn op(slot: &mut ArithmeticScratchpadSlot, opcode: u8, a: i128, b: i128) -> Result<i128, u16> {
    slot.opcode = opcode;
    slot.set_operands(a, b);
    if slot.execute() {
        Ok(slot.result())
    } else {
        Err(slot.error_flags)
    }
}

/// $x y + z w$ through the slot.
fn mul_add(
    slot: &mut ArithmeticScratchpadSlot,
    x: i128,
    y: i128,
    z: i128,
    w: i128,
) -> Result<i128, u16> {
    let first = op(slot, OP_MUL, x, y)?;
    let second = op(slot, OP_MUL, z, w)?;
    op(slot, OP_ADD, first, second)
}

/// The recurrence's state: the two last convergents.
struct Walk {
    p_prev: i128,
    p: i128,
    q_prev: i128,
    q: i128,
    depth: u32,
}

impl Walk {
    /// Depth zero: $p_0 = a_0$, $q_0 = 1$.
    fn start(a: &Poly) -> Self {
        Self {
            p_prev: 1,
            p: poly_at(a, 0),
            q_prev: 0,
            q: 1,
            depth: 0,
        }
    }

    fn current(&self) -> Convergent {
        Convergent {
            p: self.p,
            q: self.q,
            depth: self.depth,
        }
    }

    /// One depth deeper; on a flag the walk is unchanged.
    fn advance(
        &mut self,
        a: &Poly,
        b: &Poly,
        slot: &mut ArithmeticScratchpadSlot,
    ) -> Result<(), u16> {
        // Below `MAX_DEPTH`, which every caller holds.
        let n = self.depth.wrapping_add(1);
        let (an, bn) = (poly_at(a, n), poly_at(b, n));
        let p = mul_add(slot, an, self.p, bn, self.p_prev)?;
        let q = mul_add(slot, an, self.q, bn, self.q_prev)?;
        self.p_prev = self.p;
        self.p = p;
        self.q_prev = self.q;
        self.q = q;
        self.depth = n;
        Ok(())
    }
}

/// The convergent at exactly `depth`, or why the walk stopped short.
pub fn convergent(
    a: &Poly,
    b: &Poly,
    depth: u32,
    slot: &mut ArithmeticScratchpadSlot,
) -> Result<Convergent, FractionError> {
    if depth > MAX_DEPTH {
        return Err(FractionError::DepthExceeded);
    }
    let mut walk = Walk::start(a);
    while walk.depth < depth {
        walk.advance(a, b, slot)
            .map_err(FractionError::Arithmetic)?;
    }
    Ok(walk.current())
}

/// The deepest convergent up to `depth` the slot can compute: the walk stops at the first
/// flag and returns the last convergent before it.
pub fn deepest(
    a: &Poly,
    b: &Poly,
    depth: u32,
    slot: &mut ArithmeticScratchpadSlot,
) -> Result<Convergent, FractionError> {
    if depth > MAX_DEPTH {
        return Err(FractionError::DepthExceeded);
    }
    let mut walk = Walk::start(a);
    while walk.depth < depth && walk.advance(a, b, slot).is_ok() {}
    Ok(walk.current())
}

/// The greatest common divisor of two magnitudes; one when both are zero.
fn gcd(mut a: u128, mut b: u128) -> u128 {
    // `checked_rem` is `None` exactly when `b` is zero, which ends Euclid's loop.
    while let Some(r) = a.checked_rem(b) {
        a = b;
        b = r;
    }
    a.max(1)
}

/// Whether $\lvert p/q - t_n/t_d \rvert \le \lvert o_n/o_d \rvert$, decided exactly: $p/q$ is
/// reduced by its greatest common divisor, then
/// $\lvert p\,t_d - t_n\,q \rvert \cdot \lvert o_d \rvert \le \lvert o_n \rvert \cdot \lvert q\,t_d \rvert$
/// through the slot. `Err(ERR_DIVIDE_BY_ZERO)` for a zero `q`, target denominator or
/// tolerance denominator; `Err(ERR_OVERFLOW)` when a product does not fit 128 bits.
pub fn within(
    c: &Convergent,
    target: (i128, i128),
    tolerance: (i128, i128),
    slot: &mut ArithmeticScratchpadSlot,
) -> Result<bool, u16> {
    let (tn, td) = target;
    let (on, od) = tolerance;
    if c.q == 0 || td == 0 || od == 0 {
        return Err(ERR_DIVIDE_BY_ZERO);
    }
    // At most the smaller magnitude, so it fits `i128` as a positive value.
    let g = gcd(c.p.unsigned_abs(), c.q.unsigned_abs()) as i128;
    let p = op(slot, OP_DIV, c.p, g)?;
    let q = op(slot, OP_DIV, c.q, g)?;
    let p_td = op(slot, OP_MUL, p, td)?;
    let tn_q = op(slot, OP_MUL, tn, q)?;
    let difference = op(slot, OP_SUB, p_td, tn_q)?
        .checked_abs()
        .ok_or(ERR_OVERFLOW)?;
    let od_abs = od.checked_abs().ok_or(ERR_OVERFLOW)?;
    let on_abs = on.checked_abs().ok_or(ERR_OVERFLOW)?;
    let left = op(slot, OP_MUL, difference, od_abs)?;
    let q_td = op(slot, OP_MUL, q, td)?.checked_abs().ok_or(ERR_OVERFLOW)?;
    let right = op(slot, OP_MUL, on_abs, q_td)?;
    Ok(left <= right)
}

/// Enumerates every pair of polynomials whose coefficients up to `degree` lie in
/// $[-\text{bound}, \text{bound}]$ (the lowest coefficient varying fastest, $a$ before $b$),
/// skipping an $a$ or $b$ that is identically zero, walks each candidate's convergents to
/// `depth` or the first flag, remembers the verdict of the deepest depth at which [`within`]
/// could be decided, and writes the candidates whose verdict is within into `out`, in that
/// order. Returns how many. `Bound` for a degree above `MAX_DEGREE`, a negative bound, a
/// depth above `MAX_DEPTH`, a zero denominator or a non-positive tolerance; `OutFull` when
/// one more candidate matched than `out` holds, with `out` full.
pub fn search(
    target: (i128, i128),
    tolerance: (i128, i128),
    degree: u8,
    bound: i64,
    depth: u32,
    slot: &mut ArithmeticScratchpadSlot,
    out: &mut [Candidate],
) -> Result<usize, SearchError> {
    if degree > MAX_DEGREE
        || bound < 0
        || depth > MAX_DEPTH
        || target.1 == 0
        || tolerance.0 <= 0
        || tolerance.1 <= 0
    {
        return Err(SearchError::Bound);
    }
    let active: [bool; 6] = [
        true,
        degree >= 1,
        degree >= 2,
        true,
        degree >= 1,
        degree >= 2,
    ];
    let mut coefficients = [0i64; 6];
    for (c, &on) in coefficients.iter_mut().zip(active.iter()) {
        if on {
            *c = bound.wrapping_neg();
        }
    }
    let mut found = 0usize;
    loop {
        let a: Poly = [coefficients[0], coefficients[1], coefficients[2]];
        let b: Poly = [coefficients[3], coefficients[4], coefficients[5]];
        if a != [0; 3] && b != [0; 3] {
            if let Some(matched) = deepest_verdict(&a, &b, target, tolerance, depth, slot) {
                if found >= out.len() {
                    return Err(SearchError::OutFull);
                }
                out[found] = Candidate {
                    a,
                    b,
                    convergent: matched,
                };
                found = found.wrapping_add(1);
            }
        }
        // The odometer: the lowest active coefficient steps; at the bound it wraps and
        // carries; a carry past the last active coefficient ends the enumeration.
        let mut carried = true;
        for (c, &on) in coefficients.iter_mut().zip(active.iter()) {
            if !on {
                continue;
            }
            if *c < bound {
                // Below the bound, so within `i64`.
                *c = c.wrapping_add(1);
                carried = false;
                break;
            }
            *c = bound.wrapping_neg();
        }
        if carried {
            return Ok(found);
        }
    }
}

/// The convergent at the deepest depth where the comparison could be decided, when that
/// verdict was within the tolerance.
fn deepest_verdict(
    a: &Poly,
    b: &Poly,
    target: (i128, i128),
    tolerance: (i128, i128),
    depth: u32,
    slot: &mut ArithmeticScratchpadSlot,
) -> Option<Convergent> {
    let mut walk = Walk::start(a);
    let mut verdict = None;
    loop {
        let current = walk.current();
        if current.q != 0 {
            if let Ok(inside) = within(&current, target, tolerance, slot) {
                verdict = Some((inside, current));
            }
        }
        if walk.depth >= depth || walk.advance(a, b, slot).is_err() {
            break;
        }
    }
    match verdict {
        Some((true, convergent)) => Some(convergent),
        _ => None,
    }
}

const _: () = {
    assert!(MAX_DEPTH >= 1);
    // The search's odometer has six coefficients: three per polynomial.
    assert!(MAX_DEGREE == 2);
};

#[cfg(test)]
mod tests {
    use super::*;

    /// $e = 3 - 1/(4 - 2/(5 - \dots))$: $a_n = n + 3$, $b_n = -n$.
    const E_A: Poly = [3, 1, 0];
    const E_B: Poly = [0, -1, 0];
    /// $1 + \sqrt 2 = 2 + 1/(2 + 1/(2 + \dots))$.
    const SILVER_A: Poly = [2, 0, 0];
    const SILVER_B: Poly = [1, 0, 0];
    /// The targets to fifteen decimals, as rationals, and the tolerance $10^{-12}$.
    const E: (i128, i128) = (2_718_281_828_459_045, 1_000_000_000_000_000);
    const SILVER: (i128, i128) = (2_414_213_562_373_095, 1_000_000_000_000_000);
    const TOLERANCE: (i128, i128) = (1, 1_000_000_000_000);

    fn slot() -> ArithmeticScratchpadSlot {
        ArithmeticScratchpadSlot::default()
    }

    #[test]
    fn a_polynomial_is_evaluated_exactly_at_every_depth_the_types_allow() {
        assert_eq!(poly_at(&[3, 1, 0], 0), 3);
        assert_eq!(poly_at(&[3, 1, 0], 20), 23);
        assert_eq!(poly_at(&[0, -1, 0], 7), -7);
        assert_eq!(poly_at(&[1, 2, 3], 4), 57);
        assert_eq!(
            poly_at(&[0, 0, i64::MAX], u32::MAX),
            170_141_183_381_241_069_208_199_594_094_075_314_175,
            "the square term alone"
        );
        assert_eq!(
            poly_at(&[i64::MAX; 3], u32::MAX),
            170_141_183_420_855_150_465_331_762_886_552_322_047,
            "the largest value: below 2^127"
        );
        assert_eq!(
            poly_at(&[i64::MIN; 3], u32::MAX),
            -170_141_183_420_855_150_483_778_506_955_966_906_368,
            "the smallest: above -2^127"
        );
    }

    #[test]
    fn the_convergents_of_e_and_the_silver_ratio_are_pinned() {
        let mut slot = slot();
        let c = convergent(&E_A, &E_B, 20, &mut slot).unwrap();
        assert_eq!(
            c,
            Convergent {
                p: 2_916_471_173_788_403_280_463,
                q: 1_072_909_785_605_898_240_000,
                depth: 20
            }
        );
        assert_eq!(slot.error_flags, 0);
        assert_eq!(
            convergent(&E_A, &E_B, 0, &mut slot),
            Ok(Convergent {
                p: 3,
                q: 1,
                depth: 0
            }),
            "depth zero is a_0 / 1"
        );
        assert_eq!(
            convergent(&E_A, &E_B, 1, &mut slot),
            Ok(Convergent {
                p: 11,
                q: 4,
                depth: 1
            }),
            "3 - 1/4"
        );
        assert_eq!(
            convergent(&SILVER_A, &SILVER_B, 40, &mut slot),
            Ok(Convergent {
                p: 4_217_293_152_016_490,
                q: 1_746_860_020_068_409,
                depth: 40
            })
        );
        assert_eq!(
            convergent(&SILVER_A, &SILVER_B, 64, &mut slot),
            Ok(Convergent {
                p: 6_481_122_629_115_441_680_520_770,
                q: 2_684_568_892_382_786_771_291_329,
                depth: 64
            })
        );
    }

    #[test]
    fn a_walk_stops_at_the_slot_s_flag_and_the_depth_bound() {
        let mut slot = slot();
        let deep = deepest(&E_A, &E_B, 64, &mut slot).unwrap();
        assert_eq!(deep.depth, 31, "the step to 32 does not fit 128 bits");
        assert_eq!(
            (deep.p, deep.q),
            (
                22_888_440_721_410_538_270_255_849_395_716_845_601,
                8_420_186_781_878_192_965_350_976_389_120_000_000
            )
        );
        assert_eq!(slot.error_flags, ERR_OVERFLOW, "the slot shows the failure");
        assert_eq!(
            convergent(&E_A, &E_B, 32, &mut slot),
            Err(FractionError::Arithmetic(ERR_OVERFLOW))
        );
        assert_eq!(convergent(&E_A, &E_B, 31, &mut slot), Ok(deep));
        assert_eq!(
            deepest(&E_A, &E_B, 10, &mut slot).unwrap().depth,
            10,
            "the bound, not a flag"
        );
        assert_eq!(
            convergent(&E_A, &E_B, 65, &mut slot),
            Err(FractionError::DepthExceeded)
        );
        assert_eq!(
            deepest(&E_A, &E_B, 65, &mut slot),
            Err(FractionError::DepthExceeded)
        );
        // The largest constant term: one step fits, the second does not.
        let huge: Poly = [i64::MAX, 0, 0];
        assert_eq!(
            convergent(&huge, &SILVER_B, 1, &mut slot).map(|c| c.depth),
            Ok(1),
            "MAX squared plus one fits"
        );
        assert_eq!(
            convergent(&huge, &SILVER_B, 2, &mut slot),
            Err(FractionError::Arithmetic(ERR_OVERFLOW)),
            "MAX cubed does not"
        );
        assert_eq!(
            convergent(&[0, 0, i64::MAX], &[i64::MAX; 3], 64, &mut slot),
            Err(FractionError::Arithmetic(ERR_OVERFLOW)),
            "the products outgrow the width"
        );
    }

    #[test]
    fn within_is_decided_exactly_and_reports_its_failures() {
        let mut slot = slot();
        let c = convergent(&E_A, &E_B, 20, &mut slot).unwrap();
        assert_eq!(within(&c, E, TOLERANCE, &mut slot), Ok(true));
        assert_eq!(
            within(&c, E, (1, 100_000_000_000_000_000), &mut slot),
            Ok(false),
            "not within 1e-17 of a fifteen-digit rational"
        );
        assert_eq!(
            within(&c, E, (1, 1_000_000_000_000_000_000), &mut slot),
            Err(ERR_OVERFLOW),
            "the difference times 1e18 does not fit"
        );
        assert_eq!(
            within(&c, E, (-1, -1_000_000_000_000), &mut slot),
            Ok(true),
            "signs ignored"
        );
        let three = Convergent {
            p: 6,
            q: 2,
            depth: 0,
        };
        assert_eq!(
            within(&three, (3, 1), (0, 1), &mut slot),
            Ok(true),
            "6/2 is exactly 3"
        );
        assert_eq!(within(&three, (3, 1), (1, 1_000), &mut slot), Ok(true));
        assert_eq!(
            within(&three, (7, 2), (1, 2), &mut slot),
            Ok(true),
            "at the tolerance"
        );
        assert_eq!(
            within(&three, (7, 2), (499, 1_000), &mut slot),
            Ok(false),
            "just inside it"
        );
        assert_eq!(
            within(&three, (-3, -1), (0, 1), &mut slot),
            Ok(true),
            "a negative denominator"
        );
        let zero_q = Convergent {
            p: 1,
            q: 0,
            depth: 0,
        };
        assert_eq!(
            within(&zero_q, (3, 1), (1, 1), &mut slot),
            Err(ERR_DIVIDE_BY_ZERO)
        );
        assert_eq!(
            within(&three, (3, 0), (1, 1), &mut slot),
            Err(ERR_DIVIDE_BY_ZERO)
        );
        assert_eq!(
            within(&three, (3, 1), (1, 0), &mut slot),
            Err(ERR_DIVIDE_BY_ZERO)
        );
        let deep = deepest(&E_A, &E_B, 64, &mut slot).unwrap();
        assert_eq!(
            within(&deep, E, TOLERANCE, &mut slot),
            Err(ERR_OVERFLOW),
            "2^124 times 10^15"
        );
        let min = Convergent {
            p: i128::MIN,
            q: 1,
            depth: 0,
        };
        assert_eq!(
            within(&min, (0, 1), (1, 1), &mut slot),
            Err(ERR_OVERFLOW),
            "|MIN|"
        );
        assert_eq!(
            within(&three, (3, 1), (i128::MIN, 1), &mut slot),
            Err(ERR_OVERFLOW)
        );
        assert_eq!(
            within(&three, (3, 1), (1, i128::MIN), &mut slot),
            Err(ERR_OVERFLOW)
        );
        let q_min = Convergent {
            p: 1,
            q: i128::MIN,
            depth: 0,
        };
        assert_eq!(
            within(&q_min, (0, 1), (1, 1), &mut slot),
            Err(ERR_OVERFLOW),
            "|q t_d|"
        );
        assert_eq!(gcd(0, 0), 1);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(7, 0), 7);
    }

    #[test]
    fn the_search_reports_the_one_tuple_near_e_and_the_one_near_the_silver_ratio() {
        let mut slot = slot();
        let mut out = [Candidate::default(); 4];
        assert_eq!(search(E, TOLERANCE, 1, 3, 20, &mut slot, &mut out), Ok(1));
        assert_eq!(
            out[0],
            Candidate {
                a: E_A,
                b: E_B,
                convergent: Convergent {
                    p: 2_916_471_173_788_403_280_463,
                    q: 1_072_909_785_605_898_240_000,
                    depth: 20
                }
            }
        );
        assert_eq!(out[0].statement_hash(), 0x4117_9fdf);
        // At the depth bound itself the walk overflows at 32 and the comparison last fits at
        // depth 21: the candidate carries that convergent.
        assert_eq!(
            search(E, TOLERANCE, 1, 3, MAX_DEPTH, &mut slot, &mut out),
            Ok(1)
        );
        assert_eq!(
            out[0].convergent,
            Convergent {
                p: 67_217_716_576_837_485_130_671,
                q: 24_728_016_011_107_368_960_000,
                depth: 21
            }
        );
        assert_eq!(
            search(SILVER, TOLERANCE, 0, 2, 40, &mut slot, &mut out),
            Ok(1)
        );
        assert_eq!(
            out[0],
            Candidate {
                a: SILVER_A,
                b: SILVER_B,
                convergent: Convergent {
                    p: 4_217_293_152_016_490,
                    q: 1_746_860_020_068_409,
                    depth: 40
                }
            }
        );
        assert_eq!(out[0].statement_hash(), 0xe11e_83a6);
        assert_ne!(
            Candidate {
                a: SILVER_B,
                b: SILVER_A,
                ..out[0]
            }
            .statement_hash(),
            0xe11e_83a6,
            "a and b are not interchangeable in the hash"
        );
        // Degree two enumerates the degree-one tuples among others: the same one match.
        assert_eq!(
            search(E, TOLERANCE, 2, 1, 20, &mut slot, &mut out),
            Ok(0),
            "a_0 = 3 is out"
        );
        let mut none: [Candidate; 0] = [];
        assert_eq!(
            search(SILVER, TOLERANCE, 0, 2, 40, &mut slot, &mut none),
            Err(SearchError::OutFull)
        );
        // A loose tolerance matches four tuples, in enumeration order (a_0 fastest).
        assert_eq!(search(SILVER, (1, 2), 0, 2, 8, &mut slot, &mut out), Ok(4));
        assert_eq!(
            out.map(|c| (c.a[0], c.b[0], c.convergent.depth)),
            [(2, -2, 8), (2, 1, 8), (1, 2, 8), (2, 2, 8)]
        );
        assert_eq!(
            search(SILVER, (1, 2), 0, 2, 8, &mut slot, &mut out[..3]),
            Err(SearchError::OutFull),
            "three slots hold three; the fourth match does not fit"
        );
        assert_eq!(out[2].b[0], 2, "the slice was filled before it overflowed");
    }

    #[test]
    fn the_search_refuses_every_bound() {
        let mut slot = slot();
        let mut out = [Candidate::default(); 1];
        for (target, tolerance, degree, bound, depth) in [
            (E, TOLERANCE, 3, 1, 1),
            (E, TOLERANCE, 0, -1, 1),
            (E, TOLERANCE, 0, 1, 65),
            ((1, 0), TOLERANCE, 0, 1, 1),
            (E, (0, 1), 0, 1, 1),
            (E, (-1, 1), 0, 1, 1),
            (E, (1, 0), 0, 1, 1),
            (E, (1, -1), 0, 1, 1),
        ] {
            assert_eq!(
                search(target, tolerance, degree, bound, depth, &mut slot, &mut out),
                Err(SearchError::Bound),
                "{target:?} {tolerance:?} {degree} {bound} {depth}"
            );
        }
        assert_eq!(
            search(E, TOLERANCE, 0, 0, 0, &mut slot, &mut out),
            Ok(0),
            "no tuple at all"
        );
        assert_eq!(
            search((3, 1), (1, 1), 0, 0, 0, &mut slot, &mut out),
            Ok(0),
            "a and b zero"
        );
        assert_eq!(
            search((3, 1), (1, 1), 0, 3, 0, &mut slot, &mut out),
            Err(SearchError::OutFull),
            "at depth zero every a_0 within one of 3 matches"
        );
        assert_eq!(
            search(
                (3, 1),
                (1, 1),
                0,
                3,
                0,
                &mut slot,
                &mut [Candidate::default(); 20]
            ),
            Ok(12),
            "a_0 in {{2, 3}} with six non-zero b_0"
        );
    }
}

/// Property walk (ADR-0030): the recurrence through the slot agrees with a checked `i128`
/// reference over random small polynomials, flags included.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    fn reference(a: &Poly, b: &Poly, depth: u32) -> Option<Convergent> {
        let (mut p_prev, mut p, mut q_prev, mut q) = (1i128, poly_at(a, 0), 0i128, 1i128);
        for n in 1..=depth {
            let (an, bn) = (poly_at(a, n), poly_at(b, n));
            let np = an.checked_mul(p)?.checked_add(bn.checked_mul(p_prev)?)?;
            let nq = an.checked_mul(q)?.checked_add(bn.checked_mul(q_prev)?)?;
            p_prev = p;
            p = np;
            q_prev = q;
            q = nq;
        }
        Some(Convergent { p, q, depth })
    }

    #[test]
    fn the_slot_agrees_with_a_checked_reference() {
        let mut rng = Lcg::new(0xF12A_C710);
        let mut slot = ArithmeticScratchpadSlot::default();
        let mut flagged = 0u32;
        for _ in 0..2_000 {
            let coefficient = |rng: &mut Lcg| -> i64 {
                match rng.below(8) {
                    0 => i64::from(rng.next_i16()),
                    1 => i64::from(rng.next_i32()) << 20,
                    _ => i64::from(rng.next_u8() & 7).wrapping_sub(3),
                }
            };
            let a: Poly = [
                coefficient(&mut rng),
                coefficient(&mut rng),
                coefficient(&mut rng),
            ];
            let b: Poly = [
                coefficient(&mut rng),
                coefficient(&mut rng),
                coefficient(&mut rng),
            ];
            let depth = rng.below(65);
            let expected = reference(&a, &b, depth);
            let actual = convergent(&a, &b, depth, &mut slot);
            match expected {
                Some(c) => assert_eq!(actual, Ok(c)),
                None => {
                    assert_eq!(actual, Err(FractionError::Arithmetic(ERR_OVERFLOW)));
                    flagged = flagged.wrapping_add(1);
                    let deep = deepest(&a, &b, depth, &mut slot).unwrap();
                    assert!(deep.depth < depth);
                    assert_eq!(reference(&a, &b, deep.depth), Some(deep));
                    assert_eq!(reference(&a, &b, deep.depth.wrapping_add(1)), None);
                }
            }
        }
        assert!(flagged > 100, "flagged: {flagged}");
    }
}
