//! This module provides convenient macros for creating SqlAddress and SqlFixedBytes instances
//! from string literals at compile time, similar to alloy's address! and fixed_bytes! macros.

/// Creates a SqlAddress from a hex string literal.
///
/// This macro accepts hex strings with or without the "0x" prefix and creates
/// a SqlAddress at compile time. The input is validated at compile time.
///
/// The macro supports both runtime and const contexts, making it suitable
/// for declaring const SqlAddress values.
///
/// # Examples
///
/// ```
/// use ethereum_mysql::{sqladdress, SqlAddress};
///
/// // With 0x prefix
/// let addr1 = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
///
/// // Without 0x prefix  
/// let addr2 = sqladdress!("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
///
/// // Zero address
/// let zero = sqladdress!("0x0000000000000000000000000000000000000000");
///
/// // Const context
/// const ADMIN: SqlAddress = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
/// ```
#[macro_export]
macro_rules! sqladdress {
    ($s:expr) => {
        $crate::SqlAddress::new_from_address($crate::alloy::primitives::address!($s))
    };
}

/// Macro to create a `SqlFixedBytes<N>` from a hex string literal at compile time.
///
/// Usage:
/// ```
/// use ethereum_mysql::{sqlhash, SqlFixedBytes};
/// const HASH: SqlFixedBytes<32> = sqlhash!(32, "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
/// const SHORT: SqlFixedBytes<4> = sqlhash!(4, "0x12345678");
/// ```
///
/// N 必须是字面量（literal），与 hex 长度匹配，否则编译报错。
#[macro_export]
macro_rules! sqlhash {
    ($n:literal, $s:literal) => {{
        $crate::SqlFixedBytes::<$n>::from_bytes($crate::alloy::primitives::fixed_bytes!($s))
    }};
}
/// Macro to create a SqlU256 from a literal (compile-time check for negative, only usable in runtime context).
///
/// Usage:
/// let a: SqlU256 = sqlu256!(100); // OK
/// let b: SqlU256 = sqlu256!(-100); // Compile error
/// // const A: SqlU256 = sqlu256!(100); // ❌ Not supported: `From<u128>` is not const
#[macro_export]
macro_rules! sqlu256 {
    ($val:literal) => {{
        const _: () = assert!($val >= 0, "SqlU256 cannot be negative at compile time");
        $crate::SqlU256::from($val as u128)
    }};
}

#[cfg(test)]
mod tests {
    use crate::SqlHash;
    use alloy::primitives::hex;

    #[test]
    fn test_sqlhash_const_and_runtime() {
        // Const context
        const TRANSFER_EVENT_SIGNATURE: SqlHash = sqlhash!(
            32,
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
        // Runtime context
        let runtime_hash: SqlHash = sqlhash!(
            32,
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        );
        // Both should be equal
        assert_eq!(TRANSFER_EVENT_SIGNATURE, runtime_hash);
        // Should match expected bytes
        let expected =
            hex::decode("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
                .unwrap();
        assert_eq!(TRANSFER_EVENT_SIGNATURE.as_slice(), expected.as_slice());
        // Shorter length test
        const SHORT: crate::SqlFixedBytes<4> = sqlhash!(4, "0x095ea7b3");
        let short_expected = hex::decode("095ea7b3").unwrap();
        assert_eq!(SHORT.as_slice(), short_expected.as_slice());
    }
    #[test]
    fn test_sqlu256_runtime() {
        // Runtime context only
        let runtime_amount: crate::SqlU256 = sqlu256!(12345678901234567890u128);
        use alloy::primitives::U256;
        let expected = U256::from(12345678901234567890u128);
        assert_eq!(*runtime_amount, expected);
    }
}
