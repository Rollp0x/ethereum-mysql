# ethereum-mysql

[![Crates.io](https://img.shields.io/crates/v/ethereum-mysql.svg)](https://crates.io/crates/ethereum-mysql)
[![Documentation](https://docs.rs/ethereum-mysql/badge.svg)](https://docs.rs/ethereum-mysql)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Ethereum types wrapper for seamless SQLx database integration.

This crate provides SQL-compatible wrappers for Ethereum types (`SqlAddress`, `SqlU256`), specifically designed for the [SQLx](https://github.com/launchbadge/sqlx) async SQL toolkit. It supports multiple databases (MySQL, PostgreSQL, SQLite) through SQLx's feature system.

## Features

- **Multi-database support**: MySQL, PostgreSQL, SQLite via SQLx
- **Zero-cost abstractions**: Wraps `alloy::primitives` types (`Address`, `U256`)
- **Complete type support**: `SqlAddress` for Ethereum addresses, `SqlU256` for large integers
- **Arithmetic operations**: Full arithmetic support for `SqlU256` (+, -, *, /, %, bitwise, etc.)
- **🆕 Primitive operations**: Direct arithmetic with Rust primitives (`value * 2`, `2 * value`)
- **Type conversions**: Seamless conversion between `SqlU256` and Rust integer types
- **Compile-time macros**: Create addresses at compile time with `sqladdress!`
- **Constants**: Pre-defined useful constants like `SqlAddress::ZERO`, `SqlU256::ZERO`
- **Serde support**: Optional JSON serialization with serde
- **SQLx native**: Implements `sqlx::Type`, `sqlx::Encode`, and `sqlx::Decode`
- **Pure Rust**: No C dependencies, works with SQLx's pure Rust philosophy

## 🎯 Key Advantages

### 🛡️ **Automatic Type Safety & Validation**
Unlike traditional approaches that require manual string validation, this library provides **compile-time and runtime type safety**:

```rust
// ❌ Traditional approach - error-prone manual validation
#[derive(Deserialize)]
struct OldRequest {
    address: String,  // Could be invalid!
    amount: String,   // Could be invalid!
}

async fn old_handler(req: OldRequest) -> Result<(), Error> {
    // 😰 Manual validation required every time
    let addr = Address::from_str(&req.address)?;  // Runtime error possible
    let amount = U256::from_str(&req.amount)?;    // Runtime error possible
    // ... rest of logic
}

// ✅ Our approach - automatic validation
#[derive(Deserialize)]
struct NewRequest {
    address: SqlAddress,  // 🎯 Auto-validated during deserialization!
    amount: SqlU256,      // 🎯 Auto-validated during deserialization!
}

async fn new_handler(req: NewRequest) -> Result<(), Error> {
    // 🚀 If we reach here, ALL data is already validated!
    // 🚀 Direct database operations without any conversion!
    sqlx::query("INSERT INTO users (addr, balance) VALUES (?, ?)")
        .bind(&req.address)  // Direct binding - no conversion needed
        .bind(&req.amount)   // Direct binding - no conversion needed
        .execute(&pool).await?;
}
```

### ⚡ **Zero Conversion Overhead**
Direct database integration without intermediate conversions:

```rust
// ❌ Other libraries require conversion steps
let addr_str: String = sqlx::query_scalar("SELECT address FROM users WHERE id = ?")
    .bind(user_id).fetch_one(&pool).await?;
let address = Address::from_str(&addr_str)?;  // Extra conversion step

// ✅ Our library - direct type retrieval
let address: SqlAddress = sqlx::query_scalar("SELECT address FROM users WHERE id = ?")
    .bind(user_id).fetch_one(&pool).await?;  // Direct, no conversion needed
```

### 🔥 **Intuitive Primitive Operations**
Revolutionary arithmetic operations with Rust primitives:

```rust
// ❌ Traditional approach - verbose conversions
let fee_rate = U256::from(25);
let base = U256::from(10000);
let fee = (balance * fee_rate) / base;

// ✅ Our approach - direct primitive arithmetic
let fee = balance * 25 / 10000;  // Natural, intuitive syntax!
let doubled = balance * 2;        // Works both ways
let tripled = 3 * balance;        // Much cleaner code
```

### 🎯 **Bulletproof API Development**
Build Web3 APIs with confidence - invalid data is rejected before reaching your business logic:

```rust
// 🛡️ This struct definition provides automatic protection
#[derive(Deserialize)]
struct TransferRequest {
    from: SqlAddress,     // Invalid addresses = HTTP 400 error automatically
    to: SqlAddress,       // No manual validation code needed
    amount: SqlU256,      // Supports both "123" and "0x7b" formats
}

// 🚀 Your handler focuses on business logic, not validation
async fn transfer(Json(req): Json<TransferRequest>) -> Result<Response, Error> {
    // All fields are guaranteed valid when we reach this point!
    let balance: SqlU256 = get_balance(&req.from, &pool).await?;
    if balance >= req.amount {
        execute_transfer(&req.from, &req.to, &req.amount, &pool).await?;
    }
    Ok(success_response())
}
```

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

### Basic U256 Usage

```rust
use ethereum_mysql::SqlU256;
use std::str::FromStr;

// Create from various integer types
let small = SqlU256::from(42u64);
let large = SqlU256::from(u128::MAX);

// Constants
let zero = SqlU256::ZERO;

// From strings (supports both decimal and hex)
let from_decimal = SqlU256::from_str("123456789").unwrap();
let from_hex = SqlU256::from_str("0x75bcd15").unwrap();
assert_eq!(from_decimal, from_hex);

// Arithmetic operations
let a = SqlU256::from(100u64);
let b = SqlU256::from(50u64);

let sum = a + b;                    // 150
let difference = a - b;             // 50  
let product = a * b;                // 5000
let quotient = a / b;               // 2
let remainder = a % b;              // 0

// 🆕 NEW: Direct primitive operations (no conversion needed!)
let doubled = a * 2;                // SqlU256 * primitive
let tripled = 3 * a;                // primitive * SqlU256  
let fee = balance * 25 / 10000;     // 0.25% fee calculation
let gas_cost = gas_price * 21000;   // Direct gas calculation

// Advanced operations
let power = a.pow(3);               // 100^3 = 1,000,000
let square = a.square();            // 100^2 = 10,000
let gcd = SqlU256::from(12u64).gcd(SqlU256::from(8u64)); // 4

// Safe operations
let checked_sum = a.checked_add(b); // Some(150)
let saturated = a.saturating_sub(SqlU256::from(200u64)); // 0

// 🔄 Converting back to original types
let sql_value = SqlU256::from(42u64);

// Method 1: .inner() - Get reference to inner U256 (zero-cost)
let u256_ref: &U256 = sql_value.inner();
println!("Inner U256 reference: {:?}", u256_ref);

// Method 2: .into() - Convert to owned U256 (always safe, no try needed!)
let u256_owned: U256 = sql_value.into();
println!("Owned U256: {:?}", u256_owned);

// Method 3: Auto-deref (thanks to Deref trait)
let u256_deref: U256 = *sql_value;  // Automatic dereference

// 🔄 Converting to Rust primitive types (these need try_into)
let back_to_u64: u64 = small.try_into().unwrap();
let too_large = large.try_into::<u64>(); // Error - value too large for u64

// Display format (always hex)
println!("{}", a); // "0x64"
```

### SQLx Database Integration

```rust
use ethereum_mysql::{SqlAddress, SqlU256};
use sqlx::MySqlPool;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://user:pass@localhost/db").await?;
    
    // Insert address and balance directly - no manual conversion needed
    let user_address = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
    let balance = SqlU256::from_str("1000000000000000000").unwrap(); // 1 ETH in wei
    
    sqlx::query("INSERT INTO users (wallet_address, balance) VALUES (?, ?)")
        .bind(&user_address)  // SqlAddress implements sqlx::Encode
        .bind(&balance)       // SqlU256 implements sqlx::Encode
        .execute(&pool)
        .await?;
    
    // Query with arithmetic operations
    let min_balance = SqlU256::from_str("500000000000000000").unwrap(); // 0.5 ETH in wei
    let rows = sqlx::query("SELECT wallet_address, balance FROM users WHERE balance >= ?")
        .bind(&min_balance)
        .fetch_all(&pool)
        .await?;
    
    for row in rows {
        let address: SqlAddress = row.get("wallet_address"); // SqlAddress implements sqlx::Decode
        let balance: SqlU256 = row.get("balance");           // SqlU256 implements sqlx::Decode
        
        // Perform arithmetic on retrieved values
        let balance_in_eth = balance / SqlU256::from_str("1000000000000000000").unwrap();
        println!("User {} has {} ETH", address, balance_in_eth);
    }
    
    Ok(())
}
```

### JSON Serialization (with serde feature)

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
    balance: SqlU256::from_str("1500000000000000000").unwrap(), // 1.5 ETH
    staked_amount: Some(SqlU256::from_str("1000000000000000000").unwrap()), // 1 ETH
};

let json = serde_json::to_string(&user)?;
// {
//   "id": 1,
//   "wallet": "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d",
//   "balance": "0x14d1120d7b160000",
//   "staked_amount": "0xde0b6b3a7640000"
// }

let parsed: User = serde_json::from_str(&json)?;

// JSON input accepts both formats for SqlU256
let json_with_decimal = r#"{
    "id": 1,
    "wallet": "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d", 
    "balance": "1500000000000000000",
    "staked_amount": "0xde0b6b3a7640000"
}"#;
let parsed_mixed: User = serde_json::from_str(json_with_decimal)?;
```

### 🚀 Web3 API Integration (Type-Safe User Input)

One of the biggest advantages of this library is automatic validation of user input in API endpoints. No more manual address validation or conversion errors!

```rust
use ethereum_mysql::{SqlAddress, SqlU256};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

// ✅ Request structs with automatic validation
#[derive(Deserialize)]
struct TransferRequest {
    from: SqlAddress,     // 🎯 Auto-validates address format!
    to: SqlAddress,       // 🎯 Invalid addresses are rejected before reaching your code
    amount: SqlU256,      // 🎯 Supports both hex ("0x1a2b") and decimal ("123") input
}

#[derive(Deserialize)]
struct StakeRequest {
    user_wallet: SqlAddress,
    stake_amount: SqlU256,
    lock_period_days: u32,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// 🎯 API endpoint with zero manual validation needed
async fn transfer_tokens(
    pool: &MySqlPool,
    req: TransferRequest  // All fields are already validated!
) -> Result<ApiResponse<String>, Box<dyn std::error::Error>> {
    
    // 🚀 No validation needed - if we reach here, all inputs are valid!
    // 🚀 Direct database operations without any conversion!
    
    // Check sender balance
    let balance: SqlU256 = sqlx::query_scalar(
        "SELECT balance FROM users WHERE wallet_address = ?"
    )
    .bind(&req.from)  // Direct use, no conversion!
    .fetch_one(pool)
    .await?;
    
    // Direct arithmetic with primitives
    if balance < req.amount {
        return Ok(ApiResponse {
            success: false,
            data: None,
            error: Some("Insufficient balance".to_string()),
        });
    }
    
    // Calculate fee (0.1%)
    let fee = req.amount * 1 / 1000;  // Direct primitive arithmetic!
    let net_amount = req.amount - fee;
    
    // Execute transfer (single transaction)
    let mut tx = pool.begin().await?;
    
    // Update sender balance
    sqlx::query("UPDATE users SET balance = balance - ? WHERE wallet_address = ?")
        .bind(&req.amount)  // Direct binding!
        .bind(&req.from)
        .execute(&mut *tx)
        .await?;
    
    // Update receiver balance  
    sqlx::query("UPDATE users SET balance = balance + ? WHERE wallet_address = ?")
        .bind(&net_amount)  // Direct binding!
        .bind(&req.to)
        .execute(&mut *tx)
        .await?;
    
    // Record transaction
    sqlx::query(
        "INSERT INTO transactions (from_addr, to_addr, amount, fee) VALUES (?, ?, ?, ?)"
    )
    .bind(&req.from)     // All direct bindings!
    .bind(&req.to)       // No conversions needed!
    .bind(&req.amount)
    .bind(&fee)
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;
    
    Ok(ApiResponse {
        success: true,
        data: Some("Transfer completed successfully".to_string()),
        error: None,
    })
}

// 🎯 Example JSON requests that work automatically:

// ✅ Valid request - will be parsed successfully
let valid_json = r#"{
    "from": "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d",
    "to": "0x8ba1f109551bD432803012645Hac136c82C2c1",  
    "amount": "1000000000000000000"
}"#;

// ❌ Invalid address - will be rejected during deserialization
let invalid_json = r#"{
    "from": "0xinvalid_address",
    "to": "0x8ba1f109551bD432803012645Hac136c82C2c1",
    "amount": "1000000000000000000"  
}"#;

// ✅ Hex amount format also works
let hex_amount_json = r#"{
    "from": "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d",
    "to": "0x8ba1f109551bD432803012645Hac136c82C2c1",
    "amount": "0xde0b6b3a7640000"
}"#;

// 🎯 Integration with web frameworks (example with Axum)
use axum::{extract::Json, response::Json as ResponseJson};

async fn api_transfer(
    pool: &MySqlPool,
    Json(req): Json<TransferRequest>  // Automatic validation happens here!
) -> Result<ResponseJson<ApiResponse<String>>, StatusCode> {
    match transfer_tokens(pool, req).await {
        Ok(response) => Ok(ResponseJson(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 🎯 Benefits Summary:
// ✅ Zero manual validation code needed
// ✅ Invalid addresses rejected automatically  
// ✅ Type-safe database operations
// ✅ Direct arithmetic with primitives
// ✅ Both hex and decimal input formats supported
// ✅ Impossible to forget validation steps
// ✅ Clean, readable business logic
```

## Database Schema Examples

### MySQL

```sql
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    wallet_address VARCHAR(42) NOT NULL,
    balance VARCHAR(66) NOT NULL DEFAULT '0x0',  -- SqlU256 as hex string (max 66 chars)
    token_amount VARCHAR(66) DEFAULT NULL,       -- Optional SqlU256 field
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_wallet (wallet_address),
    INDEX idx_balance (balance)
);

-- Example with transactions
CREATE TABLE transactions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount VARCHAR(66) NOT NULL,                 -- SqlU256 transaction amount
    gas_price VARCHAR(66) NOT NULL,              -- SqlU256 gas price
    gas_limit VARCHAR(66) NOT NULL,              -- SqlU256 gas limit
    block_number VARCHAR(66) NOT NULL,           -- SqlU256 block number
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### PostgreSQL

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(42) NOT NULL,
    balance VARCHAR(66) NOT NULL DEFAULT '0x0',  -- SqlU256 as hex string
    token_amount VARCHAR(66) DEFAULT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_wallet ON users(wallet_address);
CREATE INDEX idx_balance ON users(balance);

-- PostgreSQL supports better constraints
ALTER TABLE users ADD CONSTRAINT balance_format 
    CHECK (balance ~ '^0x[0-9a-fA-F]+$');       -- Ensure hex format
```

### SQLite

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wallet_address TEXT NOT NULL,
    balance TEXT NOT NULL DEFAULT '0x0',         -- SqlU256 as hex string
    token_amount TEXT DEFAULT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_wallet ON users(wallet_address);
CREATE INDEX idx_balance ON users(balance);
```

## Important Notes

### SqlU256 Storage Format

- **Database Storage**: Always stored as hexadecimal strings (e.g., `"0x1a2b3c"`)
- **Input Flexibility**: `FromStr` accepts both decimal (`"123"`) and hex (`"0x7b"`) formats
- **Output Consistency**: `Display` and JSON serialization always use hex format
- **Max Length**: 66 characters (`0x` + 64 hex digits for U256::MAX)

### Type Safety

```rust
// All these work seamlessly with the database
let wei_amount = SqlU256::from_str("1000000000000000000").unwrap(); // 1 ETH
let gwei_price = SqlU256::from(20_000_000_000u64);                  // 20 Gwei
let block_number = SqlU256::from(18_500_000u64);

// Arithmetic operations work naturally
let total_cost = wei_amount + (gwei_price * SqlU256::from(21000u64)); // gas calculation
let next_block = block_number + SqlU256::from(1u64);

// Database operations are type-safe
sqlx::query("INSERT INTO transactions (amount, gas_price, block) VALUES (?, ?, ?)")
    .bind(&wei_amount)    // Stored as "0xde0b6b3a7640000" 
    .bind(&gwei_price)    // Stored as "0x4a817c800"
    .bind(&block_number)  // Stored as "0x11a5140"
    .execute(&pool).await?;
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

## 📊 Comparison with Traditional Approaches

| Aspect | Traditional String-Based | **ethereum-mysql Library** |
|--------|-------------------------|----------------------------|
| **Type Safety** | ❌ Manual validation required | ✅ Automatic validation during deserialization |
| **API Security** | ❌ Invalid data reaches business logic | ✅ Invalid data rejected at API boundary |
| **Code Verbosity** | ❌ Repetitive validation code | ✅ Zero validation boilerplate |
| **Runtime Errors** | ❌ Common string conversion failures | ✅ Compile-time safety + early validation |
| **Database Integration** | ❌ Manual string conversion steps | ✅ Direct type binding/retrieval |
| **Arithmetic Operations** | ❌ Verbose type conversions | ✅ Direct primitive arithmetic (`value * 2`) |
| **Development Speed** | ❌ Slow (manual validation everywhere) | ✅ Fast (focus on business logic) |
| **Bug Probability** | ❌ High (forgotten validations) | ✅ Low (impossible to forget validation) |
| **Input Format Support** | ❌ Usually only one format | ✅ Both decimal and hex automatically |
| **Performance** | ❌ Extra conversion overhead | ✅ Zero conversion overhead |

### 🎯 Real-World Impact Example

```rust
// ❌ Traditional approach: ~15 lines of validation code per endpoint
async fn old_transfer_handler(body: String) -> Result<Response, Error> {
    let req: serde_json::Value = serde_json::from_str(&body)?;
    
    // Manual validation - error-prone and repetitive
    let from_str = req["from"].as_str().ok_or(Error::InvalidInput)?;
    let to_str = req["to"].as_str().ok_or(Error::InvalidInput)?;
    let amount_str = req["amount"].as_str().ok_or(Error::InvalidInput)?;
    
    // More validation - can be forgotten!
    let from_addr = Address::from_str(from_str).map_err(|_| Error::InvalidAddress)?;
    let to_addr = Address::from_str(to_str).map_err(|_| Error::InvalidAddress)?;
    let amount = U256::from_str(amount_str).map_err(|_| Error::InvalidAmount)?;
    
    // Convert for database - more overhead
    let from_sql = SqlAddress::from(from_addr);
    let to_sql = SqlAddress::from(to_addr);
    let amount_sql = SqlU256::from(amount);
    
    // Finally, business logic...
    execute_transfer(&from_sql, &to_sql, &amount_sql, &pool).await
}

// ✅ Our approach: ~3 lines total, impossible to forget validation
async fn new_transfer_handler(Json(req): Json<TransferRequest>) -> Result<Response, Error> {
    // All validation automatic! If we're here, data is guaranteed valid.
    execute_transfer(&req.from, &req.to, &req.amount, &pool).await
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.