use ethereum_mysql::{sqladdress, SqlAddress, SqlU256};
use std::str::FromStr;

// Test if constant declarations are valid
const ADMIN_ADDRESS: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
const ZERO_ADDRESS: SqlAddress = sqladdress!("0x0000000000000000000000000000000000000000");
const SHORT_ADDRESS: SqlAddress = sqladdress!("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");

// Test SqlU256 constants
const ZERO_U256: SqlU256 = SqlU256::ZERO;
const CUSTOM_U256: SqlU256 = SqlU256::ZERO;  // Using ZERO to create another constant

#[test]
fn test_const_addresses() {
    // Verify that constant addresses are created correctly - using alloy's standard format
    assert_eq!(ADMIN_ADDRESS.to_string(), "0x742D35CC6635C0532925A3b8D42cC72b5c2A9a1D");
    assert_eq!(ZERO_ADDRESS.to_string(), "0x0000000000000000000000000000000000000000");
    assert_eq!(SHORT_ADDRESS.to_string(), "0x742D35CC6635C0532925A3b8D42cC72b5c2A9a1D");
}

#[test]
fn test_const_address_equality() {
    // Verify constant address equality
    assert_eq!(ADMIN_ADDRESS, SHORT_ADDRESS);
    assert_ne!(ADMIN_ADDRESS, ZERO_ADDRESS);
}

#[test]
fn test_const_address_runtime_usage() {
    // Verify constant address usage at runtime
    let runtime_addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
    assert_eq!(ADMIN_ADDRESS, runtime_addr);
    
    // Verify that new addresses can be created at runtime
    let new_addr = sqladdress!("0x1234567890123456789012345678901234567890");
    assert_ne!(ADMIN_ADDRESS, new_addr);
}

#[test]
fn test_const_u256_zero() {
    // Verify that SqlU256::ZERO constant works
    assert_eq!(ZERO_U256, SqlU256::from(0u64));
    assert_eq!(ZERO_U256.to_string(), "0x0");
    
    // Verify ZERO constant equality with runtime created zero
    let runtime_zero = SqlU256::from_str("0").unwrap();
    assert_eq!(ZERO_U256, runtime_zero);
    
    // Verify ZERO constant equality with different zero formats
    let hex_zero = SqlU256::from_str("0x0").unwrap();
    let empty_zero = SqlU256::from_str("").unwrap(); // We know this parses as zero
    assert_eq!(ZERO_U256, hex_zero);
    assert_eq!(ZERO_U256, empty_zero);
}

#[test]
fn test_const_u256_operations() {
    // Verify that constants can be used in operations
    let one = SqlU256::from(1u64);
    let result = ZERO_U256 + one;
    assert_eq!(result, one);
    
    // Verify constants work with arithmetic
    let doubled_custom = CUSTOM_U256 * 2;
    assert_eq!(doubled_custom, SqlU256::from(0u64)); // Since CUSTOM_U256 is zero
    
    // Verify constants are copy/clone
    let copied_zero = ZERO_U256;
    assert_eq!(copied_zero, ZERO_U256);
}

#[test]
fn test_mixed_constants() {
    // Verify that both address and U256 constants work together
    assert_eq!(ZERO_ADDRESS, SqlAddress::ZERO);
    assert_eq!(ZERO_U256, SqlU256::ZERO);
    
    // Different types of zero should not interfere
    assert_eq!(ZERO_ADDRESS.to_string(), "0x0000000000000000000000000000000000000000");
    assert_eq!(ZERO_U256.to_string(), "0x0");
}
