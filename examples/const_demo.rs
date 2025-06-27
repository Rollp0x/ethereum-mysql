// Comprehensive demonstration of compile-time constants with sqladdress! macro
// Shows how to use SqlAddress in const contexts, arrays, and static variables

use ethereum_mysql::{sqladdress, SqlAddress};

fn main() {
    println!("🧪 Const sqladdress! Macro Demonstration");
    println!("======================================\n");

    // 1. Basic const declarations
    println!("1. Basic Const Declarations:");

    const ZERO_ADDRESS: SqlAddress = sqladdress!("0x0000000000000000000000000000000000000000");
    const WETH_ADDRESS: SqlAddress = sqladdress!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    const BURN_ADDRESS: SqlAddress = sqladdress!("000000000000000000000000000000000000dEaD");

    println!("   ✅ Const declarations compiled successfully!");
    println!("   ZERO_ADDRESS: {}", ZERO_ADDRESS);
    println!("   WETH_ADDRESS: {}", WETH_ADDRESS);
    println!("   BURN_ADDRESS: {}", BURN_ADDRESS);

    // Verify they're actually compile-time constants
    println!(
        "   Zero address equals SqlAddress::ZERO: {}",
        ZERO_ADDRESS == SqlAddress::ZERO
    );

    println!();

    // 2. Const arrays - useful for predefined address lists
    println!("2. Const Arrays:");

    const COMMON_ADDRESSES: [SqlAddress; 3] = [
        sqladdress!("0x0000000000000000000000000000000000000000"), // Zero address
        sqladdress!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), // WETH
        sqladdress!("000000000000000000000000000000000000dEaD"),   // Burn address
    ];

    println!("   ✅ Const array compiled successfully!");
    println!("   Array contents:");
    for (i, addr) in COMMON_ADDRESSES.iter().enumerate() {
        let label = match i {
            0 => "Zero",
            1 => "WETH",
            2 => "Burn",
            _ => "Unknown",
        };
        println!("     [{}] {}: {}", i, label, addr);
    }

    println!();

    // 3. Static variables - global constants with stable addresses
    println!("3. Static Variables:");

    static STATIC_ADDRESS: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
    static ADMIN_ADDRESSES: [SqlAddress; 2] = [
        sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
        sqladdress!("0x1234567890123456789012345678901234567890"),
    ];

    println!("   ✅ Static variables compiled successfully!");
    println!("   STATIC_ADDRESS: {}", STATIC_ADDRESS);
    println!("   Admin addresses:");
    for (i, addr) in ADMIN_ADDRESSES.iter().enumerate() {
        println!("     Admin {}: {}", i + 1, addr);
    }

    println!();

    // 4. Mixed usage - combining const with runtime values
    println!("4. Mixed Const and Runtime Usage:");

    // Function that uses const addresses
    fn is_system_address(addr: SqlAddress) -> bool {
        const SYSTEM_ADDRESSES: [SqlAddress; 2] = [
            sqladdress!("0x0000000000000000000000000000000000000000"), // Zero
            sqladdress!("0x000000000000000000000000000000000000dEaD"), // Burn
        ];

        SYSTEM_ADDRESSES.contains(&addr)
    }

    let test_addresses = vec![
        SqlAddress::ZERO,
        sqladdress!("0x000000000000000000000000000000000000dEaD"),
        sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
    ];

    for addr in test_addresses {
        println!("   {} -> System address: {}", addr, is_system_address(addr));
    }

    println!();

    // 5. Practical application - configuration constants
    println!("5. Practical Application - Configuration:");

    // Example: DeFi protocol configuration
    mod protocol_config {
        use super::*;

        pub const TREASURY: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
        pub const GOVERNANCE: SqlAddress =
            sqladdress!("0x1234567890123456789012345678901234567890");
        pub const EMERGENCY_PAUSE: SqlAddress =
            sqladdress!("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd");

        pub const SYSTEM_CONTRACTS: [SqlAddress; 3] = [TREASURY, GOVERNANCE, EMERGENCY_PAUSE];
    }

    println!("   Protocol configuration loaded:");
    println!("   Treasury: {}", protocol_config::TREASURY);
    println!("   Governance: {}", protocol_config::GOVERNANCE);
    println!("   Emergency Pause: {}", protocol_config::EMERGENCY_PAUSE);
    println!(
        "   Total system contracts: {}",
        protocol_config::SYSTEM_CONTRACTS.len()
    );

    println!("\n🎉 All const demonstrations completed!");
    println!("💡 Key benefits:");
    println!("   • Zero runtime cost - addresses computed at compile time");
    println!("   • Type safety - invalid addresses cause compilation errors");
    println!("   • No string parsing overhead in production code");
    println!("   • Perfect for configuration and known addresses");
}
