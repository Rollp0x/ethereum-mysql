# ethereum-mysql

[![Crates.io](https://img.shields.io/crates/v/ethereum-mysql.svg)](https://crates.io/crates/ethereum-mysql)
[![Documentation](https://docs.rs/ethereum-mysql/badge.svg)](https://docs.rs/ethereum-mysql)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Ethereum Address wrapper for seamless SQLx database integration.

This crate provides SQL-compatible wrappers for Ethereum types, specifically designed for the [SQLx](https://github.com/launchbadge/sqlx) async SQL toolkit. It supports multiple databases (MySQL, PostgreSQL, SQLite) through SQLx's feature system.

## Features

- **Multi-database support**: MySQL, PostgreSQL, SQLite via SQLx
- **Zero-cost abstractions**: Wraps `alloy::primitives` types
- **Compile-time macros**: Create addresses at compile time with `sqladdress!`
- **Constants**: Pre-defined useful constants like `SqlAddress::ZERO`
- **Serde support**: Optional JSON serialization with serde
- **SQLx native**: Implements `sqlx::Type`, `sqlx::Encode`, and `sqlx::Decode`
- **Pure Rust**: No C dependencies, works with SQLx's pure Rust philosophy

## Quick Start

Add to your `Cargo.toml`:

```toml
# Basic usage (no default features)
ethereum-mysql = "1.0.0"

# Enable specific databases
ethereum-mysql = { version = "1.0.0", features = ["mysql"] }
ethereum-mysql = { version = "1.0.0", features = ["postgres", "serde"] }

# Enable all features
ethereum-mysql = { version = "1.0.0", features = ["full"] }
```

## Feature Flags

- `mysql` - MySQL/MariaDB support via SQLx
- `postgres` - PostgreSQL support via SQLx
- `sqlite` - SQLite support via SQLx  
- `serde` - JSON serialization support
- `full` - Enable all features above

## Usage Examples

### Basic Address Creation

```rust
use ethereum_mysql::{SqlAddress, sqladdress};
use std::str::FromStr;

// Use the zero address constant
let zero = SqlAddress::ZERO;

// Create from macro (compile-time validation)
let addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");

// Create from string (runtime parsing)
let addr2 = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();

// Create constant addresses
const ADMIN: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
```

### SQLx Database Integration

```rust
use ethereum_mysql::SqlAddress;
use sqlx::MySqlPool;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://user:pass@localhost/db").await?;
    
    // Insert address directly - no manual conversion needed
    let user_address = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
    sqlx::query("INSERT INTO users (wallet_address) VALUES (?)")
        .bind(&user_address)  // SqlAddress implements sqlx::Encode
        .execute(&pool)
        .await?;
    
    // Query addresses directly - no manual conversion needed  
    let rows = sqlx::query("SELECT wallet_address FROM users")
        .fetch_all(&pool)
        .await?;
    
    for row in rows {
        let address: SqlAddress = row.get("wallet_address"); // SqlAddress implements sqlx::Decode
        println!("User address: {}", address);
    }
    
    Ok(())
}
```

### JSON Serialization (with serde feature)

```rust
use ethereum_mysql::SqlAddress;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    wallet: SqlAddress,
}

let user = User {
    id: 1,
    wallet: sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
};

let json = serde_json::to_string(&user)?;
let parsed: User = serde_json::from_str(&json)?;
```

## Database Schema Examples

### MySQL

```sql
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    wallet_address VARCHAR(42) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_wallet (wallet_address)
);
```

### PostgreSQL

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(42) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_wallet ON users(wallet_address);
```

### SQLite

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wallet_address TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_wallet ON users(wallet_address);
```

## Why SQLx?

This library is built specifically for [SQLx](https://github.com/launchbadge/sqlx) because:

- **Pure Rust**: No C dependencies or ORM overhead
- **Compile-time safety**: SQL queries are checked at compile time
- **Async-first**: Built for modern async Rust applications
- **Multi-database**: Single API for multiple database backends
- **Performance**: Zero-cost abstractions and prepared statements

## Requirements

- Rust 1.75+ (2024 edition)
- SQLx 0.8+
- alloy 1.0+

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.