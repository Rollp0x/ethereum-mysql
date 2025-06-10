//! Integration tests for SqlU256 serde functionality.
//! 
//! These tests verify that SqlU256 serialization and deserialization works correctly
//! with serde, particularly focusing on format consistency and compatibility.

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use ethereum_mysql::SqlU256;
    use alloy::primitives::U256;
    use std::str::FromStr;

    #[test]
    fn test_serde_serialization_deserialization() {
        let original_value = SqlU256::from(U256::from(123456789_u64));
        
        // Test serialization
        let serialized = serde_json::to_string(&original_value).unwrap();
        
        // Test deserialization from serialized result
        let deserialized: SqlU256 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_value, deserialized);
    }

    #[test]
    fn test_serde_decimal_and_hex_input_compatibility() {
        let expected = SqlU256::from(U256::from(123456789_u64));
        
        // Test deserialization from decimal JSON string
        let decimal_json = "\"123456789\"";
        let from_decimal: SqlU256 = serde_json::from_str(decimal_json).unwrap();
        assert_eq!(expected, from_decimal);
        
        // Test deserialization from hexadecimal JSON string
        let hex_json = "\"0x75bcd15\"";
        let from_hex: SqlU256 = serde_json::from_str(hex_json).unwrap();
        assert_eq!(expected, from_hex);
        
        // Both should produce the same result
        assert_eq!(from_decimal, from_hex);
    }

    #[test]
    fn test_serde_format_consistency() {
        let test_values = [
            U256::from(0_u64),
            U256::from(255_u64),
            U256::from(0xdeadbeef_u64),
            U256::from(123456789_u64),
            U256::MAX,
        ];

        for value in test_values {
            let sql_u256 = SqlU256::from(value);
            
            // Test Display format (used by database storage)
            let display_format = format!("{}", sql_u256);
            
            // Test JSON serialization format
            let json_format = serde_json::to_string(&sql_u256).unwrap();
            let json_value: String = serde_json::from_str(&json_format).unwrap();
            
            // Both formats should be parseable back to the same value
            let from_display = SqlU256::from_str(&display_format).unwrap();
            let from_json = SqlU256::from_str(&json_value).unwrap();
            
            assert_eq!(sql_u256, from_display);
            assert_eq!(sql_u256, from_json);
            assert_eq!(from_display, from_json);
        }
    }

    #[test]
    fn test_serde_round_trip_consistency() {
        let original = SqlU256::from(U256::from(0xdeadbeef_u64));
        
        // JSON serialization round trip
        let json_str = serde_json::to_string(&original).unwrap();
        let from_json: SqlU256 = serde_json::from_str(&json_str).unwrap();
        let json_str2 = serde_json::to_string(&from_json).unwrap();
        
        assert_eq!(original, from_json);
        assert_eq!(json_str, json_str2);
    }

    #[test]
    fn test_serde_error_handling() {
        // Test clearly invalid JSON strings that should always fail
        let invalid_cases = [
            "\"invalid_text\"",        // clearly invalid number
            "\"0xinvalid_hex\"",      // invalid hex digits
            "null",                    // null value
            "\"123abc456\"",          // mixed valid/invalid characters
        ];
        
        for invalid_json in invalid_cases {
            let result: Result<SqlU256, _> = serde_json::from_str(invalid_json);
            assert!(result.is_err(), "Expected error for input: {}", invalid_json);
        }
        
        // Note: U256's serde implementation may accept various formats including:
        // - Raw numbers (42)
        // - Empty strings (may parse as 0)
        // - Decimal strings ("123")
        // - Hex strings ("0x7b")
        // This flexibility is by design for compatibility
    }

    #[test]
    fn test_serde_struct_integration() {
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestStruct {
            id: u64,
            balance: SqlU256,
            optional_amount: Option<SqlU256>,
        }
        
        let original = TestStruct {
            id: 1,
            balance: SqlU256::from_str("1000000000000000000").unwrap(), // 1 ETH in wei
            optional_amount: Some(SqlU256::from_str("0xde0b6b3a7640000").unwrap()),
        };
        
        let json = serde_json::to_string(&original).unwrap();
        let parsed: TestStruct = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_serde_mixed_format_parsing() {
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct MixedFormatTest {
            decimal_amount: SqlU256,
            hex_amount: SqlU256,
        }
        
        // JSON with mixed decimal and hex formats
        let mixed_json = r#"{
            "decimal_amount": "1500000000000000000",
            "hex_amount": "0x14d1120d7b160000"
        }"#;
        
        let parsed: MixedFormatTest = serde_json::from_str(mixed_json).unwrap();
        
        // Both should represent the same value (1.5 ETH in wei)
        assert_eq!(parsed.decimal_amount, parsed.hex_amount);
        
        let expected_value = SqlU256::from_str("1500000000000000000").unwrap();
        assert_eq!(parsed.decimal_amount, expected_value);
        assert_eq!(parsed.hex_amount, expected_value);
    }
}
