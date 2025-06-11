use std::ops::Deref;
use std::str::FromStr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use alloy::primitives::Address;

/// SQL-compatible wrapper for Ethereum Address.
/// 
/// This type wraps `alloy::primitives::Address` and provides seamless integration
/// with SQL databases through SQLx. It supports MySQL, PostgreSQL, and SQLite,
/// storing addresses as VARCHAR/TEXT in the database using the standard hex format (0x-prefixed).
///
/// # Database Support
/// 
/// - **MySQL**: Enable with `mysql` feature
/// - **PostgreSQL**: Enable with `postgres` feature  
/// - **SQLite**: Enable with `sqlite` feature
///
/// # Examples
///
/// ```no_run
/// use ethereum_mysql::SqlAddress;
/// use alloy::primitives::Address;
/// use std::str::FromStr;
///
/// // Create from raw Address
/// let addr = Address::ZERO;
/// let sql_addr = SqlAddress::from(addr);
///
/// // Create from string
/// let sql_addr = SqlAddress::from_str("0x0000000000000000000000000000000000000000").unwrap();
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SqlAddress(Address);

impl SqlAddress {
    /// The zero address (0x0000000000000000000000000000000000000000)
    /// 
    /// This constant represents the Ethereum zero address, commonly used as a null value
    /// or burn address in smart contracts. It's available at compile time.
    pub const ZERO: Self = SqlAddress(Address::ZERO);

    /// Creates a new SqlAddress from an alloy Address.
    /// 
    /// This is a const function that can be used in const contexts and static declarations.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ethereum_mysql::SqlAddress;
    /// use alloy::primitives::Address;
    /// 
    /// const MY_ADDRESS: SqlAddress = SqlAddress::new(Address::ZERO);
    /// ```
    pub const fn new(address: Address) -> Self {
        SqlAddress(address)
    }

    /// Returns the inner alloy Address.
    /// 
    /// This method provides access to the underlying `alloy::primitives::Address`
    /// for use with other Ethereum libraries or blockchain RPC calls.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ethereum_mysql::SqlAddress;
    /// use alloy::primitives::Address;
    /// use std::str::FromStr;
    /// 
    /// let sql_addr = SqlAddress::new(Address::ZERO);
    /// let inner: Address = sql_addr.inner();
    /// ```
    pub fn inner(&self) -> Address {
        self.0
    }

}

impl AsRef<Address> for SqlAddress {
    /// Returns a reference to the inner Address.
    fn as_ref(&self) -> &Address {
        &self.0
    }
}

impl Deref for SqlAddress {
    type Target = Address;

    /// Dereferences to the inner Address, allowing direct access to Address methods.
    /// 
    /// This enables calling any method available on `alloy::primitives::Address`
    /// directly on a `SqlAddress` instance.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}   

impl From<Address> for SqlAddress {
    /// Creates a SqlAddress from an alloy Address.
    fn from(address: Address) -> Self {
        SqlAddress(address)
    }
}

impl From<SqlAddress> for Address {
    /// Extracts the inner Address from a SqlAddress.
    fn from(sql_address: SqlAddress) -> Self {
        sql_address.0
    }
}

impl FromStr for SqlAddress {
    type Err = alloy::primitives::AddressError;

    /// Parses a string into a SqlAddress.
    /// 
    /// Supports various formats:
    /// - With 0x prefix: "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"
    /// - Without prefix: "742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"
    /// - Mixed case (checksummed) and lowercase formats
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SqlAddress(s.parse()?))
    }
}

impl std::fmt::Display for SqlAddress {
    /// Formats the address for display using EIP-55 checksum format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ===============================
// Database Integration - MySQL
// ===============================

/// MySQL database support for SqlAddress.
/// 
/// Stores addresses as VARCHAR(42) in MySQL, using the standard 0x-prefixed hex format.
/// This allows for efficient indexing and querying of Ethereum addresses.
#[cfg(feature = "mysql")]
impl sqlx::Type<sqlx::MySql> for SqlAddress {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <String as sqlx::Type<sqlx::MySql>>::type_info()
    }
    
    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

/// Decodes SqlAddress from MySQL database values.
/// 
/// Handles conversion from VARCHAR/TEXT database fields to SqlAddress instances.
#[cfg(feature = "mysql")]
impl<'r> sqlx::Decode<'r, sqlx::MySql> for SqlAddress {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = <String as sqlx::Decode<'r, sqlx::MySql>>::decode(value)?;
        SqlAddress::from_str(&s)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Sync + Send>)
    }
}

/// Encodes SqlAddress for MySQL database storage.
/// 
/// Converts SqlAddress to string format for database insertion/updates.
#[cfg(feature = "mysql")]
impl<'q> sqlx::Encode<'q, sqlx::MySql> for SqlAddress {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::MySql as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <String as sqlx::Encode<'q, sqlx::MySql>>::encode_by_ref(&self.0.to_string(), buf)
    }
}

// ===============================
// Database Integration - PostgreSQL
// ===============================

/// PostgreSQL database support for SqlAddress.
/// 
/// Stores addresses as VARCHAR(42) or TEXT in PostgreSQL, using the standard 0x-prefixed hex format.
/// PostgreSQL's case-sensitive string handling works well with Ethereum's checksummed addresses.
#[cfg(feature = "postgres")]
impl sqlx::Type<sqlx::Postgres> for SqlAddress {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

/// Decodes SqlAddress from PostgreSQL database values.
/// 
/// Handles conversion from VARCHAR/TEXT database fields to SqlAddress instances.
#[cfg(feature = "postgres")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for SqlAddress {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = <String as sqlx::Decode<'r, sqlx::Postgres>>::decode(value)?;
        SqlAddress::from_str(&s)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Sync + Send>)
    }
}

/// Encodes SqlAddress for PostgreSQL database storage.
/// 
/// Converts SqlAddress to string format for database insertion/updates.
#[cfg(feature = "postgres")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for SqlAddress {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.0.to_string(), buf)
    }
}

// ===============================
// Database Integration - SQLite
// ===============================

/// SQLite database support for SqlAddress.
/// 
/// Stores addresses as TEXT in SQLite, using the standard 0x-prefixed hex format.
/// SQLite's flexible typing system handles address strings efficiently.
#[cfg(feature = "sqlite")]
impl sqlx::Type<sqlx::Sqlite> for SqlAddress {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
    
    fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

/// Decodes SqlAddress from SQLite database values.
/// 
/// Handles conversion from TEXT database fields to SqlAddress instances.
#[cfg(feature = "sqlite")]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for SqlAddress {
    fn decode(
        value: sqlx::sqlite::SqliteValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = <String as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
        SqlAddress::from_str(&s)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Sync + Send>)
    }
}

/// Encodes SqlAddress for SQLite database storage.
/// 
/// Converts SqlAddress to string format for database insertion/updates.
#[cfg(feature = "sqlite")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for SqlAddress {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&self.0.to_string(), buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use crate::sqladdress;
    use std::str::FromStr;

    const TEST_ADDRESS_STR: &str = "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d";
    const ZERO_ADDRESS_STR: &str = "0x0000000000000000000000000000000000000000";

    #[test]
    fn test_sql_address_creation() {
        // Create from Address
        let addr = Address::ZERO;
        let sql_addr = SqlAddress::new(addr);
        assert_eq!(sql_addr.inner(), addr);

        // Create from string
        let sql_addr = SqlAddress::from_str(ZERO_ADDRESS_STR).unwrap();
        assert_eq!(sql_addr.inner(), Address::ZERO);
    }

    #[test]
    fn test_sql_address_conversions() {
        let original_addr = TEST_ADDRESS_STR.parse::<Address>().unwrap();
        
        // Address -> SqlAddress -> Address
        let sql_addr = SqlAddress::from(original_addr);
        let converted_back: Address = sql_addr.into();
        assert_eq!(original_addr, converted_back);

        // String -> SqlAddress -> String (Note: alloy Address uses checksum format)
        let sql_addr = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        let result_str = sql_addr.to_string();
        assert_eq!(result_str.to_lowercase(), TEST_ADDRESS_STR.to_lowercase());
    }

    #[test]
    fn test_sql_address_display() {
        let sql_addr = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        let displayed = format!("{}", sql_addr);
        // alloy Address uses checksum format, so we compare lowercase versions
        assert_eq!(displayed.to_lowercase(), TEST_ADDRESS_STR.to_lowercase());
    }

    #[test]
    fn test_sql_address_deref() {
        let sql_addr = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        
        // Test Deref trait
        let _checksum = sql_addr.to_checksum(None);
        
        // Test AsRef trait
        let addr_ref: &Address = sql_addr.as_ref();
        assert_eq!(*addr_ref, sql_addr.inner());
    }

    #[test]
    fn test_sql_address_equality() {
        let addr1 = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        let addr2 = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        let addr3 = SqlAddress::from_str(ZERO_ADDRESS_STR).unwrap();

        assert_eq!(addr1, addr2);
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn test_sql_address_debug() {
        let sql_addr = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        let debug_str = format!("{:?}", sql_addr);
        assert!(debug_str.contains("SqlAddress"));
    }

    #[test]
    fn test_invalid_address() {
        let invalid_addresses = vec![
            "invalid",
            "0x123", // Too short
            "0xgg42d35Cc6635C0532925a3b8D42cC72b5c2A9A1d", // Contains invalid characters
            "", // Empty string
        ];

        for invalid_addr in invalid_addresses {
            assert!(SqlAddress::from_str(invalid_addr).is_err());
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_sql_address_serde() {
        let sql_addr = SqlAddress::from_str(TEST_ADDRESS_STR).unwrap();
        
        // Serialize
        let serialized = serde_json::to_string(&sql_addr).unwrap();
        assert!(serialized.contains(TEST_ADDRESS_STR.to_lowercase().trim_start_matches("0x")));
        
        // Deserialize
        let deserialized: SqlAddress = serde_json::from_str(&serialized).unwrap();
        assert_eq!(sql_addr, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_sql_address_serde_with_various_formats() {
        // Test different input formats
        let test_cases = vec![
            // Standard format
            (TEST_ADDRESS_STR, true),
            // Without 0x prefix
            ("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d", true),
            // All lowercase
            ("0x742d35cc6635c0532925a3b8d42cc72b5c2a9a1d", true),
            // All uppercase  
            ("0x742D35CC6635C0532925A3B8D42CC72B5C2A9A1D", true),
            // Invalid format
            ("invalid", false),
        ];

        for (addr_str, should_succeed) in test_cases {
            let result = SqlAddress::from_str(addr_str);
            assert_eq!(result.is_ok(), should_succeed, "Failed for address: {}", addr_str);
            
            if should_succeed {
                let sql_addr = result.unwrap();
                let serialized = serde_json::to_string(&sql_addr).unwrap();
                let deserialized: SqlAddress = serde_json::from_str(&serialized).unwrap();
                assert_eq!(sql_addr, deserialized);
            }
        }
    }

    #[test]
    fn test_sqladdress_macro() {
        // Test address with 0x prefix
        let addr1 = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
        let addr_from_str = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();
        assert_eq!(addr1, addr_from_str);

        // Test address without 0x prefix
        let addr2 = sqladdress!("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
        assert_eq!(addr2, addr_from_str);

        // Test zero address
        let zero_addr = sqladdress!("0x0000000000000000000000000000000000000000");
        let zero_from_str = SqlAddress::from_str(ZERO_ADDRESS_STR).unwrap();
        assert_eq!(zero_addr, zero_from_str);

        // Test that macro-created addresses work with all methods
        let addr = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
        let _inner = addr.inner();
        let _string = addr.to_string();
        let _display = format!("{}", addr);
        let _debug = format!("{:?}", addr);
    }

    #[test]
    fn test_sqladdress_macro_compile_time_validation() {
        // These are validated at compile time
        let _valid_addresses = [
            sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
            sqladdress!("742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
            sqladdress!("0x0000000000000000000000000000000000000000"),
            sqladdress!("0xffffffffffffffffffffffffffffffffffffffff"),
        ];
        
        // Verify they are all valid
        for addr in _valid_addresses.iter() {
            assert_ne!(*addr, SqlAddress::from_str("0x1111111111111111111111111111111111111111").unwrap());
        }
    }

    #[test]
    fn test_sql_address_zero_constant() {
        // Test ZERO constant
        assert_eq!(SqlAddress::ZERO.to_string(), "0x0000000000000000000000000000000000000000");
        
        // Verify ZERO constant equals other creation methods
        let zero_from_str = SqlAddress::from_str("0x0000000000000000000000000000000000000000").unwrap();
        let zero_from_macro = sqladdress!("0x0000000000000000000000000000000000000000");
        let zero_from_alloy = SqlAddress::new(Address::ZERO);
        
        assert_eq!(SqlAddress::ZERO, zero_from_str);
        assert_eq!(SqlAddress::ZERO, zero_from_macro);
        assert_eq!(SqlAddress::ZERO, zero_from_alloy);
        
        // Verify other properties of ZERO constant
        assert_eq!(SqlAddress::ZERO.inner(), Address::ZERO);
        assert_eq!(*SqlAddress::ZERO, Address::ZERO);
        
        // Verify it works in different contexts
        const ZERO_CONST: SqlAddress = SqlAddress::ZERO;
        assert_eq!(ZERO_CONST, SqlAddress::ZERO);
    }

    #[test]
    fn test_sql_address_hash() {
        use std::collections::{HashMap, HashSet};
        
        let addr1 = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
        let addr2 = sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d");
        let addr3 = sqladdress!("0x1234567890123456789012345678901234567890");
        
        // Test Hash trait - equal addresses should have equal hashes
        use std::hash::{Hash, Hasher, DefaultHasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();
        
        addr1.hash(&mut hasher1);
        addr2.hash(&mut hasher2);
        addr3.hash(&mut hasher3);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
        assert_ne!(hasher1.finish(), hasher3.finish());
        
        // Test usage in HashSet
        let mut address_set = HashSet::new();
        address_set.insert(addr1);
        address_set.insert(addr2); // Should not increase size since addr1 == addr2
        address_set.insert(addr3);
        
        assert_eq!(address_set.len(), 2);
        assert!(address_set.contains(&addr1));
        assert!(address_set.contains(&addr2));
        assert!(address_set.contains(&addr3));
        
        // Test usage in HashMap
        let mut address_map = HashMap::new();
        address_map.insert(addr1, "First address");
        address_map.insert(addr2, "Same address"); // Should overwrite
        address_map.insert(addr3, "Different address");
        
        assert_eq!(address_map.len(), 2);
        assert_eq!(address_map.get(&addr1), Some(&"Same address"));
        assert_eq!(address_map.get(&addr2), Some(&"Same address"));
        assert_eq!(address_map.get(&addr3), Some(&"Different address"));
    }

    #[test]
    fn test_sql_address_hash_consistency_with_alloy_address() {
        use std::hash::{Hash, Hasher, DefaultHasher};
        
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish()
        }
        
        let test_addresses = [
            "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d",
            "0x0000000000000000000000000000000000000000", 
            "0xffffffffffffffffffffffffffffffffffffffff",
            "0x1234567890123456789012345678901234567890",
        ];
        
        for addr_str in &test_addresses {
            let alloy_addr = Address::from_str(addr_str).unwrap();
            let sql_addr = SqlAddress::from_str(addr_str).unwrap();
            
            let alloy_hash = calculate_hash(&alloy_addr);
            let sql_hash = calculate_hash(&sql_addr);
            
            // Critical: SqlAddress must produce the same hash as the underlying Address
            assert_eq!(alloy_hash, sql_hash, 
                "Hash mismatch for address {}: alloy={}, sql={}", 
                addr_str, alloy_hash, sql_hash);
        }
        
        // Test conversion consistency
        let original = Address::from_str(TEST_ADDRESS_STR).unwrap();
        let sql_wrapped = SqlAddress::from(original);
        let converted_back: Address = sql_wrapped.into();
        
        assert_eq!(calculate_hash(&original), calculate_hash(&sql_wrapped));
        assert_eq!(calculate_hash(&original), calculate_hash(&converted_back));
        assert_eq!(calculate_hash(&sql_wrapped), calculate_hash(&converted_back));
        
        // Test zero address consistency
        assert_eq!(calculate_hash(&Address::ZERO), calculate_hash(&SqlAddress::ZERO));
    }
}