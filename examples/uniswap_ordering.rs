use ethereum_mysql::SqlAddress;
use std::str::FromStr;

fn main() {
    println!("🔄 UniswapV2 Token Pair Ordering Demo\n");

    // Real token addresses from Ethereum mainnet
    let tokens = vec![
        ("WETH", "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
        ("USDC", "0xA0b86a33E6441E85C7A4d8E4d80D6Bb4BF5BF2Aa"), 
        ("DAI", "0x6B175474E89094C44Da98b954EedeAC495271d0F"),
        ("USDT", "0xdAC17F958D2ee523a2206206994597C13D831ec7"),
        ("WBTC", "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"),
        ("UNI", "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"),
    ];

    println!("📊 Token Addresses:");
    let mut addresses = Vec::new();
    for (symbol, addr_str) in &tokens {
        let addr = SqlAddress::from_str(addr_str).unwrap();
        println!("  {}: {}", symbol, addr);
        addresses.push((symbol, addr));
    }

    println!("\n🔄 UniswapV2 Pair Creation (token0 < token1):");
    
    // Create all possible pairs and show ordering
    for i in 0..addresses.len() {
        for j in (i + 1)..addresses.len() {
            let (symbol_a, addr_a) = addresses[i];
            let (symbol_b, addr_b) = addresses[j];
            
            // UniswapV2 ordering: token0 < token1
            let (token0_symbol, token0_addr, token1_symbol, token1_addr) = 
                if addr_a < addr_b {
                    (symbol_a, addr_a, symbol_b, addr_b)
                } else {
                    (symbol_b, addr_b, symbol_a, addr_a)
                };
            
            println!("  {}/{} pair:", symbol_a, symbol_b);
            println!("    token0: {} ({})", token0_symbol, token0_addr);
            println!("    token1: {} ({})", token1_symbol, token1_addr);
            println!("    ✅ token0 < token1: {}", token0_addr < token1_addr);
            println!();
        }
    }

    println!("🎯 Why Address Ordering Matters:");
    println!("  • UniswapV2 requires deterministic pair addresses");
    println!("  • token0 must always be < token1 to ensure uniqueness");
    println!("  • Prevents duplicate pairs (A/B vs B/A)");
    println!("  • Essential for factory contract pair creation");
    println!("  • Enables predictable pair address calculation");

    println!("\n📈 Sorting All Tokens by Address:");
    let mut sorted_tokens = addresses.clone();
    sorted_tokens.sort_by(|a, b| a.1.cmp(&b.1));
    
    for (i, (symbol, addr)) in sorted_tokens.iter().enumerate() {
        println!("  {}. {}: {}", i + 1, symbol, addr);
    }

    println!("\n💡 Practical Use Cases:");
    println!("  • Automated pair creation in DEX implementations");
    println!("  • Token sorting in DeFi protocols");
    println!("  • Consistent ordering in smart contract arrays");
    println!("  • Database indexing and query optimization");
    println!("  • Frontend token list organization");
}
