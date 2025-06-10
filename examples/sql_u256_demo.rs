// Example: Comprehensive SqlU256 usage
// This example demonstrates all the key features of SqlU256

use ethereum_mysql::{SqlAddress, SqlU256, sqladdress};
use std::str::FromStr;

fn main() {
    println!("=== SqlU256 Comprehensive Example ===\n");

    // 1. Basic Creation
    println!("1. Basic Creation:");
    let zero = SqlU256::ZERO;
    let from_u64 = SqlU256::from(42u64);
    let from_decimal = SqlU256::from_str("123456789").unwrap();
    let from_hex = SqlU256::from_str("0x75bcd15").unwrap();
    
    println!("  Zero: {}", zero);
    println!("  From u64: {}", from_u64);
    println!("  From decimal string: {}", from_decimal);
    println!("  From hex string: {}", from_hex);
    println!("  Decimal and hex are equal: {}", from_decimal == from_hex);

    // 2. Arithmetic Operations
    println!("\n2. Arithmetic Operations:");
    let a = SqlU256::from(100u64);
    let b = SqlU256::from(50u64);
    
    println!("  a = {}, b = {}", a, b);
    println!("  a + b = {}", a + b);
    println!("  a - b = {}", a - b);
    println!("  a * b = {}", a * b);
    println!("  a / b = {}", a / b);
    println!("  a % b = {}", a % b);

    // 3. Advanced Math
    println!("\n3. Advanced Mathematical Operations:");
    let base = SqlU256::from(5u64);
    println!("  base = {}", base);
    println!("  base.square() = {}", base.square());
    println!("  base.pow(3) = {}", base.pow(3));
    
    let gcd_result = SqlU256::from(12u64).gcd(SqlU256::from(8u64));
    let lcm_result = SqlU256::from(12u64).lcm(SqlU256::from(8u64));
    println!("  gcd(12, 8) = {}", gcd_result);
    println!("  lcm(12, 8) = {}", lcm_result);

    // 4. Bitwise Operations
    println!("\n4. Bitwise Operations:");
    let x = SqlU256::from(0b1100u64);
    let y = SqlU256::from(0b1010u64);
    println!("  x = {} (binary: 1100)", x);
    println!("  y = {} (binary: 1010)", y);
    println!("  x & y = {} (binary: 1000)", x & y);
    println!("  x | y = {} (binary: 1110)", x | y);
    println!("  x ^ y = {} (binary: 0110)", x ^ y);

    // 5. Safe Operations
    println!("\n5. Safe Operations:");
    let large = SqlU256::from(u64::MAX);
    let small = SqlU256::from(100u64);
    
    println!("  large.checked_add(small): {:?}", large.checked_add(small));
    println!("  small.checked_sub(large): {:?}", small.checked_sub(large));
    println!("  small.saturating_sub(large): {}", small.saturating_sub(large));

    // 6. Type Conversions
    println!("\n6. Type Conversions:");
    let values: &[u64] = &[42, 1000, 100000, 1_000_000_000];
    
    for &val in values {
        let sql_val = SqlU256::from(val);
        let back: Result<u64, _> = sql_val.try_into();
        println!("  {} -> SqlU256 -> u64: {:?}", val, back);
    }

    // 7. Ethereum-specific Examples
    println!("\n7. Ethereum-specific Examples:");
    
    // Wei amounts (1 ETH = 10^18 wei)
    let one_eth_wei = SqlU256::from_str("1000000000000000000").unwrap();
    let gas_price_gwei = SqlU256::from(20_000_000_000u64); // 20 Gwei
    let gas_limit = SqlU256::from(21000u64);
    
    println!("  1 ETH in wei: {}", one_eth_wei);
    println!("  Gas price (20 Gwei): {}", gas_price_gwei);
    println!("  Gas limit: {}", gas_limit);
    
    let transaction_cost = gas_price_gwei * gas_limit;
    let remaining_balance = one_eth_wei.saturating_sub(transaction_cost);
    
    println!("  Transaction cost: {}", transaction_cost);
    println!("  Remaining balance: {}", remaining_balance);

    // 8. Database Simulation
    println!("\n8. Database Storage Simulation:");
    
    struct MockUser {
        address: SqlAddress,
        balance: SqlU256,
        nonce: SqlU256,
    }
    
    let user = MockUser {
        address: sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
        balance: SqlU256::from_str("1500000000000000000").unwrap(), // 1.5 ETH
        nonce: SqlU256::from(42u64),
    };
    
    // Simulate database storage format
    println!("  Address (stored as): {}", user.address);
    println!("  Balance (stored as): {}", user.balance);
    println!("  Nonce (stored as): {}", user.nonce);
    
    // Simulate parsing from database
    let stored_balance = format!("{}", user.balance);
    let parsed_balance = SqlU256::from_str(&stored_balance).unwrap();
    println!("  Parsed balance equals original: {}", parsed_balance == user.balance);

    // 9. Format Compatibility
    println!("\n9. Format Compatibility:");
    let test_value = SqlU256::from(255u64);
    
    // Display format (always hex)
    let display_format = format!("{}", test_value);
    println!("  Display format: {}", display_format);
    
    // Can parse both formats
    let from_display = SqlU256::from_str(&display_format).unwrap();
    let from_decimal = SqlU256::from_str("255").unwrap();
    
    println!("  Parsed from display: {}", from_display);
    println!("  Parsed from decimal: {}", from_decimal);
    println!("  Both equal: {}", from_display == from_decimal);

    println!("\n=== All examples completed successfully! ===");
}
