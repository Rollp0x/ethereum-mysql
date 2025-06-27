use ethereum_mysql::{sqladdress, SqlAddress};
use std::str::FromStr;

// Basic SQLite tests (in-memory database, no external services required)
#[cfg(feature = "sqlx")]
mod sqlite_tests {
    use super::*;
    use sqlx::{Row, SqlitePool};

    #[tokio::test]
    async fn test_sqlite_basic_operations() {
        // Create in-memory database
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS test_addresses")
            .execute(&pool)
            .await
            .unwrap();
        // Create test table
        sqlx::query(
            "CREATE TABLE test_addresses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address BINARY(20) NOT NULL,
                name TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create test table");

        // Test inserting address
        let test_addr = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();

        sqlx::query("INSERT INTO test_addresses (address, name) VALUES (?, ?)")
            .bind(&test_addr)
            .bind("Test Address")
            .execute(&pool)
            .await
            .expect("Failed to insert address");

        // Test querying address
        let row = sqlx::query("SELECT address, name FROM test_addresses WHERE id = 1")
            .fetch_one(&pool)
            .await
            .expect("Failed to select address");

        let retrieved_addr: SqlAddress = row.get("address");
        let name: String = row.get("name");

        assert_eq!(retrieved_addr, test_addr);
        assert_eq!(name, "Test Address");
    }

    #[tokio::test]
    async fn test_sqlite_zero_address() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS test_addresses")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE test_addresses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address BINARY(20) NOT NULL,
                name TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create test table");

        // Use ZERO constant
        sqlx::query("INSERT INTO test_addresses (address, name) VALUES (?, ?)")
            .bind(&SqlAddress::ZERO)
            .bind("Zero Address")
            .execute(&pool)
            .await
            .expect("Failed to insert zero address");

        let row = sqlx::query("SELECT address FROM test_addresses WHERE name = ?")
            .bind("Zero Address")
            .fetch_one(&pool)
            .await
            .expect("Failed to select zero address");

        let retrieved_addr: SqlAddress = row.get("address");
        assert_eq!(retrieved_addr, SqlAddress::ZERO);
        assert_eq!(
            retrieved_addr.to_string(),
            "0x0000000000000000000000000000000000000000"
        );
    }

    #[tokio::test]
    async fn test_sqlite_macro_addresses() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS test_addresses")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE test_addresses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address BINARY(20) NOT NULL,
                description TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create test table");

        // Test addresses created using macro
        let addresses = [
            (
                sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
                "Regular Address",
            ),
            (
                sqladdress!("0x0000000000000000000000000000000000000000"),
                "Zero Address",
            ),
            (
                sqladdress!("dEaD000000000000000000000000000000000000"),
                "Dead Address",
            ),
        ];

        // Insert all addresses
        for (addr, desc) in &addresses {
            sqlx::query("INSERT INTO test_addresses (address, description) VALUES (?, ?)")
                .bind(addr)
                .bind(*desc)
                .execute(&pool)
                .await
                .expect("Failed to insert address");
        }

        // Verify they can be queried back
        let rows = sqlx::query("SELECT address, description FROM test_addresses ORDER BY id")
            .fetch_all(&pool)
            .await
            .expect("Failed to select addresses");

        assert_eq!(rows.len(), addresses.len());

        for (i, row) in rows.iter().enumerate() {
            let retrieved_addr: SqlAddress = row.get("address");
            let description: String = row.get("description");

            assert_eq!(retrieved_addr, addresses[i].0);
            assert_eq!(description, addresses[i].1);
        }
    }

    #[tokio::test]
    async fn test_sqlite_multiple_addresses() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to SQLite");
        let _ = sqlx::query("DROP TABLE IF EXISTS test_addresses")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE test_addresses (
                user_id INTEGER,
                wallet_address BINARY(20) NOT NULL,
                is_primary BOOLEAN DEFAULT FALSE
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create wallet table");

        // Simulate user wallet scenario
        let test_data = [
            (1, "0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d", true),
            (1, "0x1234567890123456789012345678901234567890", false),
            (2, "0x0000000000000000000000000000000000000000", true), // User has no wallet
            (3, "0xdEaD000000000000000000000000000000000000", true),
        ];

        for (user_id, addr_str, is_primary) in &test_data {
            let addr = SqlAddress::from_str(addr_str).unwrap();
            sqlx::query(
                "INSERT INTO test_addresses (user_id, wallet_address, is_primary) VALUES (?, ?, ?)",
            )
            .bind(user_id)
            .bind(&addr)
            .bind(is_primary)
            .execute(&pool)
            .await
            .expect("Failed to insert wallet address");
        }

        // Query primary wallet addresses
        let primary_wallets = sqlx::query(
            "SELECT user_id, wallet_address FROM test_addresses WHERE is_primary = TRUE ORDER BY user_id"
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to select primary wallets");

        assert_eq!(primary_wallets.len(), 3);

        // Verify data
        for (i, row) in primary_wallets.iter().enumerate() {
            let user_id: i32 = row.get("user_id");
            let addr: SqlAddress = row.get("wallet_address");

            assert_eq!(user_id, (i + 1) as i32);

            match i {
                0 => assert_eq!(
                    addr.to_string(),
                    "0x742D35CC6635C0532925A3b8D42cC72b5c2A9a1D"
                ),
                1 => assert_eq!(addr, SqlAddress::ZERO),
                2 => assert_eq!(
                    addr.to_string(),
                    "0xdEad000000000000000000000000000000000000"
                ), // Note lowercase 'a'
                _ => panic!("Unexpected row"),
            }
        }

        // Check if users have valid wallets (non-zero addresses)
        let valid_wallets = sqlx::query(
            "SELECT user_id FROM test_addresses
             WHERE is_primary = TRUE AND wallet_address != ?
             ORDER BY user_id",
        )
        .bind(&SqlAddress::ZERO)
        .fetch_all(&pool)
        .await
        .expect("Failed to select valid wallets");

        assert_eq!(valid_wallets.len(), 2); // Users 1 and 3 have valid wallets

        let user1: i32 = valid_wallets[0].get("user_id");
        let user3: i32 = valid_wallets[1].get("user_id");
        assert_eq!(user1, 1);
        assert_eq!(user3, 3);
    }
}

// MySQL integration tests (requires running MySQL server)
#[cfg(feature = "sqlx")]
mod mysql_tests {
    use super::*;
    use sqlx::{MySqlPool, Row};

    // Helper function: setup MySQL connection and test table
    async fn setup_mysql_test() -> Option<MySqlPool> {
        // Try to connect to local MySQL, skip test if it fails
        let database_url = std::env::var("MYSQL_DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:123456@localhost:3306/test_db".to_string());

        match MySqlPool::connect(&database_url).await {
            Ok(pool) => {
                let _ = sqlx::query("DROP TABLE IF EXISTS ethereum_addresses")
                    .execute(&pool)
                    .await
                    .unwrap();
                // Create test table
                if sqlx::query(
                    "CREATE TABLE IF NOT EXISTS ethereum_addresses (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        address BINARY(20) NOT NULL,
                        label VARCHAR(255),
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    )",
                )
                .execute(&pool)
                .await
                .is_ok()
                {
                    // Clean existing data
                    let _ = sqlx::query("DELETE FROM ethereum_addresses")
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
    async fn test_mysql_basic_operations() {
        let Some(pool) = setup_mysql_test().await else {
            println!("⚠️  Skipping MySQL test - no connection available");
            return;
        };

        // Test inserting address
        let test_addr = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();

        sqlx::query("INSERT INTO ethereum_addresses (address, label) VALUES (?, ?)")
            .bind(&test_addr)
            .bind("Test Address")
            .execute(&pool)
            .await
            .expect("Failed to insert address");

        // Test querying address
        let row = sqlx::query("SELECT address, label FROM ethereum_addresses WHERE id = 1")
            .fetch_one(&pool)
            .await
            .expect("Failed to select address");

        let retrieved_addr: SqlAddress = row.get("address");
        let label: String = row.get("label");

        assert_eq!(retrieved_addr, test_addr);
        assert_eq!(label, "Test Address");

        println!("✅ MySQL basic operations test passed");
    }

    #[tokio::test]
    async fn test_mysql_zero_and_special_addresses() {
        let Some(pool) = setup_mysql_test().await else {
            println!("⚠️  Skipping MySQL zero address test - no connection available");
            return;
        };

        let special_addresses = [
            (SqlAddress::ZERO, "Zero Address"),
            (
                sqladdress!("0xffffffffffffffffffffffffffffffffffffffff"),
                "Max Address",
            ),
            (
                sqladdress!("0xdead000000000000000000000000000000000000"),
                "Dead Address",
            ),
        ];

        // Insert special addresses
        for (addr, label) in &special_addresses {
            sqlx::query("INSERT INTO ethereum_addresses (address, label) VALUES (?, ?)")
                .bind(addr)
                .bind(*label)
                .execute(&pool)
                .await
                .expect("Failed to insert special address");
        }

        // Verify they can be queried back
        let rows = sqlx::query("SELECT address, label FROM ethereum_addresses ORDER BY id")
            .fetch_all(&pool)
            .await
            .expect("Failed to select special addresses");

        assert_eq!(rows.len(), special_addresses.len());

        for (i, row) in rows.iter().enumerate() {
            let retrieved_addr: SqlAddress = row.get("address");
            let label: String = row.get("label");

            assert_eq!(retrieved_addr, special_addresses[i].0);
            assert_eq!(label, special_addresses[i].1);
        }

        println!("✅ MySQL special addresses test passed");
    }
}

// PostgreSQL integration tests (requires running PostgreSQL server)
#[cfg(feature = "sqlx")]
mod postgres_tests {
    use super::*;
    use sqlx::{PgPool, Row};

    // Helper function: setup PostgreSQL connection and test table
    async fn setup_postgres_test(table_suffix: &str) -> Option<PgPool> {
        // Try to connect to local PostgreSQL, skip test if it fails
        let database_url = std::env::var("POSTGRES_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:123456@localhost:5432/test_db".to_string());

        match PgPool::connect(&database_url).await {
            Ok(pool) => {
                let table_name = format!("ethereum_addresses_{}", table_suffix);
                let _ = sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
                    .execute(&pool)
                    .await;
                // First drop any existing table
                let _ = sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
                    .execute(&pool)
                    .await;

                // Create test table
                if sqlx::query(&format!(
                    "CREATE TABLE {} (
                        id SERIAL PRIMARY KEY,
                        address BYTEA NOT NULL,
                        label VARCHAR(255),
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
    async fn test_postgres_basic_operations() {
        let Some(pool) = setup_postgres_test("basic").await else {
            println!("⚠️  Skipping PostgreSQL test - no connection available");
            return;
        };

        let table_name = "ethereum_addresses_basic";

        // Test inserting address
        let test_addr = SqlAddress::from_str("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d").unwrap();

        sqlx::query(&format!(
            "INSERT INTO {} (address, label) VALUES ($1, $2)",
            table_name
        ))
        .bind(&test_addr)
        .bind("Test Address")
        .execute(&pool)
        .await
        .expect("Failed to insert address");

        // Test querying address
        let row = sqlx::query(&format!(
            "SELECT address, label FROM {} WHERE id = 1",
            table_name
        ))
        .fetch_one(&pool)
        .await
        .expect("Failed to select address");

        let retrieved_addr: SqlAddress = row.get("address");
        let label: String = row.get("label");

        assert_eq!(retrieved_addr, test_addr);
        assert_eq!(label, "Test Address");

        println!("✅ PostgreSQL basic operations test passed");
    }

    #[tokio::test]
    async fn test_postgres_zero_and_special_addresses() {
        let Some(pool) = setup_postgres_test("special").await else {
            println!("⚠️  Skipping PostgreSQL zero address test - no connection available");
            return;
        };

        let table_name = "ethereum_addresses_special";
        let special_addresses = [
            (SqlAddress::ZERO, "Zero Address"),
            (
                sqladdress!("0xffffffffffffffffffffffffffffffffffffffff"),
                "Max Address",
            ),
            (
                sqladdress!("0xdead000000000000000000000000000000000000"),
                "Dead Address",
            ),
        ];

        // Insert special addresses
        for (addr, label) in &special_addresses {
            sqlx::query(&format!(
                "INSERT INTO {} (address, label) VALUES ($1, $2)",
                table_name
            ))
            .bind(addr)
            .bind(*label)
            .execute(&pool)
            .await
            .expect("Failed to insert special address");
        }

        // Verify they can be queried back
        let rows = sqlx::query(&format!(
            "SELECT address, label FROM {} ORDER BY id",
            table_name
        ))
        .fetch_all(&pool)
        .await
        .expect("Failed to select special addresses");

        assert_eq!(rows.len(), special_addresses.len());

        for (i, row) in rows.iter().enumerate() {
            let retrieved_addr: SqlAddress = row.get("address");
            let label: String = row.get("label");

            assert_eq!(retrieved_addr, special_addresses[i].0);
            assert_eq!(label, special_addresses[i].1);
        }

        println!("✅ PostgreSQL special addresses test passed");
    }

    #[tokio::test]
    async fn test_postgres_transaction_operations() {
        let Some(pool) = setup_postgres_test("transaction").await else {
            println!("⚠️  Skipping PostgreSQL transaction test - no connection available");
            return;
        };

        let table_name = "ethereum_addresses_transaction";

        // Test batch operations in transaction
        let mut tx = pool.begin().await.expect("Failed to begin transaction");

        let test_addresses = [
            (
                sqladdress!("0x1111111111111111111111111111111111111111"),
                "Address 1",
            ),
            (
                sqladdress!("0x2222222222222222222222222222222222222222"),
                "Address 2",
            ),
            (
                sqladdress!("0x3333333333333333333333333333333333333333"),
                "Address 3",
            ),
        ];

        // Batch insert in transaction
        for (addr, label) in &test_addresses {
            sqlx::query(&format!(
                "INSERT INTO {} (address, label) VALUES ($1, $2)",
                table_name
            ))
            .bind(addr)
            .bind(*label)
            .execute(&mut *tx)
            .await
            .expect("Failed to insert address in transaction");
        }

        // Commit transaction
        tx.commit().await.expect("Failed to commit transaction");

        // Verify data exists
        let count: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {}", table_name))
            .fetch_one(&pool)
            .await
            .expect("Failed to count addresses");

        assert_eq!(count, 3);

        // Test querying by address range
        let range_results = sqlx::query(&format!(
            "SELECT address FROM {} 
             WHERE address >= $1 AND address <= $2 
             ORDER BY address",
            table_name
        ))
        .bind(&sqladdress!("0x1000000000000000000000000000000000000000"))
        .bind(&sqladdress!("0x2999999999999999999999999999999999999999"))
        .fetch_all(&pool)
        .await
        .expect("Failed to query address range");

        assert_eq!(range_results.len(), 2); // Should find Address 1 and Address 2

        println!("✅ PostgreSQL transaction operations test passed");
    }

    #[tokio::test]
    async fn test_postgres_advanced_queries() {
        let Some(pool) = setup_postgres_test("advanced").await else {
            println!("⚠️  Skipping PostgreSQL advanced queries test - no connection available");
            return;
        };

        // Create more complex test data
        let wallet_data = [
            (
                1,
                sqladdress!("0x742d35Cc6635C0532925a3b8D42cC72b5c2A9A1d"),
                "Primary Wallet",
                true,
            ),
            (
                1,
                sqladdress!("0x1234567890123456789012345678901234567890"),
                "Secondary Wallet",
                false,
            ),
            (2, SqlAddress::ZERO, "Empty Wallet", true),
            (
                3,
                sqladdress!("0xdead000000000000000000000000000000000000"),
                "Burn Address",
                true,
            ),
        ];

        let table_name = "user_wallets_advanced";

        // First drop any existing table, then create new table
        let _ = sqlx::query(&format!("DROP TABLE IF EXISTS {}", table_name))
            .execute(&pool)
            .await;

        // Create extended table structure
        sqlx::query(&format!(
            "CREATE TABLE {} (
                user_id INTEGER,
                wallet_address BYTEA NOT NULL,
                wallet_name VARCHAR(255),
                is_primary BOOLEAN DEFAULT FALSE,
                balance_wei NUMERIC(78, 0) DEFAULT 0
            )",
            table_name
        ))
        .execute(&pool)
        .await
        .expect("Failed to create user_wallets table");

        // Insert test data
        for (user_id, addr, name, is_primary) in &wallet_data {
            sqlx::query(&format!(
                "INSERT INTO {} (user_id, wallet_address, wallet_name, is_primary) 
                 VALUES ($1, $2, $3, $4)",
                table_name
            ))
            .bind(user_id)
            .bind(addr)
            .bind(*name)
            .bind(is_primary)
            .execute(&pool)
            .await
            .expect("Failed to insert wallet data");
        }

        // Test complex query: find all users with primary wallets that have non-zero addresses
        let active_users = sqlx::query(&format!(
            "SELECT user_id, wallet_address, wallet_name 
             FROM {} 
             WHERE is_primary = TRUE AND wallet_address != $1
             ORDER BY user_id",
            table_name
        ))
        .bind(&SqlAddress::ZERO)
        .fetch_all(&pool)
        .await
        .expect("Failed to query active users");

        assert_eq!(active_users.len(), 2); // Users 1 and 3

        // Verify results
        let user1_addr: SqlAddress = active_users[0].get("wallet_address");
        let user3_addr: SqlAddress = active_users[1].get("wallet_address");

        assert_eq!(
            user1_addr.to_string(),
            "0x742D35CC6635C0532925A3b8D42cC72b5c2A9a1D"
        );
        assert_eq!(
            user3_addr.to_string(),
            "0xdEad000000000000000000000000000000000000"
        );

        // Test aggregate query: count wallets per user
        let wallet_counts = sqlx::query(&format!(
            "SELECT user_id, COUNT(*) as wallet_count 
             FROM {} 
             GROUP BY user_id 
             ORDER BY user_id",
            table_name
        ))
        .fetch_all(&pool)
        .await
        .expect("Failed to query wallet counts");

        assert_eq!(wallet_counts.len(), 3);

        let user1_count: i64 = wallet_counts[0].get("wallet_count");
        let user2_count: i64 = wallet_counts[1].get("wallet_count");
        let user3_count: i64 = wallet_counts[2].get("wallet_count");

        assert_eq!(user1_count, 2); // User 1 has 2 wallets
        assert_eq!(user2_count, 1); // User 2 has 1 wallet
        assert_eq!(user3_count, 1); // User 3 has 1 wallet

        println!("✅ PostgreSQL advanced queries test passed");
    }
}
