//! Operations between SqlU256 and primitive types
//!
//! This module provides convenient arithmetic operations between SqlU256 and Rust's
//! primitive integer types, supporting both directions (e.g., `value * 2` and `2 * value`).
//! This eliminates the need for explicit conversions like `value * SqlU256::from(2)`.

use super::SqlU256;
use alloy::primitives::U256;
use std::ops::{Add, Div, Mul, Rem, Sub};

/// Macro to implement arithmetic operations between SqlU256 and primitive types in both directions
macro_rules! impl_primitive_ops {
    ($prim_type:ty) => {
        // SqlU256 op primitive (e.g., value * 2)
        impl Add<$prim_type> for SqlU256 {
            type Output = Self;

            fn add(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 + U256::from(rhs))
            }
        }

        impl Sub<$prim_type> for SqlU256 {
            type Output = Self;

            fn sub(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 - U256::from(rhs))
            }
        }

        impl Mul<$prim_type> for SqlU256 {
            type Output = Self;

            fn mul(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 * U256::from(rhs))
            }
        }

        impl Div<$prim_type> for SqlU256 {
            type Output = Self;

            fn div(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 / U256::from(rhs))
            }
        }

        impl Rem<$prim_type> for SqlU256 {
            type Output = Self;

            fn rem(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 % U256::from(rhs))
            }
        }

        // primitive op SqlU256 (e.g., 2 * value)
        impl Add<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn add(self, rhs: SqlU256) -> Self::Output {
                SqlU256(U256::from(self) + rhs.0)
            }
        }

        impl Sub<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn sub(self, rhs: SqlU256) -> Self::Output {
                SqlU256(U256::from(self) - rhs.0)
            }
        }

        impl Mul<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn mul(self, rhs: SqlU256) -> Self::Output {
                SqlU256(U256::from(self) * rhs.0)
            }
        }

        impl Div<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn div(self, rhs: SqlU256) -> Self::Output {
                SqlU256(U256::from(self) / rhs.0)
            }
        }

        impl Rem<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn rem(self, rhs: SqlU256) -> Self::Output {
                SqlU256(U256::from(self) % rhs.0)
            }
        }

        // Reference variants for SqlU256 op &primitive
        impl Add<&$prim_type> for SqlU256 {
            type Output = Self;

            fn add(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 + U256::from(*rhs))
            }
        }

        impl Sub<&$prim_type> for SqlU256 {
            type Output = Self;

            fn sub(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 - U256::from(*rhs))
            }
        }

        impl Mul<&$prim_type> for SqlU256 {
            type Output = Self;

            fn mul(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 * U256::from(*rhs))
            }
        }

        impl Div<&$prim_type> for SqlU256 {
            type Output = Self;

            fn div(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 / U256::from(*rhs))
            }
        }

        impl Rem<&$prim_type> for SqlU256 {
            type Output = Self;

            fn rem(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 % U256::from(*rhs))
            }
        }

        // Reference variants for &SqlU256 op primitive
        impl Add<$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn add(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 + U256::from(rhs))
            }
        }

        impl Sub<$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn sub(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 - U256::from(rhs))
            }
        }

        impl Mul<$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn mul(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 * U256::from(rhs))
            }
        }

        impl Div<$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn div(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 / U256::from(rhs))
            }
        }

        impl Rem<$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn rem(self, rhs: $prim_type) -> Self::Output {
                SqlU256(self.0 % U256::from(rhs))
            }
        }

        // Reference variants for &SqlU256 op &primitive
        impl Add<&$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn add(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 + U256::from(*rhs))
            }
        }

        impl Sub<&$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn sub(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 - U256::from(*rhs))
            }
        }

        impl Mul<&$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn mul(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 * U256::from(*rhs))
            }
        }

        impl Div<&$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn div(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 / U256::from(*rhs))
            }
        }

        impl Rem<&$prim_type> for &SqlU256 {
            type Output = SqlU256;

            fn rem(self, rhs: &$prim_type) -> Self::Output {
                SqlU256(self.0 % U256::from(*rhs))
            }
        }
    };
}

// Implement operations for common integer types
impl_primitive_ops!(u8);
impl_primitive_ops!(u16);
impl_primitive_ops!(u32);
impl_primitive_ops!(u64);
impl_primitive_ops!(u128);
impl_primitive_ops!(usize);

// For signed integers, we only implement the safe operations
// (addition, multiplication) to avoid potential underflow issues
macro_rules! impl_signed_ops {
    ($prim_type:ty) => {
        // SqlU256 op signed primitive (only safe operations)
        impl Add<$prim_type> for SqlU256 {
            type Output = Self;

            fn add(self, rhs: $prim_type) -> Self::Output {
                if rhs >= 0 {
                    SqlU256(self.0 + U256::from(rhs as u64))
                } else {
                    SqlU256(self.0 - U256::from((-rhs) as u64))
                }
            }
        }

        impl Sub<$prim_type> for SqlU256 {
            type Output = Self;

            fn sub(self, rhs: $prim_type) -> Self::Output {
                if rhs >= 0 {
                    SqlU256(self.0 - U256::from(rhs as u64))
                } else {
                    SqlU256(self.0 + U256::from((-rhs) as u64))
                }
            }
        }

        impl Mul<$prim_type> for SqlU256 {
            type Output = Self;

            fn mul(self, rhs: $prim_type) -> Self::Output {
                if rhs >= 0 {
                    SqlU256(self.0 * U256::from(rhs as u64))
                } else {
                    // For negative multiplication, we'd need to handle the sign,
                    // but since SqlU256 is unsigned, we panic to avoid confusion
                    panic!("Cannot multiply SqlU256 by negative number")
                }
            }
        }

        impl Div<$prim_type> for SqlU256 {
            type Output = Self;

            fn div(self, rhs: $prim_type) -> Self::Output {
                if rhs > 0 {
                    SqlU256(self.0 / U256::from(rhs as u64))
                } else if rhs == 0 {
                    panic!("attempt to divide by zero")
                } else {
                    panic!("Cannot divide SqlU256 by negative number")
                }
            }
        }

        // signed primitive op SqlU256 (only safe operations)
        impl Add<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn add(self, rhs: SqlU256) -> Self::Output {
                if self >= 0 {
                    SqlU256(U256::from(self as u64) + rhs.0)
                } else {
                    if rhs.0 >= U256::from((-self) as u64) {
                        SqlU256(rhs.0 - U256::from((-self) as u64))
                    } else {
                        panic!("Cannot subtract larger value from smaller in unsigned context")
                    }
                }
            }
        }

        impl Mul<SqlU256> for $prim_type {
            type Output = SqlU256;

            fn mul(self, rhs: SqlU256) -> Self::Output {
                if self >= 0 {
                    SqlU256(U256::from(self as u64) * rhs.0)
                } else {
                    panic!("Cannot multiply negative number by SqlU256")
                }
            }
        }
    };
}

impl_signed_ops!(i8);
impl_signed_ops!(i16);
impl_signed_ops!(i32);
impl_signed_ops!(i64);
impl_signed_ops!(i128);
impl_signed_ops!(isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_primitive_operations() {
        let value = SqlU256::from(100u64);

        // SqlU256 op primitive
        assert_eq!(value + 50u64, SqlU256::from(150u64));
        assert_eq!(value - 30u64, SqlU256::from(70u64));
        assert_eq!(value * 2u64, SqlU256::from(200u64));
        assert_eq!(value / 2u64, SqlU256::from(50u64));
        assert_eq!(value % 30u64, SqlU256::from(10u64));

        // primitive op SqlU256
        assert_eq!(50u64 + value, SqlU256::from(150u64));
        assert_eq!(200u64 - value, SqlU256::from(100u64));
        assert_eq!(2u64 * value, SqlU256::from(200u64));
        assert_eq!(1000u64 / value, SqlU256::from(10u64));
        assert_eq!(250u64 % value, SqlU256::from(50u64));
    }

    #[test]
    fn test_different_unsigned_types() {
        let value = SqlU256::from(100u64);

        // Test with different unsigned types
        assert_eq!(value * 2u8, SqlU256::from(200u64));
        assert_eq!(value * 2u16, SqlU256::from(200u64));
        assert_eq!(value * 2u32, SqlU256::from(200u64));
        assert_eq!(value * 2u128, SqlU256::from(200u64));
        assert_eq!(value * 2usize, SqlU256::from(200u64));

        assert_eq!(2u8 * value, SqlU256::from(200u64));
        assert_eq!(2u16 * value, SqlU256::from(200u64));
        assert_eq!(2u32 * value, SqlU256::from(200u64));
        assert_eq!(2u128 * value, SqlU256::from(200u64));
        assert_eq!(2usize * value, SqlU256::from(200u64));
    }

    #[test]
    fn test_reference_operations() {
        let value = SqlU256::from(100u64);
        let multiplier = 2u64;

        // Test reference variants
        assert_eq!(value * &multiplier, SqlU256::from(200u64));
        assert_eq!(&value * multiplier, SqlU256::from(200u64));
        assert_eq!(&value * &multiplier, SqlU256::from(200u64));
    }

    #[test]
    fn test_signed_positive_operations() {
        let value = SqlU256::from(100u64);

        // Positive signed integers should work
        assert_eq!(value + 50i64, SqlU256::from(150u64));
        assert_eq!(value * 2i64, SqlU256::from(200u64));
        assert_eq!(50i64 + value, SqlU256::from(150u64));
        assert_eq!(2i64 * value, SqlU256::from(200u64));
    }

    #[test]
    fn test_signed_negative_addition() {
        let value = SqlU256::from(100u64);

        // Negative addition (should work like subtraction)
        assert_eq!(value + (-30i64), SqlU256::from(70u64));

        // Negative + positive SqlU256
        assert_eq!((-30i64) + value, SqlU256::from(70u64));
    }

    #[test]
    #[should_panic(expected = "Cannot multiply SqlU256 by negative number")]
    fn test_signed_negative_multiplication_panic() {
        let value = SqlU256::from(100u64);
        let _ = value * (-2i64);
    }

    #[test]
    #[should_panic(expected = "Cannot multiply negative number by SqlU256")]
    fn test_negative_multiply_sqlu256_panic() {
        let value = SqlU256::from(100u64);
        let _ = (-2i64) * value;
    }

    #[test]
    #[should_panic(expected = "Cannot subtract larger value from smaller in unsigned context")]
    fn test_negative_addition_underflow_panic() {
        let value = SqlU256::from(50u64);
        let _ = (-100i64) + value; // Would result in negative, should panic
    }

    #[test]
    fn test_common_ethereum_scenarios() {
        // Common Ethereum scenarios
        let balance = SqlU256::from(1_000_000_000_000_000_000u64); // 1 ETH in wei
        let gas_price = SqlU256::from(20_000_000_000u64); // 20 Gwei

        // Double the balance
        let doubled_balance = balance * 2;
        assert_eq!(doubled_balance, SqlU256::from(2_000_000_000_000_000_000u64));

        // Calculate transaction cost (gas price * gas limit)
        let gas_limit = 21000u64;
        let tx_cost = gas_price * gas_limit;
        assert_eq!(tx_cost, SqlU256::from(420_000_000_000_000u64));

        // Calculate remaining balance
        let remaining = balance - tx_cost;
        assert_eq!(remaining, SqlU256::from(999_580_000_000_000_000u64));

        // Percentage calculations (e.g., 5% fee)
        let fee_percentage = 5u64;
        let fee = balance * fee_percentage / 100u64;
        assert_eq!(fee, SqlU256::from(50_000_000_000_000_000u64));
    }
}
