use alloy::primitives::utils::parse_ether;
use ethereum_mysql::{sqladdress, SqlAddress, SqlBytes, SqlHash, SqlTopicHash, SqlU256};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::str::FromStr;

const INPUT:&str = "0xa9059cbb00000000000000000000000042066a09b94e41e0263eee935177c2ecfff7fe11000000000000000000000000000000000000000000000002629f66e0c5300000";
const TRANSFER_TOPIC: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, FromRow)]
struct TxInfo {
    pub id: Option<i32>,
    tx_hash: SqlHash,
    from_address: SqlAddress,
    to_address: SqlAddress,
    value: SqlU256,
    data: SqlBytes,
    gas: i64, // PostgreSQL INTEGER is i32, BIGINT is i64
    gas_price: SqlU256,
    topic: SqlTopicHash,
}

fn get_tx_info() -> TxInfo {
    TxInfo {
        id: None,
        tx_hash: SqlHash::from_str(
            "0x370bccf7c5a2e99914102d7d717f3b3f73ce82ca330e70c56aa89eaaff3c5ef9",
        )
        .unwrap(),
        from_address: sqladdress!("0x509a82d892C8b226FD4BF69cBE602aF942b94B24"),
        to_address: sqladdress!("0xD0eC028a3D21533Fdd200838F39c85B03679285D"),
        value: SqlU256::from(parse_ether("1.0").unwrap()),
        data: SqlBytes::from_str(INPUT).unwrap(),
        gas: 40258,
        gas_price: SqlU256::from(parse_ether("0.000000000304356729").unwrap()),
        topic: SqlTopicHash::from_str(TRANSFER_TOPIC).unwrap(),
    }
}

#[tokio::main]
async fn main() {
    test_postgres_operations().await;
}

async fn test_postgres_operations() {
    let database_url = std::env::var("POSTGRES_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:123456@localhost:5432/test_db".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    let _ = sqlx::query("DROP TABLE IF EXISTS tx_infos")
        .execute(&pool)
        .await
        .unwrap();
    // Create tx_infos table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tx_infos (
                id SERIAL PRIMARY KEY,
                tx_hash VARCHAR(66) NOT NULL,
                from_address VARCHAR(42) NOT NULL,
                to_address VARCHAR(42) NOT NULL,
                value VARCHAR(66) NOT NULL,
                data TEXT NOT NULL,
                gas BIGINT NOT NULL,
                gas_price VARCHAR(66) NOT NULL,
                topic VARCHAR(66) NOT NULL
            )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create tx_infos table");

    let tx_info = get_tx_info();

    sqlx::query("INSERT INTO tx_infos (tx_hash, from_address, to_address, value, data, gas, gas_price, topic) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&tx_info.tx_hash)
            .bind(&tx_info.from_address)
            .bind(&tx_info.to_address)
            .bind(&tx_info.value)
            .bind(&tx_info.data)
            .bind(tx_info.gas)
            .bind(&tx_info.gas_price)
            .bind(&tx_info.topic)
            .execute(&pool)
            .await
            .expect("Failed to insert tx_info");

    let tx: TxInfo = sqlx::query_as("SELECT * FROM tx_infos WHERE from_address = $1")
        .bind(&tx_info.from_address)
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch tx_info");

    println!("Transaction Info:");
    println!("Tx Hash: {}", tx.tx_hash);
    println!("From: {}", tx.from_address);
    println!("To: {}", tx.to_address);
    println!("Value: {}", tx.value);
    println!("Data: {}", tx.data);
    println!("Gas: {}", tx.gas);
    println!("Gas Price: {}", tx.gas_price);
    println!("Topic: {}", tx.topic);
}
