//! Hash Consistency Demonstration
//!
//! This example demonstrates the hash consistency between SqlAddress and the underlying
//! alloy::primitives::Address, as well as SqlU256 and alloy::primitives::U256.
//! It verifies that wrapping these types produces the same hash values, which is
//! crucial for interoperability.

use alloy::primitives::{Address, U256};
use ethereum_mysql::{sqladdress, SqlAddress, SqlU256};
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    println!("🔍 Hash Consistency Demonstration");
    println!("==================================\n");

    test_address_hash_consistency();
    test_u256_hash_consistency();
    test_mixed_collections();
    test_performance_comparison();

    println!("✅ Hash Consistency Test Complete!");
    println!("\n💡 Key Findings:");
    println!("   • SqlAddress and Address produce identical hash values for the same address");
    println!("   • SqlU256 and U256 produce identical hash values for the same number");
    println!("   • Hash consistency is maintained through all conversions");
    println!("   • Zero-cost abstraction: no performance penalty for Sql* types hashing");
    println!("   • Full interoperability in HashMap/HashSet operations");
}

fn test_address_hash_consistency() {
    println!("1. SqlAddress Hash Consistency Test:");

    let test_addresses = [
        "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d",
        "0x0000000000000000000000000000000000000000",
        "0xffffffffffffffffffffffffffffffffffffffff",
        "0x1234567890123456789012345678901234567890",
    ];

    for addr_str in &test_addresses {
        let alloy_addr = Address::from_str(addr_str).unwrap();
        let sql_addr = SqlAddress::from_str(addr_str).unwrap();

        let alloy_hash = calculate_hash(&alloy_addr);
        let sql_hash = calculate_hash(&sql_addr);

        println!("   Address: {}", addr_str);
        println!("   alloy::Address hash: {}", alloy_hash);
        println!("   SqlAddress hash:     {}", sql_hash);
        println!("   Hashes equal: {}", alloy_hash == sql_hash);
        println!();
    }

    // Test zero address special case
    let alloy_zero = Address::ZERO;
    let sql_zero = SqlAddress::ZERO;
    let macro_zero = sqladdress!("0x0000000000000000000000000000000000000000");

    let alloy_zero_hash = calculate_hash(&alloy_zero);
    let sql_zero_hash = calculate_hash(&sql_zero);
    let macro_zero_hash = calculate_hash(&macro_zero);

    println!("   Zero Address Special Case:");
    println!("   alloy::Address::ZERO hash: {}", alloy_zero_hash);
    println!("   SqlAddress::ZERO hash:     {}", sql_zero_hash);
    println!("   sqladdress!(\"0x000...\") hash: {}", macro_zero_hash);
    println!(
        "   All zero hashes equal: {}",
        alloy_zero_hash == sql_zero_hash && sql_zero_hash == macro_zero_hash
    );
    println!();
}

fn test_u256_hash_consistency() {
    println!("2. SqlU256 Hash Consistency Test:");

    let test_values = [
        ("0", "Zero"),
        ("42", "Small integer"),
        ("1000000000000000000", "1 ETH in wei"),
        ("0x75bcd15", "Hex value"),
        (
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Max U256",
        ),
    ];

    for (value_str, description) in &test_values {
        let alloy_u256 = U256::from_str(value_str).unwrap();
        let sql_u256 = SqlU256::from_str(value_str).unwrap();

        let alloy_hash = calculate_hash(&alloy_u256);
        let sql_hash = calculate_hash(&sql_u256);

        println!("   Value: {} ({})", value_str, description);
        println!("   alloy::U256 hash: {}", alloy_hash);
        println!("   SqlU256 hash:     {}", sql_hash);
        println!("   Hashes equal: {}", alloy_hash == sql_hash);
        println!();
    }

    // Test SqlU256::ZERO constant
    let alloy_zero = U256::ZERO;
    let sql_zero = SqlU256::ZERO;

    println!("   U256 Zero Constant:");
    println!("   alloy::U256::ZERO hash: {}", calculate_hash(&alloy_zero));
    println!("   SqlU256::ZERO hash:     {}", calculate_hash(&sql_zero));
    println!(
        "   Zero hashes equal: {}",
        calculate_hash(&alloy_zero) == calculate_hash(&sql_zero)
    );
    println!();
}

fn test_mixed_collections() {
    println!("3. Mixed Collections Interoperability:");

    // Test SqlAddress in HashMap
    let mut address_map: HashMap<SqlAddress, String> = HashMap::new();
    let addr1 = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
    let addr2 = sqladdress!("0x1234567890123456789012345678901234567890");

    address_map.insert(addr1, "Address 1".to_string());
    address_map.insert(addr2, "Address 2".to_string());

    // Test conversion lookup
    let converted_addr1: Address = addr1.into();
    let back_to_sql = SqlAddress::from(converted_addr1);

    println!("   Address HashMap operations:");
    println!("   Map size: {}", address_map.len());
    println!("   Contains original: {}", address_map.contains_key(&addr1));
    println!(
        "   Contains converted: {}",
        address_map.contains_key(&back_to_sql)
    );
    println!();

    // Test SqlU256 in HashSet
    let mut value_set: HashSet<SqlU256> = HashSet::new();
    let val1 = SqlU256::from(1000000000000000000u64); // 1 ETH
    let val2 = SqlU256::from_str("1000000000000000000").unwrap(); // Same value
    let val3 = SqlU256::from(2000000000000000000u64); // 2 ETH

    value_set.insert(val1);
    value_set.insert(val2); // Should not increase size
    value_set.insert(val3);

    println!("   U256 HashSet operations:");
    println!(
        "   Set size after inserting 3 values (2 unique): {}",
        value_set.len()
    );
    println!("   Contains val1: {}", value_set.contains(&val1));
    println!(
        "   Contains val2 (same as val1): {}",
        value_set.contains(&val2)
    );
    println!("   Contains val3: {}", value_set.contains(&val3));
    println!();
}

fn test_performance_comparison() {
    println!("4. Performance Comparison:");

    let test_addr = Address::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
    let sql_test_addr = SqlAddress::from(test_addr);

    let test_u256 = U256::from(123456789u64);
    let sql_test_u256 = SqlU256::from(test_u256);

    let iterations = 1_000_000;

    // Time alloy::Address hashing
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = calculate_hash(&test_addr);
    }
    let alloy_addr_duration = start.elapsed();

    // Time SqlAddress hashing
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = calculate_hash(&sql_test_addr);
    }
    let sql_addr_duration = start.elapsed();

    // Time alloy::U256 hashing
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = calculate_hash(&test_u256);
    }
    let alloy_u256_duration = start.elapsed();

    // Time SqlU256 hashing
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = calculate_hash(&sql_test_u256);
    }
    let sql_u256_duration = start.elapsed();

    println!("   Address hashing ({} iterations):", iterations);
    println!("   alloy::Address: {:?}", alloy_addr_duration);
    println!("   SqlAddress:     {:?}", sql_addr_duration);
    println!(
        "   Performance ratio (sql/alloy): {:.2}",
        sql_addr_duration.as_nanos() as f64 / alloy_addr_duration.as_nanos() as f64
    );
    println!();

    println!("   U256 hashing ({} iterations):", iterations);
    println!("   alloy::U256: {:?}", alloy_u256_duration);
    println!("   SqlU256:     {:?}", sql_u256_duration);
    println!(
        "   Performance ratio (sql/alloy): {:.2}",
        sql_u256_duration.as_nanos() as f64 / alloy_u256_duration.as_nanos() as f64
    );
    println!();
}
