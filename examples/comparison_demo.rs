use ethereum_mysql::{SqlU256, SqlAddress};
use std::str::FromStr;

fn main() {
    println!("=== SqlU256 and SqlAddress Comparison Examples ===\n");

    // SqlU256 comparison examples
    println!("📊 SqlU256 Comparisons:");
    let a = SqlU256::from(100u64);
    let b = SqlU256::from(200u64);
    let zero = SqlU256::ZERO;
    
    println!("  a = {}", a);
    println!("  b = {}", b);
    println!("  zero = {}", zero);
    println!("  a < b: {}", a < b);
    println!("  a > zero: {}", a > zero);
    println!("  a.min(b): {}", a.min(b));
    println!("  a.max(b): {}", a.max(b));
    
    // Large number comparisons
    let large = SqlU256::from_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let medium = SqlU256::from_str("0x123456789abcdef0123456789abcdef").unwrap();
    
    println!("\n  Large number comparisons:");
    println!("  large = {}", large);
    println!("  medium = {}", medium);
    println!("  medium < large: {}", medium < large);
    
    // SqlAddress comparison examples
    println!("\n🏠 SqlAddress Comparisons:");
    let addr1 = SqlAddress::from_str("0x6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(); // DAI
    let addr2 = SqlAddress::from_str("0xA0b86a33E6441E85C7A4d8E4d80D6Bb4BF5BF2Aa").unwrap(); // UNI
    let zero_addr = SqlAddress::ZERO;
    
    println!("  addr1 (DAI) = {}", addr1);
    println!("  addr2 (UNI) = {}", addr2);
    println!("  zero_addr = {}", zero_addr);
    println!("  addr1 < addr2: {}", addr1 < addr2);
    println!("  addr1 > zero_addr: {}", addr1 > zero_addr);
    
    // UniswapV2 token ordering example
    println!("\n🔄 UniswapV2 Token Ordering:");
    let (token0, token1) = if addr1 < addr2 {
        (addr1, addr2)
    } else {
        (addr2, addr1)
    };
    
    println!("  Original addresses:");
    println!("    DAI: {}", addr1);
    println!("    UNI: {}", addr2);
    println!("  Sorted for UniswapV2 pair:");
    println!("    token0: {}", token0);
    println!("    token1: {}", token1);
    println!("  ✅ token0 < token1: {}", token0 < token1);
    
    // Sorting collections
    println!("\n📊 Sorting Collections:");
    
    // Sort numbers
    let mut numbers = vec![
        SqlU256::from(300u64),
        SqlU256::from(100u64),
        SqlU256::ZERO,
        SqlU256::from(200u64),
    ];
    println!("  Numbers before sort: {:?}", numbers);
    numbers.sort();
    println!("  Numbers after sort:  {:?}", numbers);
    
    // Sort addresses
    let mut addresses = vec![
        SqlAddress::from_str("0x0000000000000000000000000000000000000003").unwrap(),
        SqlAddress::from_str("0x0000000000000000000000000000000000000001").unwrap(),
        SqlAddress::ZERO,
        SqlAddress::from_str("0x0000000000000000000000000000000000000002").unwrap(),
    ];
    println!("\n  Addresses before sort:");
    for addr in &addresses {
        println!("    {}", addr);
    }
    addresses.sort();
    println!("  Addresses after sort:");
    for addr in &addresses {
        println!("    {}", addr);
    }
    
    // DeFi scenarios
    println!("\n💰 DeFi Balance Comparison Scenarios:");
    
    // Balance checking
    let balance = SqlU256::from_str("1000000000000000000").unwrap(); // 1 ETH in wei
    let transfer_amount = SqlU256::from_str("500000000000000000").unwrap(); // 0.5 ETH in wei
    let min_balance = SqlU256::from_str("100000000000000000").unwrap(); // 0.1 ETH in wei
    
    println!("  Current balance: {} wei", balance);
    println!("  Transfer amount: {} wei", transfer_amount);
    println!("  Minimum balance: {} wei", min_balance);
    
    if balance >= transfer_amount {
        let remaining = balance - transfer_amount;
        println!("  ✅ Transfer possible, remaining: {} wei", remaining);
        
        if remaining >= min_balance {
            println!("  ✅ Remaining balance above minimum");
        } else {
            println!("  ⚠️  Remaining balance below minimum");
        }
    } else {
        println!("  ❌ Insufficient balance for transfer");
    }
    
    // Price comparison
    let price_a = SqlU256::from_str("2500000000000000000000").unwrap(); // 2500 USD (scaled by 1e18)
    let price_b = SqlU256::from_str("65000000000000000000000").unwrap(); // 65000 USD (scaled by 1e18)
    
    println!("\n  Asset prices (scaled by 1e18):");
    println!("  ETH price: {}", price_a);
    println!("  BTC price: {}", price_b);
    println!("  BTC more expensive than ETH: {}", price_b > price_a);
    println!("  Price ratio (BTC/ETH): {}", price_b / price_a);
}
