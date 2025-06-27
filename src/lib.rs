//! # ethereum-mysql
//!
//! Ethereum types wrapper for seamless SQLx database integration.
//!
//! This crate provides SQL-compatible wrappers for Ethereum types (`SqlAddress`, `SqlU256`),
//! specifically designed for the SQLx async SQL toolkit. It supports multiple databases
//! (MySQL, PostgreSQL, SQLite) through SQLx's feature system.
//!
//! ## Types
//!
//! - **`SqlAddress`**: Wraps `alloy::primitives::Address` for Ethereum addresses
//! - **`SqlU256`**: Wraps `alloy::primitives::U256` for 256-bit unsigned integers with full arithmetic support
//!
//! ## SQLx Integration
//!
//! This library is built specifically for [SQLx](https://github.com/launchbadge/sqlx),
//! the async SQL toolkit for Rust. SQLx is a pure Rust library that provides compile-time
//! checked queries and async database operations. This crate implements the necessary SQLx
//! traits (`Type`, `Encode`, `Decode`) to provide seamless database integration without
//! manual type conversion.
//!
//! **Note**: This library is designed specifically for SQLx. While it might work with other
//! Rust database libraries, it has only been tested and optimized for SQLx.
//!
//! ## Features
//!
//! - **Multi-database support**: MySQL, PostgreSQL, SQLite via SQLx
//! - **Zero-cost abstractions**: Wraps `alloy::primitives` types (`Address`, `U256`)
//! - **Complete type support**: `SqlAddress` for Ethereum addresses, `SqlU256` for large integers
//! - **Arithmetic operations**: Full arithmetic support for `SqlU256` (+, -, *, /, %, bitwise, etc.)
//! - **Type conversions**: Seamless conversion between `SqlU256` and Rust integer types
//! - **Compile-time macros**: Create addresses at compile time with `sqladdress!`
//! - **Constants**: Pre-defined useful constants like `SqlAddress::ZERO`, `SqlU256::ZERO`
//! - **Serde support**: Optional JSON serialization with serde
//! - **SQLx native**: Implements `sqlx::Type`, `sqlx::Encode`, and `sqlx::Decode`
//! - **Pure Rust**: No C dependencies, works with SQLx's pure Rust philosophy
//!
//! ## Database Features
//!
//! Enable the database you need (all via SQLx):
//! - `sqlx` - MySQL/MariaDB support via SQLx
//! - `serde` - JSON serialization support
//! - `default` - Enable all features above
//! - `full` - Enable all features above
//!
//!
//! ## Why SQLx?
//!
//! SQLx is chosen as the target framework because:
//! - **Pure Rust**: No C dependencies or ORM overhead
//! - **Compile-time safety**: SQL queries are checked at compile time
//! - **Async-first**: Built for modern async Rust applications
//! - **Multi-database**: Single API for multiple database backends
//! - **Performance**: Zero-cost abstractions and prepared statements
//!
//! ## Examples
//!
//! ### Basic Usage with SQLx
//!
//! ```rust
//! use ethereum_mysql::{SqlAddress, SqlU256, sqladdress};
//! use std::str::FromStr;
//!
//! // Address usage
//! let zero = SqlAddress::ZERO;
//! let addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
//! let addr2 = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
//!
//! // U256 usage with arithmetic
//! let balance = SqlU256::from_str("1000000000000000000").unwrap(); // 1 ETH in wei
//! let gas_price = SqlU256::from(20_000_000_000u64);                // 20 Gwei
//! let gas_limit = 21000u64;  // Direct primitive value
//!
//! // Arithmetic operations (NEW: Direct primitive operations!)
//! let total_cost = gas_price * gas_limit;  // No need for SqlU256::from(gas_limit)!
//! let doubled_balance = balance * 2;       // Direct multiplication with primitives
//! let remaining = balance - total_cost;    // Clean arithmetic
//!
//! // Both directions work: primitive * SqlU256 and SqlU256 * primitive
//! assert_eq!(2 * balance, balance * 2);
//!
//! // Type conversions
//! let small_amount = SqlU256::from(42u64);
//! let back_to_u64: u64 = small_amount.try_into().unwrap();
//! ```
//!
//! ### SQLx Database Usage
//!
//! ```rust,no_run
//! # use ethereum_mysql::{SqlAddress, SqlU256};
//! # use sqlx::Row;
//! # use std::str::FromStr;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // MySQL example with SQLx
//! # #[cfg(feature = "sqlx")]
//! # {
//! use sqlx::MySqlPool;
//!
//! let pool = MySqlPool::connect("mysql://user:pass@localhost/db").await?;
//!
//! // Insert both address and balance - no manual conversion needed
//! let user_address = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
//! let balance = SqlU256::from_str("1500000000000000000").unwrap(); // 1.5 ETH
//!
//! sqlx::query("INSERT INTO users (wallet_address, balance) VALUES (?, ?)")
//!     .bind(&user_address)  // SqlAddress implements sqlx::Encode
//!     .bind(&balance)       // SqlU256 implements sqlx::Encode  
//!     .execute(&pool)
//!     .await?;
//!
//! // Query with arithmetic operations
//! let rows = sqlx::query("SELECT wallet_address, balance FROM users")
//!     .fetch_all(&pool)
//!     .await?;
//!
//! for row in rows {
//!     let address: SqlAddress = row.get("wallet_address"); // SqlAddress implements sqlx::Decode
//!     let balance: SqlU256 = row.get("balance");           // SqlU256 implements sqlx::Decode
//!     println!("User {} has balance: {}", address, balance);
//! }
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! ### API Integration Example
//!
//! ```rust,no_run
//! # use ethereum_mysql::{SqlAddress, SqlU256};
//! # use std::str::FromStr;
//! // Perfect for API endpoints - no type conversion needed
//! async fn get_user_balance(wallet: SqlAddress) -> Result<SqlU256, String> {
//!     // Use wallet directly in SQLx queries without conversion
//!     // Also works with blockchain RPC calls via wallet.inner()
//!     Ok(SqlU256::from_str("12345000000000000000").map_err(|e| format!("{:?}", e))?) // 12.345 ETH
//! }
//!
//! // User input validation
//! let user_input = "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d";
//! match SqlAddress::from_str(user_input) {
//!     Ok(address) => {
//!         // Valid Ethereum address, can use directly
//!         println!("Valid address: {}", address);
//!     }
//!     Err(_) => {
//!         println!("Invalid Ethereum address format");
//!     }
//! }
//! ```
//! ```rust,no_run
//! # use ethereum_mysql::SqlAddress;
//! # use std::str::FromStr;
//! // Perfect for API endpoints - no type conversion needed
//! async fn get_user_balance(wallet: SqlAddress) -> Result<u64, Box<dyn std::error::Error>> {
//!     // Use wallet directly in SQLx queries without conversion
//!     // Also works with blockchain RPC calls via wallet.inner()
//!     Ok(12345)
//! }
//!
//! // User input validation
//! let user_input = "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d";
//! match SqlAddress::from_str(user_input) {
//!     Ok(address) => {
//!         // Valid Ethereum address, can use directly
//!         println!("Valid address: {}", address);
//!     }
//!     Err(_) => {
//!         println!("Invalid Ethereum address format");
//!     }
//! }
//! ```

#![warn(missing_docs)]

mod address;
mod macros;
mod sql_u256;

pub use address::{Address, SqlAddress};
pub use sql_u256::{SqlU256, U256};

#[cfg(feature = "sqlx")]
pub mod sqlx;

// Re-export alloy for macro usage
#[doc(hidden)]
pub use alloy;
