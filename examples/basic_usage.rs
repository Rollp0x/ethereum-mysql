// Basic usage examples for ethereum-mysql crate
// Demonstrates various ways to create and use SqlAddress

use alloy::primitives::Address;
use ethereum_mysql::{sqladdress, SqlAddress};
use std::str::FromStr;

fn main() {
    println!("🚀 ethereum-mysql Basic Usage Examples");
    println!("=====================================\n");

    // 1. Different ways to create SqlAddress instances
    println!("1. Creating SqlAddress instances:");

    // Create from string
    let addr1 = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
    println!("   From string: {}", addr1);

    // Create from raw Address
    let raw_addr = Address::ZERO;
    let addr2 = SqlAddress::from(raw_addr);
    println!("   From Address: {}", addr2);

    // Create using sqladdress! macro (recommended approach)
    let addr3 = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
    println!("   From macro (with 0x): {}", addr3);

    let addr4 = sqladdress!("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
    println!("   From macro (without 0x): {}", addr4);

    println!();

    // 2. API parameter usage demonstration
    println!("2. API Parameter Usage:");

    // Simulate API endpoint: get account balance
    fn get_account_balance(address: SqlAddress) -> u64 {
        println!("   Querying balance for address: {}", address);
        // In real applications, this would:
        // 1. Use address directly for database queries (no type conversion needed)
        // 2. Or call blockchain RPC (use address.inner() to get raw Address)
        // Mock balance
        12345
    }

    // User input can be directly used as SqlAddress type
    let user_input = "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d";
    match SqlAddress::from_str(user_input) {
        Ok(address) => {
            let balance = get_account_balance(address);
            println!("   Account balance: {} tokens", balance);
        }
        Err(e) => {
            println!("   Invalid address: {}", e);
        }
    }

    println!();

    // 3. Type conversion demonstrations
    println!("3. Type Conversions:");

    let sql_addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");

    // Get inner Address
    let raw_address: Address = sql_addr.inner();
    println!("   Raw Address: {:?}", raw_address);

    // Convert to string (for JSON serialization or database storage)
    let addr_string = sql_addr.to_string();
    println!("   As String: {}", addr_string);

    // Access Address methods directly via Deref
    let checksum = sql_addr.to_checksum(None);
    println!("   Checksum format: {}", checksum);

    println!();

    // 4. Common address constants
    println!("4. Common Address Constants:");

    // Using ZERO constant
    let zero_addr = SqlAddress::ZERO;
    let burn_addr = sqladdress!("0x000000000000000000000000000000000000dEaD");
    let weth_addr = sqladdress!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

    println!("   Zero Address: {}", zero_addr);
    println!("   Burn Address: {}", burn_addr);
    println!("   WETH Address: {}", weth_addr);

    // Demonstrate equality with zero address
    println!(
        "   Is zero_addr actually zero? {}",
        zero_addr == SqlAddress::ZERO
    );

    println!();

    // 5. Error handling demonstration
    println!("5. Error Handling:");

    let invalid_addresses = vec![
        "invalid",
        "0x123",                                       // Too short
        "0xgg42d35Cc6635C0532925a3b8D42cC72b5c2A9A1d", // Contains invalid characters
        "",                                            // Empty string
        "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1dX", // Too long
    ];

    for invalid_addr in invalid_addresses {
        match SqlAddress::from_str(invalid_addr) {
            Ok(addr) => println!("   ✅ Valid: {}", addr),
            Err(e) => println!("   ❌ Invalid '{}': {}", invalid_addr, e),
        }
    }

    println!();

    // 6. Database usage pattern (pseudo-code)
    println!("6. Database Usage Pattern:");

    // Demonstrate how SqlAddress would be used in database operations
    fn simulate_database_operations() {
        println!("   // In real applications with sqlx:");
        println!("   // let pool = SqlitePool::connect(\"sqlite::memory:\").await?;");
        println!("   // ");
        println!("   // INSERT operation:");
        println!("   // sqlx::query(\"INSERT INTO addresses (address, label) VALUES (?, ?)\")");
        println!("   //     .bind(&SqlAddress::ZERO)  // Direct binding, no conversion needed");
        println!("   //     .bind(\"Zero Address\")");
        println!("   //     .execute(&pool).await?;");
        println!("   // ");
        println!("   // SELECT operation:");
        println!("   // let row = sqlx::query(\"SELECT address FROM addresses WHERE id = 1\")");
        println!("   //     .fetch_one(&pool).await?;");
        println!("   // let address: SqlAddress = row.get(\"address\");  // Direct retrieval");
    }

    simulate_database_operations();

    println!("\n✨ All examples completed successfully!");
    println!("💡 Next steps: Check out const_demo.rs and zero_constant.rs for more examples!");
}
