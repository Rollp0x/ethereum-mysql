use ethereum_mysql::{sqladdress, SqlAddress};

// Test if constant declarations are valid
const ADMIN_ADDRESS: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
const ZERO_ADDRESS: SqlAddress = sqladdress!("0x0000000000000000000000000000000000000000");
const SHORT_ADDRESS: SqlAddress = sqladdress!("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");

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
