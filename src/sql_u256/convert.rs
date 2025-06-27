//! Type conversion implementations for SqlU256
//!
//! This module provides conversions from various integer types to SqlU256,
//! following Rust's standard library patterns.

use super::SqlU256;
use alloy::primitives::U256;

// Macro to reduce boilerplate for unsigned integer conversions
macro_rules! impl_from_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SqlU256 {
                fn from(value: $t) -> Self {
                    SqlU256(U256::from(value))
                }
            }
        )*
    };
}

// Implement From for all unsigned integer types
impl_from_unsigned!(u8, u16, u32, u64, u128);

impl From<usize> for SqlU256 {
    fn from(value: usize) -> Self {
        SqlU256(U256::from(value as u64))
    }
}

// Macro to reduce boilerplate for signed integer TryFrom conversions
macro_rules! impl_try_from_signed {
    ($($t:ty => $cast:ty),*) => {
        $(
            impl TryFrom<$t> for SqlU256 {
                type Error = &'static str;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err("Cannot convert negative value to SqlU256")
                    } else {
                        Ok(SqlU256(U256::from(value as $cast)))
                    }
                }
            }
        )*
    };
}

// Implement TryFrom for all signed integer types
impl_try_from_signed!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128);

impl TryFrom<isize> for SqlU256 {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 {
            Err("Cannot convert negative value to SqlU256")
        } else {
            Ok(SqlU256(U256::from(value as u64)))
        }
    }
}

// Macro to reduce boilerplate for SqlU256 to smaller integer TryFrom conversions
macro_rules! impl_try_from_sql_u256 {
    ($($t:ty),*) => {
        $(
            impl TryFrom<SqlU256> for $t {
                type Error = &'static str;

                fn try_from(value: SqlU256) -> Result<Self, Self::Error> {
                    if value.0 > U256::from(<$t>::MAX) {
                        Err(concat!("SqlU256 value too large for ", stringify!($t)))
                    } else {
                        Ok(value.0.to::<$t>())
                    }
                }
            }
        )*
    };
}

// Implement TryFrom for all smaller integer types
impl_try_from_sql_u256!(u8, u16, u32, u64, u128);

impl TryFrom<SqlU256> for usize {
    type Error = &'static str;

    fn try_from(value: SqlU256) -> Result<Self, Self::Error> {
        if value.0 > U256::from(u64::MAX) {
            Err("SqlU256 value too large for usize")
        } else {
            Ok(value.0.to::<u64>() as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_unsigned_integers() {
        assert_eq!(SqlU256::from(42u8), SqlU256(U256::from(42)));
        assert_eq!(SqlU256::from(1000u16), SqlU256(U256::from(1000)));
        assert_eq!(SqlU256::from(100000u32), SqlU256(U256::from(100000)));
        assert_eq!(
            SqlU256::from(10000000000u64),
            SqlU256(U256::from(10000000000u64))
        );
        assert_eq!(SqlU256::from(u128::MAX), SqlU256(U256::from(u128::MAX)));
        assert_eq!(SqlU256::from(123usize), SqlU256(U256::from(123)));
    }

    #[test]
    fn test_try_from_positive_signed_integers() {
        assert_eq!(SqlU256::try_from(42i8).unwrap(), SqlU256(U256::from(42)));
        assert_eq!(
            SqlU256::try_from(1000i16).unwrap(),
            SqlU256(U256::from(1000))
        );
        assert_eq!(
            SqlU256::try_from(100000i32).unwrap(),
            SqlU256(U256::from(100000))
        );
        assert_eq!(
            SqlU256::try_from(10000000000i64).unwrap(),
            SqlU256(U256::from(10000000000u64))
        );
        assert_eq!(
            SqlU256::try_from(i128::MAX).unwrap(),
            SqlU256(U256::from(i128::MAX as u128))
        );
        assert_eq!(
            SqlU256::try_from(123isize).unwrap(),
            SqlU256(U256::from(123))
        );
    }

    #[test]
    fn test_try_from_negative_signed_integers() {
        assert!(SqlU256::try_from(-1i8).is_err());
        assert!(SqlU256::try_from(-1000i16).is_err());
        assert!(SqlU256::try_from(-100000i32).is_err());
        assert!(SqlU256::try_from(-10000000000i64).is_err());
        assert!(SqlU256::try_from(i128::MIN).is_err());
        assert!(SqlU256::try_from(-123isize).is_err());
    }

    #[test]
    fn test_try_from_sql_u256_to_smaller_types() {
        // Test successful conversions
        let small_value = SqlU256::from(42u8);
        assert_eq!(u8::try_from(small_value).unwrap(), 42u8);
        assert_eq!(u16::try_from(small_value).unwrap(), 42u16);
        assert_eq!(u32::try_from(small_value).unwrap(), 42u32);
        assert_eq!(u64::try_from(small_value).unwrap(), 42u64);
        assert_eq!(u128::try_from(small_value).unwrap(), 42u128);
        assert_eq!(usize::try_from(small_value).unwrap(), 42usize);

        // Test overflow cases
        let large_value = SqlU256::from(u64::MAX);
        assert!(u8::try_from(large_value).is_err());
        assert!(u16::try_from(large_value).is_err());
        assert!(u32::try_from(large_value).is_err());
        assert_eq!(u64::try_from(large_value).unwrap(), u64::MAX);

        // Test very large value
        let very_large = SqlU256(U256::from(u128::MAX));
        assert!(u64::try_from(very_large).is_err());
        assert_eq!(u128::try_from(very_large).unwrap(), u128::MAX);
    }

    #[test]
    fn test_conversion_chain() {
        // Test that we can chain conversions naturally
        let value: SqlU256 = 42u64.into();
        assert_eq!(value, SqlU256(U256::from(42)));

        let back: u64 = value.try_into().unwrap();
        assert_eq!(back, 42u64);
    }
}
