// #[cfg(feature = "sqlx_str")]
// use std::str::FromStr;

// Basic SQLite tests (in-memory database, no external services required)
#[cfg(feature = "sqlx_str")]
mod sqlite_tests {
    use ethereum_mysql::{SqlU256,U256};
    use sqlx::{Row, SqlitePool};

    #[tokio::test]
    async fn test_sqlite_basic_u256_operations() {
        // Create in-memory database
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS test_balances")
            .execute(&pool)
            .await
            .unwrap();
        // Create test table
        sqlx::query(
            "CREATE TABLE test_balances (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                amount VARCHAR(66) NOT NULL,
                description TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create test table");

        // Test inserting U256 value
        let test_amount = SqlU256::from(U256::from(123456789_u64));

        sqlx::query("INSERT INTO test_balances (amount, description) VALUES (?, ?)")
            .bind(&test_amount)
            .bind("Test Balance")
            .execute(&pool)
            .await
            .expect("Failed to insert U256 amount");

        // Test querying U256 value
        let row = sqlx::query("SELECT amount, description FROM test_balances WHERE id = 1")
            .fetch_one(&pool)
            .await
            .expect("Failed to select U256 amount");

        let retrieved_amount: SqlU256 = row.get("amount");
        let description: String = row.get("description");

        assert_eq!(retrieved_amount, test_amount);
        assert_eq!(description, "Test Balance");
        println!(
            "✅ Basic U256 operation: {} stored and retrieved correctly",
            retrieved_amount
        );
    }

    #[tokio::test]
    async fn test_sqlite_zero_and_max_u256() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS test_balances")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE test_balances (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                amount VARCHAR(66) NOT NULL,
                description TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create test table");

        // Test extreme values
        let test_values = [
            (SqlU256::from(U256::ZERO), "Zero Value"),
            (SqlU256::from(U256::from(1_u64)), "Minimum Value"),
            (
                SqlU256::from(U256::from(0xdeadbeef_u64)),
                "Common Hex Value",
            ),
            (SqlU256::from(U256::MAX), "Maximum Value"),
        ];

        // Insert all test values
        for (amount, desc) in &test_values {
            sqlx::query("INSERT INTO test_balances (amount, description) VALUES (?, ?)")
                .bind(amount)
                .bind(*desc)
                .execute(&pool)
                .await
                .expect("Failed to insert U256 value");
        }

        // Verify they can be queried back
        let rows = sqlx::query("SELECT amount, description FROM test_balances ORDER BY id")
            .fetch_all(&pool)
            .await
            .expect("Failed to select U256 values");

        assert_eq!(rows.len(), test_values.len());

        for (i, row) in rows.iter().enumerate() {
            let retrieved_amount: SqlU256 = row.get("amount");
            let description: String = row.get("description");

            assert_eq!(retrieved_amount, test_values[i].0);
            assert_eq!(description, test_values[i].1);
            println!(
                "✅ {}: {} = {}",
                description, test_values[i].0, retrieved_amount
            );
        }
    }

    #[tokio::test]
    async fn test_sqlite_u256_comparisons() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS user_balances")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE user_balances (
                user_id INTEGER,
                balance VARCHAR(66) NOT NULL,
                is_vip BOOLEAN DEFAULT FALSE
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create user_balances table");

        // Simulate user balance scenario
        let test_data = [
            (1, U256::from(1000_u64), false),
            (2, U256::from(50000_u64), true),
            (3, U256::ZERO, false),
            (4, U256::from(0xdeadbeef_u64), true), // Large balance
            (5, U256::MAX, true),                  // Ultra whale
        ];

        for (user_id, balance, is_vip) in &test_data {
            let sql_balance = SqlU256::from(*balance);
            sqlx::query("INSERT INTO user_balances (user_id, balance, is_vip) VALUES (?, ?, ?)")
                .bind(user_id)
                .bind(&sql_balance)
                .bind(is_vip)
                .execute(&pool)
                .await
                .expect("Failed to insert user balance");
        }

        // Query users with high balances (using string comparison - works with hex format)
        let high_balance_threshold = SqlU256::from(U256::from(10000_u64));
        let high_balance_users = sqlx::query(
            "SELECT user_id, balance FROM user_balances WHERE balance > ? ORDER BY user_id",
        )
        .bind(&high_balance_threshold)
        .fetch_all(&pool)
        .await
        .expect("Failed to select high balance users");

        // Note: String comparison might not work as expected with hex values
        // This test demonstrates the limitation
        println!("High balance users found: {}", high_balance_users.len());
        for row in high_balance_users {
            let user_id: i32 = row.get("user_id");
            let balance: SqlU256 = row.get("balance");
            println!("User {}: {}", user_id, balance);
        }

        // Query zero balance users
        let zero_balance_users =
            sqlx::query("SELECT user_id FROM user_balances WHERE balance = ? ORDER BY user_id")
                .bind(&SqlU256::from(U256::ZERO))
                .fetch_all(&pool)
                .await
                .expect("Failed to select zero balance users");

        assert_eq!(zero_balance_users.len(), 1);
        let zero_user: i32 = zero_balance_users[0].get("user_id");
        assert_eq!(zero_user, 3);
    }

    #[tokio::test]
    async fn test_sqlite_u256_aggregations() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS transactions")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_address BINARY(20) NOT NULL,
                to_address BINARY(20) NOT NULL,
                amount VARCHAR(66) NOT NULL,
                transaction_type TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create transactions table");

        // Insert sample transactions
        let transactions = [
            ("0x1111", "0x2222", U256::from(1000_u64), "transfer"),
            ("0x2222", "0x3333", U256::from(2000_u64), "transfer"),
            ("0x1111", "0x3333", U256::from(500_u64), "transfer"),
            ("0x0000", "0x1111", U256::from(10000_u64), "mint"),
            ("0x2222", "0x0000", U256::from(100_u64), "burn"),
        ];

        for (from_addr, to_addr, amount, tx_type) in &transactions {
            let sql_amount = SqlU256::from(*amount);
            sqlx::query(
                "INSERT INTO transactions (from_address, to_address, amount, transaction_type) 
                 VALUES (?, ?, ?, ?)",
            )
            .bind(*from_addr)
            .bind(*to_addr)
            .bind(&sql_amount)
            .bind(*tx_type)
            .execute(&pool)
            .await
            .expect("Failed to insert transaction");
        }

        // Count transactions by type
        let tx_counts = sqlx::query(
            "SELECT transaction_type, COUNT(*) as count 
             FROM transactions 
             GROUP BY transaction_type 
             ORDER BY transaction_type",
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to count transactions by type");

        println!("Transaction counts by type:");
        for row in tx_counts {
            let tx_type: String = row.get("transaction_type");
            let count: i64 = row.get("count");
            println!("  {}: {}", tx_type, count);
        }

        // Find largest transaction
        let largest_tx =
            sqlx::query("SELECT amount FROM transactions ORDER BY amount DESC LIMIT 1")
                .fetch_one(&pool)
                .await
                .expect("Failed to find largest transaction");

        let largest_amount: SqlU256 = largest_tx.get("amount");
        println!("Largest transaction: {}", largest_amount);
    }
}

// MySQL integration tests (requires running MySQL server)
#[cfg(feature = "sqlx_str")]
mod mysql_tests {
    use ethereum_mysql::{SqlU256,U256};
    use sqlx::{MySqlPool, Row};

    // Helper function: setup MySQL connection and test table
    async fn setup_mysql_test(table_suffix: &str) -> Option<MySqlPool> {
        // Try to connect to local MySQL, skip test if it fails
        let database_url = std::env::var("MYSQL_DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:123456@localhost:3306/test_db".to_string());

        match MySqlPool::connect(&database_url).await {
            Ok(pool) => {
                let table_name = format!("u256_test_{}", table_suffix);
                // First drop any existing table
                let _ = sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
                    .execute(&pool)
                    .await;
                // Create test table
                if sqlx::query(&format!(
                    "CREATE TABLE IF NOT EXISTS {} (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        amount VARCHAR(66) NOT NULL,
                        description VARCHAR(255),
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    )",
                    table_name
                ))
                .execute(&pool)
                .await
                .is_ok()
                {
                    // Clean existing data
                    let _ = sqlx::query(&format!("DELETE FROM {}", table_name))
                        .execute(&pool)
                        .await;
                    Some(pool)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    #[tokio::test]
    async fn test_mysql_basic_u256_operations() {
        let Some(pool) = setup_mysql_test("basic").await else {
            println!("⚠️  Skipping MySQL test - no connection available");
            return;
        };

        let table_name = "u256_test_basic";

        // Test inserting U256 value
        let test_amount = SqlU256::from(U256::from(0xdeadbeef_u64));

        sqlx::query(&format!(
            "INSERT INTO {} (amount, description) VALUES (?, ?)",
            table_name
        ))
        .bind(&test_amount)
        .bind("Test Amount")
        .execute(&pool)
        .await
        .expect("Failed to insert U256 amount");

        // Test querying U256 value
        let row = sqlx::query(&format!(
            "SELECT amount, description FROM {} WHERE id = 1",
            table_name
        ))
        .fetch_one(&pool)
        .await
        .expect("Failed to select U256 amount");

        let retrieved_amount: SqlU256 = row.get("amount");
        let description: String = row.get("description");

        assert_eq!(retrieved_amount, test_amount);
        assert_eq!(description, "Test Amount");
        assert_eq!(retrieved_amount.to_string(), "0xdeadbeef");

        println!("✅ MySQL basic U256 operations test passed");
    }

    #[tokio::test]
    async fn test_mysql_extreme_u256_values() {
        let Some(pool) = setup_mysql_test("extreme").await else {
            println!("⚠️  Skipping MySQL extreme values test - no connection available");
            return;
        };

        let table_name = "u256_test_extreme";
        let extreme_values = [
            (SqlU256::from(U256::ZERO), "Zero"),
            (SqlU256::from(U256::from(1_u64)), "One"),
            (SqlU256::from(U256::MAX), "Maximum U256"),
        ];

        // Insert extreme values
        for (amount, desc) in &extreme_values {
            sqlx::query(&format!(
                "INSERT INTO {} (amount, description) VALUES (?, ?)",
                table_name
            ))
            .bind(amount)
            .bind(*desc)
            .execute(&pool)
            .await
            .expect("Failed to insert extreme U256 value");
        }

        // Verify they can be queried back
        let rows = sqlx::query(&format!(
            "SELECT amount, description FROM {} ORDER BY id",
            table_name
        ))
        .fetch_all(&pool)
        .await
        .expect("Failed to select extreme U256 values");

        assert_eq!(rows.len(), extreme_values.len());

        for (i, row) in rows.iter().enumerate() {
            let retrieved_amount: SqlU256 = row.get("amount");
            let description: String = row.get("description");

            assert_eq!(retrieved_amount, extreme_values[i].0);
            assert_eq!(description, extreme_values[i].1);

            // Verify string format
            let amount_str = retrieved_amount.to_string();
            assert!(
                amount_str.starts_with("0x"),
                "Should start with 0x: {}",
                amount_str
            );

            println!(
                "✅ {}: {} (length: {})",
                description,
                amount_str,
                amount_str.len()
            );
        }

        // Verify MAX value has exactly 66 characters
        let max_row = &rows[2];
        let max_amount: SqlU256 = max_row.get("amount");
        let max_str = max_amount.to_string();
        assert_eq!(
            max_str.len(),
            66,
            "U256::MAX should be exactly 66 characters"
        );

        println!("✅ MySQL extreme U256 values test passed");
    }
}

// PostgreSQL integration tests (requires running PostgreSQL server)
#[cfg(feature = "sqlx_str")]
mod postgres_tests {
    use ethereum_mysql::{SqlU256,U256};
    use sqlx::{PgPool, Row};

    // Helper function: setup PostgreSQL connection and test table
    async fn setup_postgres_test(table_suffix: &str) -> Option<PgPool> {
        // Try to connect to local PostgreSQL, skip test if it fails
        let database_url = std::env::var("POSTGRES_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:123456@localhost:5432/test_db".to_string());

        match PgPool::connect(&database_url).await {
            Ok(pool) => {
                let table_name = format!("u256_test_{}", table_suffix);
                // First drop any existing table
                let _ = sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
                    .execute(&pool)
                    .await;

                // Create test table
                if sqlx::query(&format!(
                    "CREATE TABLE {} (
                        id SERIAL PRIMARY KEY,
                        amount VARCHAR(66) NOT NULL,
                        description VARCHAR(255),
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                    )",
                    table_name
                ))
                .execute(&pool)
                .await
                .is_ok()
                {
                    Some(pool)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    #[tokio::test]
    async fn test_postgres_basic_u256_operations() {
        let Some(pool) = setup_postgres_test("basic").await else {
            println!("⚠️  Skipping PostgreSQL test - no connection available");
            return;
        };

        let table_name = "u256_test_basic";

        // Test inserting U256 value
        let test_amount = SqlU256::from(U256::from(123456789_u64));

        sqlx::query(&format!(
            "INSERT INTO {} (amount, description) VALUES ($1, $2)",
            table_name
        ))
        .bind(&test_amount)
        .bind("Test Amount")
        .execute(&pool)
        .await
        .expect("Failed to insert U256 amount");

        // Test querying U256 value
        let row = sqlx::query(&format!(
            "SELECT amount, description FROM {} WHERE id = 1",
            table_name
        ))
        .fetch_one(&pool)
        .await
        .expect("Failed to select U256 amount");

        let retrieved_amount: SqlU256 = row.get("amount");
        let description: String = row.get("description");

        assert_eq!(retrieved_amount, test_amount);
        assert_eq!(description, "Test Amount");

        println!("✅ PostgreSQL basic U256 operations test passed");
    }

    #[tokio::test]
    async fn test_postgres_u256_transaction_operations() {
        let Some(pool) = setup_postgres_test("transaction").await else {
            println!("⚠️  Skipping PostgreSQL transaction test - no connection available");
            return;
        };

        let table_name = "u256_test_transaction";

        // Test batch operations in transaction
        let mut tx = pool.begin().await.expect("Failed to begin transaction");

        let test_amounts = [
            (SqlU256::from(U256::from(1000_u64)), "Amount 1"),
            (SqlU256::from(U256::from(2000_u64)), "Amount 2"),
            (SqlU256::from(U256::from(3000_u64)), "Amount 3"),
        ];

        // Batch insert in transaction
        for (amount, desc) in &test_amounts {
            sqlx::query(&format!(
                "INSERT INTO {} (amount, description) VALUES ($1, $2)",
                table_name
            ))
            .bind(amount)
            .bind(*desc)
            .execute(&mut *tx)
            .await
            .expect("Failed to insert U256 amount in transaction");
        }

        // Commit transaction
        tx.commit().await.expect("Failed to commit transaction");

        // Verify data exists
        let count: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {}", table_name))
            .fetch_one(&pool)
            .await
            .expect("Failed to count U256 amounts");

        assert_eq!(count, 3);

        println!("✅ PostgreSQL U256 transaction operations test passed");
    }

    #[tokio::test]
    async fn test_postgres_u256_advanced_queries() {
        let Some(pool) = setup_postgres_test("advanced").await else {
            println!(
                "⚠️  Skipping PostgreSQL advanced U256 queries test - no connection available"
            );
            return;
        };

        // Create DeFi-like test data
        let defi_data = [
            (1, SqlU256::from(U256::from(1000000_u64)), "USDC Balance"),
            (1, SqlU256::from(U256::from(500000_u64)), "DAI Balance"),
            (2, SqlU256::from(U256::ZERO), "Empty Wallet"),
            (
                3,
                SqlU256::from(U256::from(0xdeadbeef_u64)),
                "Large Token Balance",
            ),
        ];

        let table_name = "user_token_balances_advanced";
        // First drop any existing table
        let _ = sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
            .execute(&pool)
            .await;
        // Create extended table structure
        sqlx::query(&format!(
            "CREATE TABLE {} (
                user_id INTEGER,
                balance VARCHAR(66) NOT NULL,
                token_name VARCHAR(255),
                last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )",
            table_name
        ))
        .execute(&pool)
        .await
        .expect("Failed to create user_token_balances table");

        // Insert test data
        for (user_id, balance, token_name) in &defi_data {
            sqlx::query(&format!(
                "INSERT INTO {} (user_id, balance, token_name) 
                 VALUES ($1, $2, $3)",
                table_name
            ))
            .bind(user_id)
            .bind(balance)
            .bind(*token_name)
            .execute(&pool)
            .await
            .expect("Failed to insert token balance data");
        }

        // Test complex query: find all users with non-zero balances
        let active_balances = sqlx::query(&format!(
            "SELECT user_id, balance, token_name 
             FROM {} 
             WHERE balance != $1
             ORDER BY user_id",
            table_name
        ))
        .bind(&SqlU256::from(U256::ZERO))
        .fetch_all(&pool)
        .await
        .expect("Failed to query active balances");

        assert_eq!(active_balances.len(), 3); // All except user 2's empty wallet

        // Test aggregate query: count tokens per user
        let balance_counts = sqlx::query(&format!(
            "SELECT user_id, COUNT(*) as token_count 
             FROM {} 
             GROUP BY user_id 
             ORDER BY user_id",
            table_name
        ))
        .fetch_all(&pool)
        .await
        .expect("Failed to query balance counts");

        assert_eq!(balance_counts.len(), 3);

        let user1_count: i64 = balance_counts[0].get("token_count");
        assert_eq!(user1_count, 2); // User 1 has 2 token balances

        println!("✅ PostgreSQL advanced U256 queries test passed");
    }
}

#[cfg(feature = "sqlx_str")]
mod format_consistency_tests {
    use ethereum_mysql::{SqlU256,U256};
    use std::str::FromStr;

    #[test]
    fn test_u256_format_consistency() {
        println!("=== U256 format consistency test ===");

        let test_values = [
            U256::ZERO,
            U256::from(1_u64),
            U256::from(255_u64),
            U256::from(0xdeadbeef_u64),
            U256::from(123456789_u64),
            U256::MAX,
        ];

        for value in test_values {
            let sql_u256 = SqlU256::from(value);
            let formatted = format!("{}", sql_u256);
            let parsed_back = SqlU256::from_str(&formatted).unwrap();

            assert_eq!(sql_u256, parsed_back);
            assert!(formatted.starts_with("0x"));
            assert!(formatted.len() >= 3);
            assert!(formatted.len() <= 66);

            println!("✅ {} -> {} -> {} (consistent)", value, formatted, parsed_back);
        }
    }

    #[test]
    fn test_u256_backward_compatibility() {
        println!("=== U256 backward compatibility test ===");

        // Test if old formats (decimal) that may exist in the database can be parsed
        let compatibility_tests = [
            ("0", U256::ZERO),
            ("1", U256::from(1_u64)),
            ("255", U256::from(255_u64)),
            ("123456789", U256::from(123456789_u64)),
            ("0x0", U256::ZERO),
            ("0x1", U256::from(1_u64)),
            ("0xff", U256::from(255_u64)),
            ("0x75bcd15", U256::from(123456789_u64)),
        ];

        for (input, expected_value) in compatibility_tests {
            match SqlU256::from_str(input) {
                Ok(parsed) => {
                    assert_eq!(*parsed.inner(), expected_value);
                    println!("✅ '{}' -> {} (expected: {})", input, parsed, expected_value);
                }
                Err(e) => {
                    println!("❌ '{}' failed to parse: {:?}", input, e);
                    panic!("Should be able to parse: {}", input);
                }
            }
        }
    }
}
