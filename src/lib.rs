//! # ethereum-mysql
//!
//! Ethereum Address wrapper for seamless SQLx database integration.
//!
//! This crate provides SQL-compatible wrappers for Ethereum types, specifically designed
//! for the SQLx async SQL toolkit. It supports multiple databases (MySQL, PostgreSQL, SQLite)
//! through SQLx's feature system.
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
//! - **Zero-cost abstractions**: Wraps `alloy::primitives` types
//! - **Compile-time macros**: Create addresses at compile time with `sqladdress!`
//! - **Constants**: Pre-defined useful constants like `SqlAddress::ZERO`
//! - **Serde support**: Optional JSON serialization with serde
//! - **SQLx native**: Implements `sqlx::Type`, `sqlx::Encode`, and `sqlx::Decode`
//! - **Pure Rust**: No C dependencies, works with SQLx's pure Rust philosophy
//!
//! ## Database Features
//!
//! Enable the database you need (all via SQLx):
//! - `mysql` - MySQL/MariaDB support via SQLx
//! - `postgres` - PostgreSQL support via SQLx  
//! - `sqlite` - SQLite support via SQLx
//! - `serde` - JSON serialization support
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
//! use ethereum_mysql::{SqlAddress, sqladdress};
//! use std::str::FromStr;
//!
//! // Use the zero address constant
//! let zero = SqlAddress::ZERO;
//!
//! // Create from macro (compile-time)
//! let addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
//!
//! // Create from string (runtime)
//! let addr2 = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
//!
//! // Check if address is zero
//! assert_eq!(addr == SqlAddress::ZERO, false);
//! assert_eq!(zero == SqlAddress::ZERO, true);
//! ```
//!
//! ### SQLx Database Usage
//!
//! ```rust,no_run
//! # use ethereum_mysql::SqlAddress;
//! # use sqlx::Row;
//! # use std::str::FromStr;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // MySQL example with SQLx
//! # #[cfg(feature = "mysql")]
//! # {
//! use sqlx::MySqlPool;
//! 
//! let pool = MySqlPool::connect("mysql://user:pass@localhost/db").await?;
//! 
//! // Insert address directly - no manual conversion needed
//! let user_address = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d")?;
//! sqlx::query("INSERT INTO users (wallet_address) VALUES (?)")
//!     .bind(&user_address)  // SqlAddress implements sqlx::Encode
//!     .execute(&pool)
//!     .await?;
//! 
//! // Query addresses directly - no manual conversion needed  
//! let rows = sqlx::query("SELECT wallet_address FROM users")
//!     .fetch_all(&pool)
//!     .await?;
//! 
//! for row in rows {
//!     let address: SqlAddress = row.get("wallet_address"); // SqlAddress implements sqlx::Decode
//!     println!("User address: {}", address);
//! }
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! ### API Integration Example
//!
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

pub use address::SqlAddress;

// Re-export alloy for macro usage
#[doc(hidden)]
pub use alloy;

/// SQLx-compatible Ethereum Address wrapper.
/// 
/// This is the main type for storing Ethereum addresses in SQL databases via SQLx.
/// It implements all necessary SQLx traits for seamless database integration.
pub type Address = SqlAddress;