//! Zero Address Constant Demonstration
//!
//! This example demonstrates the usage of the `SqlAddress::ZERO` constant, which represents
//! the Ethereum zero address (0x0000000000000000000000000000000000000000).
//!
//! The zero address is commonly used in Ethereum applications to represent:
//! - Uninitialized addresses
//! - Null values in address fields
//! - Default states in smart contracts
//! - Burn addresses (tokens sent to zero address are effectively burned)
//!
//! ## Examples Covered:
//! 1. Basic usage of SqlAddress::ZERO
//! 2. Comparison with other address creation methods
//! 3. Usage in const and static contexts
//! 4. Practical applications in address validation
//! 5. Usage in collections and data structures
//! 6. Real-world business scenarios

use ethereum_mysql::{sqladdress, SqlAddress};
use std::str::FromStr;

fn main() {
    println!("=== SqlAddress::ZERO Constant Demonstration ===\n");

    // Section 1: Basic usage of ZERO constant
    println!("1. Basic Usage of SqlAddress::ZERO:");
    println!("   SqlAddress::ZERO = {}", SqlAddress::ZERO);
    println!("   Debug format: {:?}", SqlAddress::ZERO);
    println!(
        "   Is zero address: {}",
        SqlAddress::ZERO.to_string().starts_with("0x000000000")
    );

    // Section 2: Comparison with other creation methods
    println!("\n2. Comparison with Other Creation Methods:");
    let zero_from_str = SqlAddress::from_str("0x0000000000000000000000000000000000000000").unwrap();
    let zero_from_macro = sqladdress!("0x0000000000000000000000000000000000000000");

    println!("   from_str():       {}", zero_from_str);
    println!("   sqladdress!():    {}", zero_from_macro);
    println!("   SqlAddress::ZERO: {}", SqlAddress::ZERO);

    println!(
        "   All methods equal: {}",
        SqlAddress::ZERO == zero_from_str && SqlAddress::ZERO == zero_from_macro
    );

    // Section 3: Usage in const and static contexts
    println!("\n3. Usage in Const and Static Variables:");
    const ZERO_CONST: SqlAddress = SqlAddress::ZERO;
    static ZERO_STATIC: SqlAddress = SqlAddress::ZERO;

    println!("   ZERO_CONST:  {}", ZERO_CONST);
    println!("   ZERO_STATIC: {}", ZERO_STATIC);

    // Demonstrate compile-time evaluation - SqlAddress::ZERO is available at compile time
    println!("   ZERO constant is compile-time evaluable: true");

    // Section 4: Practical applications in address validation
    println!("\n4. Practical Applications - Address Validation:");

    /// Checks if an address is the zero address (null address)
    fn is_zero_address(addr: SqlAddress) -> bool {
        addr == SqlAddress::ZERO
    }

    /// Validates if an address is suitable for token transfers
    fn is_valid_transfer_address(addr: SqlAddress) -> bool {
        addr != SqlAddress::ZERO
    }

    let test_addresses = [
        SqlAddress::ZERO,
        sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
        sqladdress!("0x1234567890123456789012345678901234567890"),
    ];

    for (i, addr) in test_addresses.iter().enumerate() {
        println!(
            "   Address {}: {} -> Is zero: {}, Valid for transfer: {}",
            i,
            addr,
            is_zero_address(*addr),
            is_valid_transfer_address(*addr)
        );
    }

    // Section 5: Usage in collections and data structures
    println!("\n5. Usage in Arrays and Collections:");

    // Compile-time array initialization with zero addresses
    const DEFAULT_ADDRESSES: [SqlAddress; 3] =
        [SqlAddress::ZERO, SqlAddress::ZERO, SqlAddress::ZERO];

    println!("   Default address array (const):");
    for (i, addr) in DEFAULT_ADDRESSES.iter().enumerate() {
        println!("     [{}]: {}", i, addr);
    }

    // Runtime vector with mixed addresses
    let mut address_registry = Vec::new();
    address_registry.push(SqlAddress::ZERO); // Placeholder for admin
    address_registry.push(sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"));
    address_registry.push(SqlAddress::ZERO); // Placeholder for future expansion

    println!("   Address registry with placeholders:");
    for (i, addr) in address_registry.iter().enumerate() {
        let status = if *addr == SqlAddress::ZERO {
            "Placeholder"
        } else {
            "Active"
        };
        println!("     [{}]: {} ({})", i, addr, status);
    }

    // Section 6: Real-world business scenarios
    println!("\n6. Real-World Business Scenarios:");

    /// Represents a user account in a DeFi application
    #[derive(Debug, Clone)]
    struct DeFiUser {
        id: u32,
        name: String,
        wallet_address: SqlAddress,
        referrer: SqlAddress, // Zero address means no referrer
    }

    impl DeFiUser {
        fn has_wallet(&self) -> bool {
            self.wallet_address != SqlAddress::ZERO
        }

        fn has_referrer(&self) -> bool {
            self.referrer != SqlAddress::ZERO
        }

        fn get_status(&self) -> &'static str {
            match (self.has_wallet(), self.has_referrer()) {
                (true, true) => "Complete Profile with Referrer",
                (true, false) => "Complete Profile",
                (false, _) => "Incomplete Profile",
            }
        }
    }

    let users = vec![
        DeFiUser {
            id: 1,
            name: "Alice".to_string(),
            wallet_address: sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
            referrer: sqladdress!("0x1234567890123456789012345678901234567890"),
        },
        DeFiUser {
            id: 2,
            name: "Bob".to_string(),
            wallet_address: SqlAddress::ZERO, // No wallet connected yet
            referrer: SqlAddress::ZERO,
        },
        DeFiUser {
            id: 3,
            name: "Charlie".to_string(),
            wallet_address: sqladdress!("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"),
            referrer: SqlAddress::ZERO, // No referrer
        },
    ];

    println!("   DeFi User Management System:");
    for user in &users {
        println!(
            "     User #{}: {} - Wallet: {} - Status: {}",
            user.id,
            user.name,
            user.wallet_address,
            user.get_status()
        );
    }

    // Demonstrate filtering and business logic
    let users_needing_setup: Vec<_> = users.iter().filter(|user| !user.has_wallet()).collect();

    let eligible_for_referral_bonus: Vec<_> = users
        .iter()
        .filter(|user| user.has_wallet() && user.has_referrer())
        .collect();

    println!("\n   Business Analytics:");
    println!(
        "     Users needing wallet setup: {}",
        users_needing_setup.len()
    );
    println!(
        "     Users eligible for referral bonus: {}",
        eligible_for_referral_bonus.len()
    );

    // Smart contract interaction simulation
    println!("\n7. Smart Contract Integration Patterns:");

    /// Simulates a token contract interaction
    fn simulate_token_transfer(
        from: SqlAddress,
        to: SqlAddress,
        amount: u64,
    ) -> Result<String, String> {
        if from == SqlAddress::ZERO {
            return Err(
                "Cannot transfer from zero address (minting requires special permissions)"
                    .to_string(),
            );
        }
        if to == SqlAddress::ZERO {
            return Ok(format!("Burned {} tokens from {}", amount, from));
        }
        Ok(format!(
            "Transferred {} tokens from {} to {}",
            amount, from, to
        ))
    }

    let transfer_scenarios = [
        (
            sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
            SqlAddress::ZERO,
            100,
        ), // Burn
        (
            SqlAddress::ZERO,
            sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
            50,
        ), // Invalid mint
        (
            sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
            sqladdress!("0x1234567890123456789012345678901234567890"),
            75,
        ), // Normal transfer
    ];

    for (i, (from, to, amount)) in transfer_scenarios.iter().enumerate() {
        match simulate_token_transfer(*from, *to, *amount) {
            Ok(msg) => println!("     Scenario {}: ✅ {}", i + 1, msg),
            Err(err) => println!("     Scenario {}: ❌ {}", i + 1, err),
        }
    }

    println!("\n✅ SqlAddress::ZERO Constant Demonstration Complete!");
    println!("\n💡 Key Takeaways:");
    println!("   • Use SqlAddress::ZERO for uninitialized or null address values");
    println!("   • Zero address checks are essential for smart contract safety");
    println!("   • The constant is compile-time evaluated for optimal performance");
    println!("   • Common in DeFi for burn mechanics and validation logic");
}
