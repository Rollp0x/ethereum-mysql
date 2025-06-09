//! This module provides convenient macros for creating SqlAddress instances
//! from string literals at compile time, similar to alloy's address! macro.

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
        $crate::SqlAddress::new($crate::alloy::primitives::address!($s))
    };
}


