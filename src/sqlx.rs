//! This module is only available when the `sqlx` feature is enabled.
//! Support for the [`sqlx`](https://crates.io/crates/sqlx) crate.
//!
//! Currently only encodes to/from a big-endian byte array.
//!
//! **Note:** The database column type must be `BINARY(20)` (MySQL/SQLite), `BYTEA` (Postgres), or
//! equivalent binary type for correct Address roundtrip.
#![cfg_attr(docsrs, doc(cfg(feature = "sqlx")))]

use std::boxed::Box;
use std::vec::Vec;

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
/// such as when a byte array is too large to fit into a U256 or Address.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Returned when the database value is too large to fit into the target type (e.g., U256 or Address).
    #[error("Value too large for target type")]
    Overflow,
}

use crate::{Address, SqlAddress, SqlU256, U256};

// for SqlAddress
impl<DB: Database> Type<DB> for SqlAddress
where
    Vec<u8>: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Vec<u8> as Type<DB>>::compatible(ty)
    }
}

impl<'a, DB: Database> Encode<'a, DB> for SqlAddress
where
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        Vec::from(self.as_slice()).encode_by_ref(buf)
    }
}

impl<'a, DB: Database> Decode<'a, DB> for SqlAddress
where
    Vec<u8>: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let bytes = Vec::<u8>::decode(value)?;
        let addr = Address::try_from(bytes.as_slice()).map_err(|e| Box::new(e) as BoxDynError)?;
        Ok(SqlAddress::from(addr))
    }
}

// for SqlU256
impl<DB: Database> Type<DB> for SqlU256
where
    Vec<u8>: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Vec<u8> as Type<DB>>::compatible(ty)
    }
}

impl<'a, DB: Database> Encode<'a, DB> for SqlU256
where
    Vec<u8>: Encode<'a, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'a>,
    ) -> Result<IsNull, BoxDynError> {
        self.to_be_bytes_vec().encode_by_ref(buf)
    }
}

impl<'a, DB: Database> Decode<'a, DB> for SqlU256
where
    Vec<u8>: Decode<'a, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        let bytes = Vec::<u8>::decode(value)?;
        U256::try_from_be_slice(bytes.as_slice())
            .ok_or_else(|| DecodeError::Overflow.into())
            .map(SqlU256::from)
    }
}
