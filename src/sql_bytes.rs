pub use alloy::primitives::Bytes;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

/// A wrapper around `Bytes` to represent SQL byte arrays.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SqlBytes(Bytes);

impl SqlBytes {
    /// Creates a new `SqlBytes` initialized to zero.
    pub fn new() -> Self {
        SqlBytes(Bytes::new())
    }

    /// Returns a reference to the inner `Bytes`.
    pub fn inner(&self) -> &Bytes {
        &self.0
    }

    /// Converts the bytes to a SqlU256 using big-endian interpretation.
    ///
    /// If the length is less than 32, left-pads with zeros; if more, truncates high bytes (alloy behavior).
    pub fn to_u256(&self) -> crate::SqlU256 {
        crate::SqlU256::from(alloy::primitives::U256::from_be_slice(self.as_ref()))
    }
}

impl AsRef<Bytes> for SqlBytes {
    /// Returns a reference to the inner Bytes.
    fn as_ref(&self) -> &Bytes {
        &self.0
    }
}

impl Deref for SqlBytes {
    type Target = Bytes;

    /// Dereferences to the inner Bytes, allowing direct access to Bytes methods.
    ///
    /// This enables calling any method available on `alloy::primitives::Bytes`
    /// directly on a `SqlBytes` instance.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Bytes> for SqlBytes {
    fn from(bytes: Bytes) -> Self {
        SqlBytes(bytes)
    }
}

impl From<SqlBytes> for Bytes {
    fn from(sql_bytes: SqlBytes) -> Self {
        sql_bytes.0
    }
}

impl std::fmt::Display for SqlBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string().to_lowercase())
    }
}

impl FromStr for SqlBytes {
    type Err = <Bytes as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Bytes::from_str(s).map(SqlBytes)
    }
}

impl Default for SqlBytes {
    fn default() -> Self {
        SqlBytes::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let sql_bytes = SqlBytes::from_str("0x1234").unwrap();
        assert_eq!(format!("{}", sql_bytes), "0x1234");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let input = "0xa9059cbb00000000000000000000000042066a09b94e41e0263eee935177c2ecfff7fe11000000000000000000000000000000000000000000000002629f66e0c5300000";
        let val = SqlBytes::from_str(input).unwrap();
        let json = serde_json::to_string(&val).unwrap();
        let de: SqlBytes = serde_json::from_str(&json).unwrap();
        assert_eq!(val, de);
    }
}
