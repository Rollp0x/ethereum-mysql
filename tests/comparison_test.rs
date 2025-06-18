use ethereum_mysql::{SqlU256, SqlAddress};
use std::str::FromStr;

#[test]
fn test_sql_u256_comparison() {
    let a = SqlU256::from(100u64);
    let b = SqlU256::from(200u64);
    let c = SqlU256::from(100u64);

    // Test ordering
    assert!(a < b);
    assert!(b > a);
    assert!(a <= b);
    assert!(b >= a);
    assert!(a <= c);
    assert!(a >= c);
    assert!(!(a > b));
    assert!(!(b < a));

    // Test with zero
    let zero = SqlU256::ZERO;
    assert!(zero < a);
    assert!(a > zero);
    assert!(zero <= a);
    assert!(a >= zero);

    // Test with large numbers
    let large = SqlU256::from_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let medium = SqlU256::from_str("0x123456789abcdef").unwrap();
    
    assert!(medium < large);
    assert!(large > medium);
    assert!(zero < medium);
    assert!(medium > zero);
}

#[test]
fn test_sql_address_comparison() {
    // Create addresses with different values for comparison
    let addr1 = SqlAddress::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let addr2 = SqlAddress::from_str("0x0000000000000000000000000000000000000002").unwrap();
    let addr3 = SqlAddress::from_str("0x0000000000000000000000000000000000000001").unwrap();

    // Test ordering
    assert!(addr1 < addr2);
    assert!(addr2 > addr1);
    assert!(addr1 <= addr2);
    assert!(addr2 >= addr1);
    assert!(addr1 <= addr3);
    assert!(addr1 >= addr3);
    assert!(!(addr1 > addr2));
    assert!(!(addr2 < addr1));

    // Test with zero address
    let zero = SqlAddress::ZERO;
    assert!(zero < addr1);
    assert!(addr1 > zero);
    assert!(zero <= addr1);
    assert!(addr1 >= zero);

    // Test lexicographic ordering (addresses are compared as byte arrays)
    let addr_a = SqlAddress::from_str("0xa000000000000000000000000000000000000000").unwrap();
    let addr_b = SqlAddress::from_str("0xb000000000000000000000000000000000000000").unwrap();
    
    assert!(addr_a < addr_b);
    assert!(addr_b > addr_a);
}

#[test]
fn test_uniswap_token_ordering() {
    // Simulate UniswapV2 token ordering scenario
    let token_a = SqlAddress::from_str("0x6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(); // DAI
    let token_b = SqlAddress::from_str("0xA0b86a33E6441E85C7A4d8E4d80D6Bb4BF5BF2Aa").unwrap(); // UNI
    
    // In UniswapV2, token0 < token1 (smaller address becomes token0)
    let (token0, token1) = if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };
    
    assert!(token0 < token1);
    
    // Verify the ordering is consistent
    assert_eq!(token0, token_a); // DAI has smaller address
    assert_eq!(token1, token_b); // UNI has larger address
}

#[test]
fn test_sorting() {
    // Test sorting collections of SqlU256
    let mut numbers = vec![
        SqlU256::from(300u64),
        SqlU256::from(100u64),
        SqlU256::from(200u64),
        SqlU256::ZERO,
    ];
    
    numbers.sort();
    
    assert_eq!(numbers[0], SqlU256::ZERO);
    assert_eq!(numbers[1], SqlU256::from(100u64));
    assert_eq!(numbers[2], SqlU256::from(200u64));
    assert_eq!(numbers[3], SqlU256::from(300u64));

    // Test sorting collections of SqlAddress
    let mut addresses = vec![
        SqlAddress::from_str("0x0000000000000000000000000000000000000003").unwrap(),
        SqlAddress::from_str("0x0000000000000000000000000000000000000001").unwrap(),
        SqlAddress::from_str("0x0000000000000000000000000000000000000002").unwrap(),
        SqlAddress::ZERO,
    ];
    
    addresses.sort();
    
    assert_eq!(addresses[0], SqlAddress::ZERO);
    assert_eq!(addresses[1], SqlAddress::from_str("0x0000000000000000000000000000000000000001").unwrap());
    assert_eq!(addresses[2], SqlAddress::from_str("0x0000000000000000000000000000000000000002").unwrap());
    assert_eq!(addresses[3], SqlAddress::from_str("0x0000000000000000000000000000000000000003").unwrap());
}

#[test]
fn test_min_max() {
    let a = SqlU256::from(100u64);
    let b = SqlU256::from(200u64);
    
    assert_eq!(a.min(b), a);
    assert_eq!(a.max(b), b);
    assert_eq!(b.min(a), a);
    assert_eq!(b.max(a), b);

    let addr1 = SqlAddress::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let addr2 = SqlAddress::from_str("0x0000000000000000000000000000000000000002").unwrap();
    
    assert_eq!(addr1.min(addr2), addr1);
    assert_eq!(addr1.max(addr2), addr2);
    assert_eq!(addr2.min(addr1), addr1);
    assert_eq!(addr2.max(addr1), addr2);
}
