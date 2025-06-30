//! Arithmetic operations for SqlU256
//!
//! This module provides arithmetic operations (+, -, *, /, %) and other mathematical
//! operations for SqlU256, following Rust's standard library patterns.

use crate::{SqlU256, U256};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

/// Macro to implement binary arithmetic operations for all reference combinations
macro_rules! impl_binary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for SqlU256 {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                SqlU256::from(self.0 $op rhs.0)
            }
        }

        impl $trait<&SqlU256> for SqlU256 {
            type Output = Self;

            fn $method(self, rhs: &Self) -> Self::Output {
                SqlU256::from(self.0 $op rhs.0)
            }
        }

        impl $trait<SqlU256> for &SqlU256 {
            type Output = SqlU256;

            fn $method(self, rhs: SqlU256) -> Self::Output {
                SqlU256::from(self.0 $op rhs.0)
            }
        }

        impl $trait<&SqlU256> for &SqlU256 {
            type Output = SqlU256;

            fn $method(self, rhs: &SqlU256) -> Self::Output {
                SqlU256::from(self.0 $op rhs.0)
            }
        }
    };
}

/// Macro to implement unary operations
macro_rules! impl_unary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for SqlU256 {
            type Output = Self;

            fn $method(self) -> Self::Output {
                SqlU256::from($op self.0)
            }
        }
    };
}

/// Macro to implement shift operations
macro_rules! impl_shift_op {
    ($trait:ident, $method:ident, $op:tt, $rhs:ty) => {
        impl $trait<$rhs> for SqlU256 {
            type Output = Self;

            fn $method(self, rhs: $rhs) -> Self::Output {
                SqlU256::from(self.0 $op rhs)
            }
        }
    };
}

// Binary arithmetic operations
impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);
impl_binary_op!(Mul, mul, *);
impl_binary_op!(Div, div, /);
impl_binary_op!(Rem, rem, %);

// Bitwise operations (only value-to-value, no reference variants needed for these)
impl BitAnd for SqlU256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        SqlU256::from(self.0 & rhs.0)
    }
}

impl BitOr for SqlU256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        SqlU256::from(self.0 | rhs.0)
    }
}

impl BitXor for SqlU256 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        SqlU256::from(self.0 ^ rhs.0)
    }
}

// Unary operations
impl_unary_op!(Not, not, !);

// Shift operations
impl_shift_op!(Shl, shl, <<, usize);
impl_shift_op!(Shr, shr, >>, usize);

// Additional mathematical operations
impl SqlU256 {
    /// Returns the square of this value
    pub fn square(self) -> Self {
        self * self
    }

    /// Returns the power of this value raised to the given exponent
    pub fn pow(self, exp: usize) -> Self {
        SqlU256::from(self.0.pow(U256::from(exp)))
    }

    /// Returns the greatest common divisor of two values
    pub fn gcd(self, other: Self) -> Self {
        let mut a = self.0;
        let mut b = other.0;

        while !b.is_zero() {
            let temp = b;
            b = a % b;
            a = temp;
        }
        // Convert U256 back to SqlU256
        SqlU256::from(a)
    }

    /// Returns the least common multiple of two values
    pub fn lcm(self, other: Self) -> Self {
        if self.0.is_zero() || other.0.is_zero() {
            SqlU256::ZERO
        } else {
            let gcd = self.gcd(other);
            (self / gcd) * other
        }
    }

    /// Checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(SqlU256::from)
    }

    /// Checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(SqlU256::from)
    }

    /// Checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        self.0.checked_mul(rhs.0).map(SqlU256::from)
    }

    /// Checked division. Returns `None` if `rhs == 0`.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.0.is_zero() {
            None
        } else {
            Some(SqlU256::from(self.0 / rhs.0))
        }
    }

    /// Saturating addition. Clamps the result to `U256::MAX` if overflow occurred.
    pub fn saturating_add(self, rhs: Self) -> Self {
        SqlU256::from(self.0.saturating_add(rhs.0))
    }

    /// Saturating subtraction. Clamps the result to `0` if underflow occurred.
    pub fn saturating_sub(self, rhs: Self) -> Self {
        SqlU256::from(self.0.saturating_sub(rhs.0))
    }

    /// Saturating multiplication. Clamps the result to `U256::MAX` if overflow occurred.
    pub fn saturating_mul(self, rhs: Self) -> Self {
        SqlU256::from(self.0.saturating_mul(rhs.0))
    }

    /// Returns `true` if the value is zero
    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Returns the minimum of two values
    pub fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two values
    pub fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = SqlU256::from(100u64);
        let b = SqlU256::from(50u64);

        assert_eq!(a + b, SqlU256::from(150u64));
        assert_eq!(a - b, SqlU256::from(50u64));
        assert_eq!(a * b, SqlU256::from(5000u64));
        assert_eq!(a / b, SqlU256::from(2u64));
        assert_eq!(a % b, SqlU256::from(0u64));
    }

    #[test]
    fn test_arithmetic_with_references() {
        let a = SqlU256::from(100u64);
        let b = SqlU256::from(50u64);

        assert_eq!(&a + &b, SqlU256::from(150u64));
        assert_eq!(a + &b, SqlU256::from(150u64));
        assert_eq!(&a + b, SqlU256::from(150u64));
    }

    #[test]
    fn test_bitwise_operations() {
        let a = SqlU256::from(0b1100u64);
        let b = SqlU256::from(0b1010u64);

        assert_eq!(a & b, SqlU256::from(0b1000u64));
        assert_eq!(a | b, SqlU256::from(0b1110u64));
        assert_eq!(a ^ b, SqlU256::from(0b0110u64));
        assert_eq!(!SqlU256::from(0u64), SqlU256::from(!U256::ZERO));
    }

    #[test]
    fn test_shift_operations() {
        let a = SqlU256::from(8u64);

        assert_eq!(a << 1, SqlU256::from(16u64));
        assert_eq!(a >> 1, SqlU256::from(4u64));
        assert_eq!(a << 3, SqlU256::from(64u64));
        assert_eq!(a >> 2, SqlU256::from(2u64));
    }

    #[test]
    fn test_mathematical_operations() {
        let a = SqlU256::from(5u64);
        let _b = SqlU256::from(3u64);

        assert_eq!(a.square(), SqlU256::from(25u64));
        assert_eq!(a.pow(3), SqlU256::from(125u64));
        assert_eq!(
            SqlU256::from(12u64).gcd(SqlU256::from(8u64)),
            SqlU256::from(4u64)
        );
        assert_eq!(
            SqlU256::from(12u64).lcm(SqlU256::from(8u64)),
            SqlU256::from(24u64)
        );
    }

    #[test]
    fn test_checked_operations() {
        let a = SqlU256::from(100u64);
        let b = SqlU256::from(50u64);
        let zero = SqlU256::ZERO;

        assert_eq!(a.checked_add(b), Some(SqlU256::from(150u64)));
        assert_eq!(a.checked_sub(b), Some(SqlU256::from(50u64)));
        assert_eq!(a.checked_mul(b), Some(SqlU256::from(5000u64)));
        assert_eq!(a.checked_div(b), Some(SqlU256::from(2u64)));
        assert_eq!(a.checked_div(zero), None);

        // Test underflow
        assert_eq!(b.checked_sub(a), None);
    }

    #[test]
    fn test_saturating_operations() {
        let a = SqlU256::from(100u64);
        let b = SqlU256::from(150u64);

        assert_eq!(a.saturating_add(b), SqlU256::from(250u64));
        assert_eq!(a.saturating_sub(b), SqlU256::ZERO);
        assert_eq!(a.saturating_mul(b), SqlU256::from(15000u64));
    }

    #[test]
    fn test_utility_functions() {
        let a = SqlU256::from(100u64);
        let b = SqlU256::from(50u64);
        let zero = SqlU256::ZERO;

        assert!(!a.is_zero());
        assert!(zero.is_zero());
        assert_eq!(a.min(b), b);
        assert_eq!(a.max(b), a);
    }

    #[test]
    fn test_division_by_zero_panics() {
        let a = SqlU256::from(100u64);
        let zero = SqlU256::ZERO;

        // This should panic
        let result = std::panic::catch_unwind(|| {
            let _ = a / zero;
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_gcd_edge_cases() {
        let zero = SqlU256::ZERO;
        let five = SqlU256::from(5u64);

        assert_eq!(zero.gcd(five), five);
        assert_eq!(five.gcd(zero), five);
        assert_eq!(zero.gcd(zero), zero);
    }

    #[test]
    fn test_lcm_edge_cases() {
        let zero = SqlU256::ZERO;
        let five = SqlU256::from(5u64);

        assert_eq!(zero.lcm(five), zero);
        assert_eq!(five.lcm(zero), zero);
        assert_eq!(zero.lcm(zero), zero);
    }
}
