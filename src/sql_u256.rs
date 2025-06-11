use alloy::primitives::U256;
#[cfg(feature = "serde")]
use serde::Deserialize;
use std::ops::Deref;
use std::str::FromStr;

mod convert;
mod operation;
mod primitive_ops;

/// A SQL-compatible wrapper for 256-bit unsigned integers.
///
/// `SqlU256` wraps `alloy::primitives::U256` and implements all necessary traits
/// for seamless SQLx database integration. It provides full arithmetic operations,
/// type conversions, and consistent hexadecimal storage format across databases.
///
/// # Features
///
/// - **Arithmetic Operations**: Supports +, -, *, /, %, bitwise operations, and more
/// - **Type Conversions**: Convert from/to various integer types with overflow checking
/// - **Database Storage**: Consistent hexadecimal format (0x...) across all databases
/// - **Input Flexibility**: `FromStr` accepts both decimal and hexadecimal strings
/// - **SQLx Integration**: Implements `Type`, `Encode`, and `Decode` for MySQL, PostgreSQL, SQLite
///
/// # Examples
///
/// ```rust
/// use ethereum_mysql::SqlU256;
/// use alloy::primitives::U256;
/// use std::str::FromStr;
///
/// // Create from various types
/// let from_u64 = SqlU256::from(42u64);
/// let from_decimal = SqlU256::from_str("123456789").unwrap();
/// let from_hex = SqlU256::from_str("0x75bcd15").unwrap();
/// let zero = SqlU256::ZERO;
///
/// // Arithmetic operations
/// let a = SqlU256::from(100u64);
/// let b = SqlU256::from(50u64);
/// let sum = a + b;                    // 150
/// let product = a * b;                // 5000
/// let power = a.pow(2);               // 10000
///
/// // Safe operations
/// let checked = a.checked_add(b);     // Some(150)
/// let saturated = a.saturating_sub(SqlU256::from(200u64)); // 0
///
/// // Type conversions
/// let back_to_u256: U256 = from_u64.into();  // SqlU256 -> U256 (always safe)
/// let back_to_u64: u64 = from_u64.try_into().unwrap(); // SqlU256 -> u64 (may overflow)
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SqlU256(U256);

impl SqlU256 {
    /// The zero value constant.
    ///
    /// Equivalent to `SqlU256::from(0u64)` but available as a compile-time constant.
    pub const ZERO: Self = SqlU256(U256::ZERO);

    /// Creates a new `SqlU256` from a `U256` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ethereum_mysql::SqlU256;
    /// use alloy::primitives::U256;
    ///
    /// let u256_val = U256::from(42u64);
    /// let sql_u256 = SqlU256::new(u256_val);
    /// ```
    pub const fn new(value: U256) -> Self {
        SqlU256(value)
    }

    /// Returns a reference to the inner `U256` value.
    ///
    /// This is useful when you need to interact with APIs that expect `U256` directly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ethereum_mysql::SqlU256;
    ///
    /// let sql_u256 = SqlU256::from(42u64);
    /// let inner_ref: &alloy::primitives::U256 = sql_u256.inner();
    /// ```
    pub fn inner(&self) -> &U256 {
        &self.0
    }
}

impl AsRef<U256> for SqlU256 {
    fn as_ref(&self) -> &U256 {
        &self.0
    }
}
impl Deref for SqlU256 {
    type Target = U256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<U256> for SqlU256 {
    fn from(value: U256) -> Self {
        SqlU256(value)
    }
}

impl From<SqlU256> for U256 {
    fn from(value: SqlU256) -> Self {
        value.0
    }
}

impl FromStr for SqlU256 {
    type Err = <U256 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        U256::from_str(s).map(SqlU256)
    }
}

impl std::fmt::Display for SqlU256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SqlU256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// ===============================
// Database Integration - MySQL
// ===============================

/// MySQL database support for SqlU256.
/// 
/// Stores U256 values as VARCHAR in MySQL, using the standard 0x-prefixed hex format.
/// This ensures consistent format and efficient querying of large integer values.
#[cfg(feature = "mysql")]
impl sqlx::Type<sqlx::MySql> for SqlU256 {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <String as sqlx::Type<sqlx::MySql>>::type_info()
    }
    
    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

/// Decodes SqlU256 from MySQL database values.
/// 
/// Handles conversion from VARCHAR/TEXT database fields to SqlU256 instances.
/// Supports both decimal and hexadecimal string formats for backward compatibility.
#[cfg(feature = "mysql")]
impl<'r> sqlx::Decode<'r, sqlx::MySql> for SqlU256 {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = <String as sqlx::Decode<'r, sqlx::MySql>>::decode(value)?;
        SqlU256::from_str(&s)
            .map_err(|e| format!("Failed to parse U256 from string '{}': {:?}", s, e).into())
    }
}

/// Encodes SqlU256 for MySQL database storage.
/// 
/// Converts SqlU256 to hexadecimal string format (0x...) for database insertion/updates.
/// This ensures consistent format across all database operations.
#[cfg(feature = "mysql")]
impl<'q> sqlx::Encode<'q, sqlx::MySql> for SqlU256 {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::MySql as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        // Use consistent hexadecimal format for storage
        let hex_string = format!("{}", self);
        <String as sqlx::Encode<'q, sqlx::MySql>>::encode_by_ref(&hex_string, buf)
    }
}

// ===============================
// Database Integration - PostgreSQL
// ===============================

/// PostgreSQL database support for SqlU256.
/// 
/// Stores U256 values as VARCHAR/TEXT in PostgreSQL using hexadecimal format.
#[cfg(feature = "postgres")]
impl sqlx::Type<sqlx::Postgres> for SqlU256 {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    
    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

/// Decodes SqlU256 from PostgreSQL database values.
#[cfg(feature = "postgres")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for SqlU256 {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = <String as sqlx::Decode<'r, sqlx::Postgres>>::decode(value)?;
        SqlU256::from_str(&s)
            .map_err(|e| format!("Failed to parse U256 from string '{}': {:?}", s, e).into())
    }
}

/// Encodes SqlU256 for PostgreSQL database storage.
#[cfg(feature = "postgres")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for SqlU256 {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let hex_string = format!("{}", self);
        <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&hex_string, buf)
    }
}

// ===============================
// Database Integration - SQLite
// ===============================

/// SQLite database support for SqlU256.
/// 
/// Stores U256 values as TEXT in SQLite using hexadecimal format.
#[cfg(feature = "sqlite")]
impl sqlx::Type<sqlx::Sqlite> for SqlU256 {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
    
    fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

/// Decodes SqlU256 from SQLite database values.
#[cfg(feature = "sqlite")]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for SqlU256 {
    fn decode(
        value: sqlx::sqlite::SqliteValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = <String as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
        SqlU256::from_str(&s)
            .map_err(|e| format!("Failed to parse U256 from string '{}': {:?}", s, e).into())
    }
}

/// Encodes SqlU256 for SQLite database storage.
#[cfg(feature = "sqlite")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for SqlU256 {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let hex_string = format!("{}", self);
        <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&hex_string, buf)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_constants() {
        // Test ZERO constant
        let zero = SqlU256::ZERO;
        assert_eq!(zero, SqlU256::from(0u64));
        
        // Test new() constructor
        let value = SqlU256::new(U256::from(42u64));
        assert_eq!(value, SqlU256::from(42u64));
    }

    #[test]
    fn test_from_conversions() {
        // Test From<U256> for SqlU256
        let u256_val = U256::from(123456789u64);
        let sql_u256 = SqlU256::from(u256_val);
        assert_eq!(sql_u256.inner(), &u256_val);
        
        // Test From<SqlU256> for U256
        let back_to_u256: U256 = sql_u256.into();
        assert_eq!(back_to_u256, u256_val);
    }

    #[test]
    fn test_inner_and_deref() {
        let sql_u256 = SqlU256::from(42u64);
        
        // Test inner() method
        let inner_ref: &U256 = sql_u256.inner();
        assert_eq!(*inner_ref, U256::from(42u64));
        
        // Test AsRef trait
        let as_ref: &U256 = sql_u256.as_ref();
        assert_eq!(*as_ref, U256::from(42u64));
        
        // Test Deref trait (automatic dereferencing)
        let deref_val: U256 = *sql_u256;
        assert_eq!(deref_val, U256::from(42u64));
    }

    #[test]
    fn test_from_str_parsing() {
        // Test decimal string parsing
        let from_decimal = SqlU256::from_str("123456789").unwrap();
        assert_eq!(from_decimal, SqlU256::from(123456789u64));
        
        // Test hexadecimal string parsing
        let from_hex = SqlU256::from_str("0x75bcd15").unwrap();
        assert_eq!(from_hex, SqlU256::from(123456789u64));
        
        // Test that decimal and hex produce same result
        assert_eq!(from_decimal, from_hex);
        
        // Test zero parsing
        let zero_decimal = SqlU256::from_str("0").unwrap();
        let zero_hex = SqlU256::from_str("0x0").unwrap();
        assert_eq!(zero_decimal, zero_hex);
        assert_eq!(zero_decimal, SqlU256::ZERO);
    }

    #[test]
    fn test_from_str_edge_cases() {
        // Test maximum value
        let max_hex = format!("0x{:x}", U256::MAX);
        let max_sql = SqlU256::from_str(&max_hex).unwrap();
        assert_eq!(max_sql.inner(), &U256::MAX);
        
        // Test U256's lenient parsing behavior - these all parse as zero
        let zero_cases = [
            ("", "empty string"),
            ("0", "zero"),
            ("00", "double zero"),  
            ("0x", "just 0x prefix"),
            ("0x0", "0x0"),
            ("0x00", "0x00"),
        ];
        
        for (input, _desc) in zero_cases {
            let result = SqlU256::from_str(input).unwrap();
            assert_eq!(result, SqlU256::ZERO, "Input '{}' should parse as zero", input);
        }
        
        // Test clearly invalid strings that should fail
        assert!(SqlU256::from_str("not_a_number").is_err());
        assert!(SqlU256::from_str("0xnot_hex").is_err());
        assert!(SqlU256::from_str("123abc").is_err());
        assert!(SqlU256::from_str("0x123xyz").is_err());
    }

    #[test]
    fn test_display_formatting() {
        let test_cases = [
            (0u64, "0x0"),
            (255u64, "0xff"),
            (0xdeadbeef_u64, "0xdeadbeef"),
            (123456789u64, "0x75bcd15"),
        ];

        for (input, expected) in test_cases {
            let sql_u256 = SqlU256::from(input);
            let display_str = format!("{}", sql_u256);
            assert_eq!(display_str, expected);
        }
    }

    #[test]
    fn test_round_trip_consistency() {
        let test_values = [
            U256::from(0u64),
            U256::from(1u64),
            U256::from(255u64),
            U256::from(0xdeadbeef_u64),
            U256::from(123456789u64),
            U256::MAX,
        ];

        for original_u256 in test_values {
            let sql_u256 = SqlU256::from(original_u256);
            
            // Test Display -> FromStr round trip
            let display_str = format!("{}", sql_u256);
            let parsed_back = SqlU256::from_str(&display_str).unwrap();
            assert_eq!(sql_u256, parsed_back);
            
            // Test U256 conversion round trip
            let back_to_u256: U256 = sql_u256.into();
            assert_eq!(back_to_u256, original_u256);
        }
    }

    #[test]
    fn test_equality_and_comparison() {
        let a = SqlU256::from(100u64);
        let b = SqlU256::from(100u64);
        let c = SqlU256::from(200u64);
        
        // Test equality
        assert_eq!(a, b);
        assert_ne!(a, c);
        
        // Test with ZERO constant
        let zero1 = SqlU256::ZERO;
        let zero2 = SqlU256::from(0u64);
        assert_eq!(zero1, zero2);
    }

    #[test]
    fn test_clone_and_copy() {
        let original = SqlU256::from(42u64);
        
        // Test Clone
        let cloned = original.clone();
        assert_eq!(original, cloned);
        
        // Test Copy (implicit)
        let copied = original;
        assert_eq!(original, copied);
        
        // Original should still be usable (Copy semantics)
        assert_eq!(original, SqlU256::from(42u64));
    }

    #[test]
    fn test_debug_formatting() {
        let sql_u256 = SqlU256::from(42u64);
        let debug_str = format!("{:?}", sql_u256);
        
        // Should contain the inner U256 value
        assert!(debug_str.contains("SqlU256"));
    }

    #[test]
    fn test_sql_u256_hash() {
        use std::collections::{HashMap, HashSet};
        
        let val1 = SqlU256::from(123u64);
        let val2 = SqlU256::from(123u64);
        let val3 = SqlU256::from(456u64);
        
        // Test Hash trait - equal values should have equal hashes
        use std::hash::{Hash, Hasher, DefaultHasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        let mut hasher3 = DefaultHasher::new();
        
        val1.hash(&mut hasher1);
        val2.hash(&mut hasher2);
        val3.hash(&mut hasher3);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
        assert_ne!(hasher1.finish(), hasher3.finish());
        
        // Test usage in HashSet
        let mut value_set = HashSet::new();
        value_set.insert(val1);
        value_set.insert(val2); // Should not increase size since val1 == val2
        value_set.insert(val3);
        value_set.insert(SqlU256::ZERO);
        
        assert_eq!(value_set.len(), 3);
        assert!(value_set.contains(&val1));
        assert!(value_set.contains(&val2));
        assert!(value_set.contains(&val3));
        assert!(value_set.contains(&SqlU256::ZERO));
        
        // Test usage in HashMap
        let mut value_map = HashMap::new();
        value_map.insert(val1, "First value");
        value_map.insert(val2, "Same value"); // Should overwrite
        value_map.insert(val3, "Different value");
        value_map.insert(SqlU256::ZERO, "Zero value");
        
        assert_eq!(value_map.len(), 3);
        assert_eq!(value_map.get(&val1), Some(&"Same value"));
        assert_eq!(value_map.get(&val2), Some(&"Same value"));
        assert_eq!(value_map.get(&val3), Some(&"Different value"));
        assert_eq!(value_map.get(&SqlU256::ZERO), Some(&"Zero value"));
        
        // Test with large values
        let large1 = SqlU256::from_str("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
        let large2 = SqlU256::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap(); // Same as large1 in decimal
        
        let mut large_hasher1 = DefaultHasher::new();
        let mut large_hasher2 = DefaultHasher::new();
        
        large1.hash(&mut large_hasher1);
        large2.hash(&mut large_hasher2);
        
        assert_eq!(large_hasher1.finish(), large_hasher2.finish());
        assert_eq!(large1, large2);
    }

    #[test]
    fn test_sql_u256_hash_consistency_with_alloy_u256() {
        use std::hash::{Hash, Hasher, DefaultHasher};
        
        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish()
        }
        
        let test_values = [
            "0",
            "42",
            "1000000000000000000",
            "0x75bcd15",
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ];
        
        for value_str in &test_values {
            let alloy_u256 = U256::from_str(value_str).unwrap();
            let sql_u256 = SqlU256::from_str(value_str).unwrap();
            
            let alloy_hash = calculate_hash(&alloy_u256);
            let sql_hash = calculate_hash(&sql_u256);
            
            // Critical: SqlU256 must produce the same hash as the underlying U256
            assert_eq!(alloy_hash, sql_hash, 
                "Hash mismatch for value {}: alloy={}, sql={}", 
                value_str, alloy_hash, sql_hash);
        }
        
        // Test conversion consistency
        let original = U256::from(12345u64);
        let sql_wrapped = SqlU256::from(original);
        let converted_back: U256 = sql_wrapped.into();
        
        assert_eq!(calculate_hash(&original), calculate_hash(&sql_wrapped));
        assert_eq!(calculate_hash(&original), calculate_hash(&converted_back));
        assert_eq!(calculate_hash(&sql_wrapped), calculate_hash(&converted_back));
        
        // Test zero constant consistency
        assert_eq!(calculate_hash(&U256::ZERO), calculate_hash(&SqlU256::ZERO));
        
        // Test maximum value consistency
        let max_alloy = U256::MAX;
        let max_sql = SqlU256::from(max_alloy);
        assert_eq!(calculate_hash(&max_alloy), calculate_hash(&max_sql));
    }
}
