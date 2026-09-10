//! Arithmetic scratchpad slots: exact 128-bit integer and Q16.16 arithmetic with explicit
//! error flags, for the calculations the spiking substrate cannot do exactly (whitepaper
//! §5.2.31, §8.8; admitted by ADR-0016).
//!
//! A slot holds two 128-bit two's-complement operands, split into `u64` low and `i64` high
//! words so that the record stays `#[repr(C)]`, and the result. Overflow and division by zero
//! are reported in `error_flags`, not saturated: a scratchpad is not a state field, and a wrong
//! answer must be visible. The eight opcodes are Implemented; the sequencing of slots into an
//! expression is Specified.

#![no_std]

/// Opcodes.
pub const OP_NOP: u8 = 0;
pub const OP_ADD: u8 = 1;
pub const OP_SUB: u8 = 2;
pub const OP_MUL: u8 = 3;
pub const OP_DIV: u8 = 4;
pub const OP_REM: u8 = 5;
/// `(a × b) >> 16`: the product of two Q16.16 values in Q16.16.
pub const OP_MUL_Q16: u8 = 6;
/// `(a << 16) / b`: the quotient of two Q16.16 values in Q16.16.
pub const OP_DIV_Q16: u8 = 7;

/// `error_flags` bit: the exact result does not fit 128 bits.
pub const ERR_OVERFLOW: u16 = 0x0001;
/// `error_flags` bit: division or remainder by zero.
pub const ERR_DIVIDE_BY_ZERO: u16 = 0x0002;
/// `error_flags` bit: the opcode is not one of the above.
pub const ERR_UNKNOWN_OP: u16 = 0x0004;

/// 64-byte scratchpad slot (whitepaper §5.2.31).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct ArithmeticScratchpadSlot {
    pub operand_a_lo: u64,   // [0..8] Operand A, low 64 bits
    pub operand_a_hi: i64,   // [8..16] Operand A, high 64 bits (sign)
    pub operand_b_lo: u64,   // [16..24] Operand B, low 64 bits
    pub operand_b_hi: i64,   // [24..32] Operand B, high 64 bits (sign)
    pub result_lo: u64,      // [32..40] Result, low 64 bits
    pub result_hi: i64,      // [40..48] Result, high 64 bits (sign)
    pub error_flags: u16,    // [48..50] ERR_* bits from the last execute()
    pub opcode: u8,          // [50] OP_*
    pub operand_type: u8, // [51] 0 integer, 1 Q16.16 (informational; the opcode states the scaling)
    pub _reserved: [u8; 12], // [52..64] Reserved; MUST be zero
}

#[inline(always)]
const fn join(lo: u64, hi: i64) -> i128 {
    ((hi as i128) << 64) | lo as i128
}

#[inline(always)]
const fn split(v: i128) -> (u64, i64) {
    (v as u64, (v >> 64) as i64)
}

impl ArithmeticScratchpadSlot {
    pub const fn operand_a(&self) -> i128 {
        join(self.operand_a_lo, self.operand_a_hi)
    }

    pub const fn operand_b(&self) -> i128 {
        join(self.operand_b_lo, self.operand_b_hi)
    }

    pub const fn result(&self) -> i128 {
        join(self.result_lo, self.result_hi)
    }

    pub fn set_operands(&mut self, a: i128, b: i128) {
        (self.operand_a_lo, self.operand_a_hi) = split(a);
        (self.operand_b_lo, self.operand_b_hi) = split(b);
    }

    /// Executes the opcode on the operands. On success the result is stored, the flags are
    /// clear and `true` is returned; on any error the result is zero, the flag names the error
    /// and `false` is returned. `OP_NOP` succeeds with a zero result.
    pub fn execute(&mut self) -> bool {
        let a = self.operand_a();
        let b = self.operand_b();
        let outcome: Result<i128, u16> = match self.opcode {
            OP_NOP => Ok(0),
            OP_ADD => a.checked_add(b).ok_or(ERR_OVERFLOW),
            OP_SUB => a.checked_sub(b).ok_or(ERR_OVERFLOW),
            OP_MUL => a.checked_mul(b).ok_or(ERR_OVERFLOW),
            OP_DIV => Self::divide(a, b, i128::checked_div),
            OP_REM => Self::divide(a, b, |a, b| Some(a.wrapping_rem(b))),
            OP_MUL_Q16 => a.checked_mul(b).map(|p| p >> 16).ok_or(ERR_OVERFLOW),
            OP_DIV_Q16 => match a.checked_mul(1 << 16) {
                Some(scaled) => Self::divide(scaled, b, i128::checked_div),
                None => Err(ERR_OVERFLOW),
            },
            _ => Err(ERR_UNKNOWN_OP),
        };
        match outcome {
            Ok(v) => {
                (self.result_lo, self.result_hi) = split(v);
                self.error_flags = 0;
                true
            }
            Err(flag) => {
                (self.result_lo, self.result_hi) = (0, 0);
                self.error_flags = flag;
                false
            }
        }
    }

    /// Division and remainder share a zero divisor as a failure; `i128::MIN / -1` fails the
    /// division (`checked_div` reports `None`) and not the remainder, which is exactly 0 and
    /// fits, so `OP_REM` wraps after the zero check (ADR-0028).
    fn divide(a: i128, b: i128, op: fn(i128, i128) -> Option<i128>) -> Result<i128, u16> {
        if b == 0 {
            return Err(ERR_DIVIDE_BY_ZERO);
        }
        op(a, b).ok_or(ERR_OVERFLOW)
    }
}

const _: () = {
    assert!(core::mem::size_of::<ArithmeticScratchpadSlot>() == 64);
    assert!(core::mem::align_of::<ArithmeticScratchpadSlot>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: i128 = 1 << 16;

    fn run(op: u8, a: i128, b: i128) -> (bool, i128, u16) {
        let mut s = ArithmeticScratchpadSlot {
            opcode: op,
            ..Default::default()
        };
        s.set_operands(a, b);
        let ok = s.execute();
        (ok, s.result(), s.error_flags)
    }

    #[test]
    fn record_is_one_cache_line_and_default_is_a_nop() {
        assert_eq!(core::mem::size_of::<ArithmeticScratchpadSlot>(), 64);
        assert_eq!(core::mem::align_of::<ArithmeticScratchpadSlot>(), 64);
        let mut d = ArithmeticScratchpadSlot::default();
        assert!(d.execute());
        assert_eq!((d.result(), d.error_flags), (0, 0));
    }

    #[test]
    fn operands_round_trip_through_the_split_words_including_negatives_and_extremes() {
        for v in [
            0,
            1,
            -1,
            i64::MAX as i128 + 1,
            i64::MIN as i128 - 1,
            i128::MAX,
            i128::MIN,
        ] {
            let mut s = ArithmeticScratchpadSlot::default();
            let w = v.wrapping_neg();
            s.set_operands(v, w);
            assert_eq!(s.operand_a(), v);
            assert_eq!(s.operand_b(), w);
        }
    }

    #[test]
    fn integer_operations_are_exact() {
        assert_eq!(run(OP_ADD, 7, -9), (true, -2, 0));
        assert_eq!(
            run(OP_SUB, i64::MIN as i128, 1),
            (true, i64::MIN as i128 - 1, 0)
        );
        assert_eq!(
            run(OP_MUL, i64::MAX as i128, i64::MAX as i128),
            (true, (i64::MAX as i128) * (i64::MAX as i128), 0)
        );
        assert_eq!(
            run(OP_DIV, -7, 2),
            (true, -3, 0),
            "truncates toward zero, as i128 does"
        );
        assert_eq!(run(OP_REM, -7, 2), (true, -1, 0));
    }

    #[test]
    fn q16_operations_scale_correctly() {
        assert_eq!(
            run(OP_MUL_Q16, ONE + ONE / 2, 2 * ONE),
            (true, 3 * ONE, 0),
            "1.5 × 2.0 = 3.0"
        );
        assert_eq!(
            run(OP_DIV_Q16, 3 * ONE, 2 * ONE),
            (true, ONE + ONE / 2, 0),
            "3.0 / 2.0 = 1.5"
        );
        assert_eq!(
            run(OP_MUL_Q16, -ONE, 1),
            (true, -1, 0),
            "-1.0 × 2^-16 = -2^-16, floored"
        );
    }

    #[test]
    fn overflow_is_flagged_and_the_result_is_zero() {
        assert_eq!(run(OP_ADD, i128::MAX, 1), (false, 0, ERR_OVERFLOW));
        assert_eq!(run(OP_MUL, i128::MIN, -1), (false, 0, ERR_OVERFLOW));
        assert_eq!(run(OP_DIV, i128::MIN, -1), (false, 0, ERR_OVERFLOW));
        assert_eq!(
            run(OP_REM, i128::MIN, -1),
            (true, 0, 0),
            "the remainder is exact and fits"
        );
        assert_eq!(run(OP_REM, i128::MIN, 0), (false, 0, ERR_DIVIDE_BY_ZERO));
        assert_eq!(
            run(OP_DIV_Q16, i128::MAX, ONE),
            (false, 0, ERR_OVERFLOW),
            "the pre-shift overflows"
        );
    }

    #[test]
    fn division_by_zero_and_unknown_opcodes_are_flagged() {
        assert_eq!(run(OP_DIV, 1, 0), (false, 0, ERR_DIVIDE_BY_ZERO));
        assert_eq!(run(OP_REM, 1, 0), (false, 0, ERR_DIVIDE_BY_ZERO));
        assert_eq!(run(OP_DIV_Q16, ONE, 0), (false, 0, ERR_DIVIDE_BY_ZERO));
        assert_eq!(run(8, 1, 1), (false, 0, ERR_UNKNOWN_OP));
    }

    #[test]
    fn a_success_clears_the_flags_of_an_earlier_failure() {
        let mut s = ArithmeticScratchpadSlot {
            opcode: OP_DIV,
            ..Default::default()
        };
        s.set_operands(1, 0);
        assert!(!s.execute());
        s.set_operands(9, 3);
        assert!(s.execute());
        assert_eq!((s.result(), s.error_flags), (3, 0));
    }
}
