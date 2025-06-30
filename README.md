# ethereum-mysql

[![Crates.io](https://img.shields.io/crates/v/ethereum-mysql.svg)](https://crates.io/crates/ethereum-mysql)
[![Documentation](https://docs.rs/ethereum-mysql/badge.svg)](https://docs.rs/ethereum-mysql)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Design Purpose:**

`ethereum-mysql` is a pragmatic Rust library that provides type-safe, ergonomic wrappers for Ethereum types (Address, U256, FixedBytes, Bytes) for seamless integration with SQLx and relational databases (MySQL, PostgreSQL, SQLite). The crate is designed for real-world Web3/data engineering scenarios, focusing on:

- **String-based storage:** All types are stored as human-readable hex strings (e.g. `0x...`) in the database, ensuring maximum compatibility and easy inspection.
- **Type safety:** Compile-time and runtime validation for all Ethereum types, eliminating manual string parsing and validation in business logic.
- **API ergonomics:** Direct arithmetic, comparison, and conversion with Rust primitives for U256, and compile-time address macros for zero-cost, safe usage.
- **No binary mode:** Binary column support is intentionally omitted; for binary storage, use the official alloy types directly or wait for native support in your stack.
- **Minimal, focused features:** Only the most practical and widely-used Ethereum types and operations are supported, with optional serde integration.

---

## Features

- **Multi-database support:** MySQL, PostgreSQL, SQLite via SQLx
- **Zero-cost abstractions:** Wraps `alloy::primitives` types (`Address`, `U256`, `FixedBytes`, `Bytes`)
- **Type-safe wrappers:** `SqlAddress`, `SqlU256`, `SqlFixedBytes`, `SqlBytes` for direct DB integration
- **Arithmetic operations:** Full arithmetic and comparison for `SqlU256` (U256), including with Rust primitives
- **Type conversions:** Seamless conversion between wrappers and Rust integer types
- **Compile-time macros:** Create addresses at compile time with `sqladdress!`
- **Constants:** Pre-defined constants like `SqlAddress::ZERO`, `SqlU256::ZERO`
- **Serde support:** Optional JSON serialization with serde
- **SQLx native:** Implements `sqlx_core::Type`, `sqlx_core::Encode`, and `sqlx_core::Decode` for all wrappers
- **Pure Rust:** No C dependencies, async-first, production ready
- **Common 32-byte types** `SqlHash` and `SqlTopicHash` are exported as type aliases for `SqlFixedBytes<32>`

## Database Column Types

All types are stored as strings (hex with `0x` prefix) for maximum compatibility:

| Type             | Recommended Column Type |
|------------------|------------------------|
| SqlAddress       | VARCHAR(42)            |
| SqlU256          | VARCHAR(66)            |
| SqlFixedBytes<N> | VARCHAR(2+2*N)         |
| SqlBytes         | TEXT                   |

- For PostgreSQL, use `TEXT` for all string types.
- For MySQL/SQLite, use `VARCHAR` as above.

## U256 String Encoding
- All U256/FixedBytes types are written as lowercase hex strings with `0x` prefix (e.g. `0x1234...`).
- Reading supports both `0x`-prefixed hex and decimal strings (but hex is recommended for consistency).
- For predictable sorting/comparison, always store as hex.

---

## Key Advantages

### 🛡️ Type Safety & Validation
All wrappers provide compile-time and runtime validation. No more manual string parsing or error-prone conversions in your business logic.

### ⚡ Zero Conversion Overhead
Directly bind and retrieve types from SQLx queries—no intermediate conversions or manual parsing required.

### 🔥 Intuitive Arithmetic
Direct arithmetic and comparison between `SqlU256` and Rust primitives, e.g. `balance * 25 / 10000`.

### 🚀 API-Ready
Request structs can use `SqlAddress`/`SqlU256` directly for automatic validation and type safety in web APIs.

---

## Quick Start
Add to your `Cargo.toml`:

```toml
ethereum-mysql = "3.0.0"
```

---

## Example Usage

### Address Creation
```rust
use ethereum_mysql::{SqlAddress, sqladdress};
use std::str::FromStr;

let zero = SqlAddress::ZERO;
let addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
let addr2 = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
const ADMIN: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
```

### U256 Usage
```rust
use ethereum_mysql::SqlU256;
use std::str::FromStr;

let small = SqlU256::from(42u64);
let large = SqlU256::from(u128::MAX);
let zero = SqlU256::ZERO;
let from_decimal = SqlU256::from_str("123456789").unwrap();
let from_hex = SqlU256::from_str("0x75bcd15").unwrap();
assert_eq!(from_decimal, from_hex);

let a = SqlU256::from(100u64);
let b = SqlU256::from(50u64);
let sum = a + b;
let product = a * b;
let doubled = a * 2;
let tripled = 3 * a;
```

### SQLx Integration (String Mode)
```rust
use ethereum_mysql::{SqlAddress, SqlU256, SqlHash};
use sqlx::MySqlPool;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://user:pass@localhost/db").await?;
    let user_address = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
    let balance = SqlU256::from_str("1000000000000000000").unwrap();
    let tx_hash = SqlHash::from_str("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            wallet_address VARCHAR(42) NOT NULL,
            balance VARCHAR(66),
            tx_hash VARCHAR(66),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await?;
    sqlx::query("INSERT INTO users (wallet_address, balance, tx_hash) VALUES (?, ?, ?)")
        .bind(&user_address)
        .bind(&balance)
        .bind(&tx_hash)
        .execute(&pool)
        .await?;
    Ok(())
}
```

### JSON Serialization (with serde)
```rust
use ethereum_mysql::{SqlAddress, SqlU256, sqladdress};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    wallet: SqlAddress,
    balance: SqlU256,
    staked_amount: Option<SqlU256>,
}

let user = User {
    id: 1,
    wallet: sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
    balance: SqlU256::from_str("1500000000000000000").unwrap(),
    staked_amount: Some(SqlU256::from_str("1000000000000000000").unwrap()),
};
let json = serde_json::to_string(&user)?;
```

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.