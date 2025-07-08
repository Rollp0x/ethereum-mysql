pub use alloy::primitives::FixedBytes;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A wrapper around `FixedBytes` that provides a SQL-compatible type for fixed-size byte arrays.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SqlFixedBytes<const BYTES: usize>(FixedBytes<BYTES>);
/// A type alias for a 32-byte fixed-size byte array, commonly used for hashes.
pub type SqlHash = SqlFixedBytes<32>;
/// A type alias for a 32-byte fixed-size byte array, commonly used for topic hashes.
pub type SqlTopicHash = SqlFixedBytes<32>;

impl<const BYTES: usize> SqlFixedBytes<BYTES> {
    /// Creates a new `SqlFixedBytes` from a `[u8; BYTES]`.
    pub fn new(bytes: [u8; BYTES]) -> Self {
        SqlFixedBytes(FixedBytes::new(bytes))
    }

    /// Returns a reference to the inner `FixedBytes<BYTES>`.
    pub fn inner(&self) -> &FixedBytes<BYTES> {
        &self.0
    }

    /// Creates a new `SqlFixedBytes` initialized to zero.
    pub const ZERO: Self = SqlFixedBytes(FixedBytes::ZERO);

    /// Creates a new `SqlFixedBytes` from a `FixedBytes<BYTES>`.
    pub const fn from_bytes(bytes: FixedBytes<BYTES>) -> Self {
        SqlFixedBytes(bytes)
    }

    /// Attempts to interpret the fixed bytes as an Ethereum address (last 20 bytes).
    /// Returns None if the length is not 32 or the prefix is not zeroed.
    pub fn to_address(&self) -> Option<crate::SqlAddress> {
        if BYTES == 32 {
            let bytes = self.0.as_slice();
            // Ethereum address is the last 20 bytes, prefix 12 bytes must be zero
            if bytes[..12].iter().all(|&b| b == 0) {
                let mut addr = [0u8; 20];
                addr.copy_from_slice(&bytes[12..]);
                return Some(crate::SqlAddress::new(addr));
            }
        }
        None
    }

    /// Interprets the fixed bytes as a U256 (no check, always possible for 32 bytes).
    pub fn to_u256(&self) -> crate::SqlU256 {
        use crate::SqlU256;
        use alloy::primitives::U256;
        if BYTES == 32 {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(self.0.as_slice());
            SqlU256::from(U256::from_be_bytes(arr))
        } else {
            // For non-32 bytes, fallback to zero
            SqlU256::ZERO
        }
    }
}

impl<const BYTES: usize> AsRef<FixedBytes<BYTES>> for SqlFixedBytes<BYTES> {
    fn as_ref(&self) -> &FixedBytes<BYTES> {
        &self.0
    }
}

impl<const BYTES: usize> Deref for SqlFixedBytes<BYTES> {
    type Target = FixedBytes<BYTES>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const BYTES: usize> From<FixedBytes<BYTES>> for SqlFixedBytes<BYTES> {
    fn from(bytes: FixedBytes<BYTES>) -> Self {
        SqlFixedBytes(bytes)
    }
}

impl<const BYTES: usize> From<SqlFixedBytes<BYTES>> for FixedBytes<BYTES> {
    fn from(sql_bytes: SqlFixedBytes<BYTES>) -> Self {
        sql_bytes.0
    }
}

impl<const BYTES: usize> FromStr for SqlFixedBytes<BYTES> {
    type Err = <FixedBytes<BYTES> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FixedBytes::<BYTES>::from_str(s).map(SqlFixedBytes)
    }
}

impl<const BYTES: usize> std::fmt::Display for SqlFixedBytes<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::FixedBytes;
    use std::str::FromStr;

    #[test]
    fn test_from_str_and_display() {
        let hex = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let val = SqlFixedBytes::<32>::from_str(hex).unwrap();
        assert_eq!(val.to_string(), hex.to_lowercase());
    }

    #[test]
    fn test_zero() {
        let zero = SqlFixedBytes::<32>::ZERO;
        assert_eq!(zero.inner().as_slice(), &[0u8; 32]);
    }

    #[test]
    fn test_as_ref_and_deref() {
        let hex = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let val = SqlFixedBytes::<32>::from_str(hex).unwrap();
        let as_ref: &FixedBytes<32> = val.as_ref();
        let deref: &FixedBytes<32> = &val;
        assert_eq!(as_ref, deref);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let hex = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let val = SqlFixedBytes::<32>::from_str(hex).unwrap();
        let json = serde_json::to_string(&val).unwrap();
        let de: SqlFixedBytes<32> = serde_json::from_str(&json).unwrap();
        assert_eq!(val, de);
    }

    #[test]
    fn test_fixed_bytes_5() {
        let hex = "0x68656c6c6f"; // "hello" in hex
        let val = SqlFixedBytes::<5>::from_str(hex).unwrap();
        assert_eq!(val.inner().as_slice(), b"hello");
        assert_eq!(val.to_string(), hex);
    }

    #[test]
    fn test_fixed_bytes_1() {
        let hex = "0x01";
        let val = SqlFixedBytes::<1>::from_str(hex).unwrap();
        assert_eq!(val.inner().as_slice(), &[1u8]);
        assert_eq!(val.to_string(), hex);
    }

    #[test]
    fn test_fixed_bytes_0() {
        let hex = "0x";
        let val = SqlFixedBytes::<0>::from_str(hex).unwrap();
        assert_eq!(val.inner().as_slice(), &[] as &[u8]);
        assert_eq!(val.to_string(), hex);
    }
}
