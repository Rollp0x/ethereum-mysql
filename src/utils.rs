//! Utilities for parsing and formatting SqlU256 with decimals (e.g. for ERC20/ETH amounts).

use crate::SqlU256;
use alloy::primitives::{
    utils::{format_units, parse_units, UnitsError},
    U256,
};

/// Parses a decimal string (e.g. "1.23") into a SqlU256, given the number of decimals.
///
/// # Examples
/// ```
/// use ethereum_mysql::utils::{parse_suint, format_suint};
/// let v = parse_suint("1.23", 6).unwrap();
/// assert_eq!(format_suint(v, 6).unwrap(), "1.230000");
/// ```
pub fn parse_suint(s: &str, decimals: u8) -> Result<SqlU256, UnitsError> {
    parse_units(s, decimals).map(|v| {
        let value: U256 = v.into();
        SqlU256::from(value)
    })
}

/// Formats a SqlU256 as a decimal string with the given number of decimals.
///
/// # Examples
/// ```
/// use ethereum_mysql::utils::{parse_suint, format_suint};
/// let v = parse_suint("1.23", 6).unwrap();
/// assert_eq!(format_suint(v, 6).unwrap(), "1.230000");
/// ```
pub fn format_suint(value: SqlU256, decimals: u8) -> Result<String, UnitsError> {
    format_units(value.into_inner(), decimals)
}

/// Parses a decimal string as Ether (18 decimals).
pub fn parse_sether(s: &str) -> Result<SqlU256, UnitsError> {
    parse_suint(s, 18)
}

/// Formats a SqlU256 as Ether (18 decimals).
pub fn format_sether(value: SqlU256) -> Result<String, UnitsError> {
    format_suint(value, 18)
}
