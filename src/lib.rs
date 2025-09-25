#![cfg_attr(docsrs, feature(doc_cfg))]
//! # ethereum-mysql
//!
//! Type-safe, ergonomic wrappers for Ethereum types with seamless SQLx database integration.
//!
//! This crate provides SQL-compatible wrappers for common Ethereum types (`Address`, `U256`, `FixedBytes`, `Bytes`),
//! designed for use with the async SQLx toolkit and relational databases (MySQL, PostgreSQL, SQLite).
//!
//! ## Supported Types
//!
//! - **SqlAddress**: Type-safe wrapper for `alloy::primitives::Address` (Ethereum address)
//! - **SqlU256**: Wrapper for `alloy::primitives::U256` (256-bit unsigned integer) with full arithmetic and conversion support
//! - **`SqlFixedBytes<N>`**: Generic wrapper for fixed-size byte arrays (e.g. hashes, topics)
//!   - **SqlHash**/**SqlTopicHash**: Type aliases for `SqlFixedBytes<32>` (commonly used for hashes/topics)
//! - **SqlBytes**: Wrapper for dynamic-length byte arrays
//!
//! ## Design Highlights
//!
//! - **String-based storage only**: All types are stored as lowercase hex strings (with `0x` prefix) in the database for maximum compatibility and easy inspection.
//! - **Type safety**: Compile-time and runtime validation for all Ethereum types, eliminating manual string parsing and validation in business logic.
//! - **API ergonomics**: Direct arithmetic, comparison, and conversion with Rust primitives for U256, and compile-time address macros for zero-cost, safe usage.
//! - **No binary mode**: Binary column support and related feature flags have been removed for simplicity and reliability.
//! - **Minimal, focused API**: Only the most practical and widely-used Ethereum types and operations are supported, with optional serde integration.
//!
//! ## SQLx Integration
//!
//! This crate implements the necessary SQLx traits (`Type`, `Encode`, `Decode`) for all wrappers, enabling direct use in queries and result sets without manual conversion.
//!
//! - **Multi-database support**: MySQL, PostgreSQL, SQLite (via SQLx)
//! - **Serde support**: Optional JSON serialization for all wrappers (enable the `serde` feature)
//! - **Constants**: Pre-defined constants like `SqlAddress::ZERO`, `SqlU256::ZERO`
//! - **Compile-time macros**: Create addresses at compile time with `sqladdress!`
//!
//! ## Recommended Database Column Types
//!
//! | Type               | Recommended Column Type |
//! |--------------------|------------------------|
//! | SqlAddress         | VARCHAR(42)            |
//! | SqlU256            | VARCHAR(66)            |
//! | `SqlFixedBytes<N>` | VARCHAR(2+2*N)         |
//! | SqlBytes           | TEXT                   |
//!
//! For PostgreSQL, use `TEXT` for all string types. For MySQL/SQLite, use `VARCHAR` as above.
//!
//! ## Example Usage
//!
//! ```rust
//! use ethereum_mysql::{SqlAddress, SqlU256, SqlHash, sqladdress};
//! use std::str::FromStr;
//!
//! // Address usage
//! let zero = SqlAddress::ZERO;
//! let addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
//! let addr2 = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
//!
//! // U256 usage with arithmetic
//! let balance = SqlU256::from_str("1000000000000000000").unwrap(); // 1 ETH in wei
//! let doubled = balance * 2;
//!
//! // Hash usage
//! let tx_hash = SqlHash::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").unwrap();
//! println!("Tx hash: {}", tx_hash);
//! ```
//!
//! ## Migration Notes
//!
//! - All binary mode and related feature flags have been removed. Only string-based storage is supported.
//! - Update your database schema to use string (hex) columns for all Ethereum types.
//! - See README for more details and migration guidance.

#![warn(missing_docs)]

mod macros;
mod sql_address;
mod sql_bytes;
mod sql_fixed_bytes;
mod sql_uint;

pub mod utils;

pub use sql_address::{Address, SqlAddress};
pub use sql_bytes::{Bytes, SqlBytes};
pub use sql_fixed_bytes::{FixedBytes, SqlFixedBytes, SqlHash, SqlTopicHash};
pub use sql_uint::{SqlU256, SqlUint, U256};

#[cfg(feature = "sqlx")]
pub mod sqlx;

// Re-export alloy for macro usage
#[doc(hidden)]
pub use alloy;
