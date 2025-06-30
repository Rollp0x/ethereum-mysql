//! This module is only available when the `sqlx` feature is enabled.
//! Support for the [`sqlx`](https://crates.io/crates/sqlx) crate.
//!
//! This implementation encodes and decodes Ethereum types to and from string (hex/decimal) format.
//!
//! **Note:** The recommended database column type is `VARCHAR(42)` or `CHAR(42)` (MySQL/SQLite) for addresses,
//! and `VARCHAR(66)` or `TEXT` for U256 values. This is suitable for cross-language and legacy database integration.
//! 
//! **U256 string encoding/decoding notes:**
//! - When writing to the database, U256 is always encoded as a lowercase hex string with `0x` prefix (e.g. `0x1234...`).
//! - When reading from the database, both `0x`-prefixed hex strings and pure decimal strings are supported.
//! - For best compatibility and predictable sorting/comparison, always store U256 as hex strings in the database.
//! - If you store decimal strings, reading is supported, but database-level comparison/sorting may not match Rust-side logic.
//! 
#![cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]

use std::str::FromStr;
use thiserror::Error;

use sqlx_core::{
    database::Database,
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    types::Type,
};

/// Error type for decoding failures when converting database values to Ethereum types.
///
/// This is used when a value from the database cannot be represented in the target type,
/// such as when a byte not a valid Ethereum address or U256 string.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Returned when the database value is not a valid Ethereum address string.
    #[error("Address decode error: source {0}")]
    AddressDecodeError(String),

    /// Returned when the database value is not a valid Uint string.
    #[error("Uint decode error: source {0}")]
    UintDecodeError(String),

    /// Returned when the database value is not a valid FixedBytes string.
    #[error("FixedBytes decode error: source {0}")]
    FixedBytesDecodeError(String),

    /// Returned when the database value is not a valid Bytes string.
    #[error("Bytes decode error: source {0}")]
    BytesDecodeError(String),
}

use crate::{SqlAddress, SqlUint,SqlFixedBytes,SqlBytes};

// for SqlAddress
impl<DB: Database> Type<DB> for SqlAddress
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

impl<'a, DB: Database> Encode<'a, DB> for SqlAddress
where
    String: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.to_string().to_lowercase().encode_by_ref(buf)
    }
}

impl<'a, DB: Database> Decode<'a, DB> for SqlAddress
where
    String: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let s = String::decode(value)?;
        SqlAddress::from_str(&s)
            .map_err(|_| DecodeError::AddressDecodeError(s).into())
    }
}

// for SqlUint
impl<const BITS: usize, const LIMBS: usize, DB: Database> Type<DB> for SqlUint<BITS, LIMBS>
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

impl<'a, const BITS: usize, const LIMBS: usize, DB: Database> Encode<'a, DB> for SqlUint<BITS, LIMBS>
where
    String: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.to_string().to_lowercase().encode_by_ref(buf)
    }
}

impl<'a, const BITS: usize, const LIMBS: usize, DB: Database> Decode<'a, DB> for SqlUint<BITS, LIMBS>
where
    String: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let s = String::decode(value)?;
        SqlUint::<BITS, LIMBS>::from_str(&s)
            .map_err(|_| DecodeError::UintDecodeError(s.to_string()).into())
    }
}

/// for SqlFixedBytes<32>
impl<DB: Database> Type<DB> for SqlFixedBytes<32>
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}
impl<'a, DB: Database> Encode<'a, DB> for SqlFixedBytes<32>
where
    String: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.to_string().to_lowercase().encode_by_ref(buf)
    }
}
impl<'a, DB: Database> Decode<'a, DB> for SqlFixedBytes<32>
where
    String: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let s = String::decode(value)?;
        SqlFixedBytes::<32>::from_str(&s)
            .map_err(|_| DecodeError::FixedBytesDecodeError(s).into())
    }
}

// for SqlBytes
impl<DB: Database> Type<DB> for SqlBytes
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

impl<'a, DB: Database> Encode<'a, DB> for SqlBytes
where
    String: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.to_string().to_lowercase().encode_by_ref(buf)
    }
}

impl<'a, DB: Database> Decode<'a, DB> for SqlBytes
where
    String: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let s = String::decode(value)?;
        SqlBytes::from_str(&s)
            .map_err(|e| DecodeError::BytesDecodeError(e.to_string()).into())
    }
}