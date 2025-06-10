use ethereum_mysql::SqlU256;
use std::str::FromStr;

fn main() {
    println!("🚀 SqlU256 Primitive Operations Demo");
    println!("====================================");
    
    // Ethereum balance example
    let balance = SqlU256::from_str("1000000000000000000").unwrap(); // 1 ETH in wei
    println!("Original balance: {} wei (1 ETH)", balance);
    
    // 🎯 New feature: Direct operations with primitive types (no explicit conversion needed)
    println!("\n✨ Convenient Operations (no SqlU256::from() needed):");
    
    // Balance doubling - new way vs old way
    let doubled_new = balance * 2;           // 🆕 New way: concise
    let doubled_old = balance * SqlU256::from(2u64); // ❌ Old way: verbose
    println!("  Balance doubled (new): balance * 2 = {}", doubled_new);
    println!("  Balance doubled (old): balance * SqlU256::from(2u64) = {}", doubled_old);
    assert_eq!(doubled_new, doubled_old);
    
    // 🎯 Bidirectional operation support
    println!("\n🔄 Bidirectional Operation Support:");
    let result1 = balance * 3;    // SqlU256 * primitive
    let result2 = 3 * balance;    // primitive * SqlU256
    println!("  balance * 3 = {}", result1);
    println!("  3 * balance = {}", result2);
    assert_eq!(result1, result2);
    
    // 🎯 Common Ethereum calculation scenarios
    println!("\n💰 Real Ethereum Scenarios:");
    
    // 1. Gas fee calculation
    let gas_price = SqlU256::from(20_000_000_000u64); // 20 Gwei
    let gas_limit = 21000u64;
    
    let tx_cost_new = gas_price * gas_limit;  // 🆕 Concise
    let tx_cost_old = gas_price * SqlU256::from(gas_limit); // ❌ Verbose
    println!("  Transaction cost (new): gas_price * {} = {} wei", gas_limit, tx_cost_new);
    println!("  Transaction cost (old): gas_price * SqlU256::from({}) = {} wei", gas_limit, tx_cost_old);
    
    // 2. Percentage calculation
    let fee_rate = 25u64; // 0.25%
    let fee = balance * fee_rate / 10000;  // 🆕 Concise percentage calculation
    println!("  Fee (0.25%): balance * {} / 10000 = {} wei", fee_rate, fee);
    
    // 3. Balance calculation
    let remaining = balance - tx_cost_new;
    println!("  Remaining balance: balance - tx_cost = {} wei", remaining);
    
    // 🎯 All supported types
    println!("\n📝 Supported Primitive Types:");
    let value = SqlU256::from(100u64);
    
    // Unsigned integer types
    println!("  u8:    {} * 2u8 = {}", value, value * 2u8);
    println!("  u16:   {} * 2u16 = {}", value, value * 2u16);
    println!("  u32:   {} * 2u32 = {}", value, value * 2u32);
    println!("  u64:   {} * 2u64 = {}", value, value * 2u64);
    println!("  u128:  {} * 2u128 = {}", value, value * 2u128);
    println!("  usize: {} * 2usize = {}", value, value * 2usize);
    
    // Signed integer types (positive numbers)
    println!("  i32:   {} + 50i32 = {}", value, value + 50i32);
    println!("  i64:   {} + 50i64 = {}", value, value + 50i64);
    
    // Negative number addition (equivalent to subtraction)
    println!("  Negative: {} + (-30i64) = {}", value, value + (-30i64));
    
    // 🎯 Reference types are also supported
    println!("\n🔗 Reference Type Support:");
    let multiplier = 5u64;
    let ref_result1 = value * &multiplier;   // SqlU256 * &primitive
    let ref_result2 = &value * multiplier;   // &SqlU256 * primitive
    let ref_result3 = &value * &multiplier;  // &SqlU256 * &primitive
    
    println!("  value * &multiplier = {}", ref_result1);
    println!("  &value * multiplier = {}", ref_result2);
    println!("  &value * &multiplier = {}", ref_result3);
    assert_eq!(ref_result1, ref_result2);
    assert_eq!(ref_result2, ref_result3);
    
    // 🎯 Chained calculations
    println!("\n⛓️  Chained Calculation Examples:");
    let complex_calc = balance * 2 + 1000000u64 - 500000u64 / 2;
    println!("  balance * 2 + 1000000 - 500000 / 2 = {}", complex_calc);
    
    // 🎯 Real DeFi scenarios
    println!("\n🏦 DeFi Scenario Examples:");
    let staking_amount = SqlU256::from_str("10000000000000000000").unwrap(); // 10 ETH
    let apy_rate = 500u64; // 5% APY (in basis points)
    let days_staked = 365u64;
    
    // Calculate annual rewards
    let annual_reward = staking_amount * apy_rate / 10000;
    let daily_reward = annual_reward / days_staked;
    
    println!("  Staking amount: {} wei (10 ETH)", staking_amount);
    println!("  Annual rate: {}% ({}bp)", apy_rate / 100, apy_rate);
    println!("  Annual reward: {} wei", annual_reward);
    println!("  Daily reward: {} wei", daily_reward);
    
    println!("\n✅ All calculations completed! The new primitive operations greatly simplify the code!");
}
