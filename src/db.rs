
use std::env;
use crate::transaction::{Transaction, User, Incentive};
use sqlx::{Pool, Sqlite, SqlitePool, Acquire, Error};
use reqwest::Client;



pub async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&db_url).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            balance REAL NOT NULL
        );"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender_id TEXT NOT NULL,
            recipient_id TEXT NOT NULL,
            amount REAL NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );"
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}


pub async fn insert_transaction(pool: &Pool<Sqlite>, tx: &Transaction) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO transactions (account_id, amount) VALUES (?, ?)"
    )
    .bind(&tx.sender_id)
    .bind(tx.amount)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn process_transaction(pool: &Pool<Sqlite>, tx: &Transaction) -> Result<(), Error> {
    let mut conn = pool.acquire().await?;

    // Fetch sender and recipient
    let sender = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&tx.sender_id)
        .fetch_optional(&mut *conn)
        .await?;

    let recipient = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&tx.recipient_id)
        .fetch_optional(&mut *conn)
        .await?;

    let (Some(sender), Some(recipient)) = (sender, recipient) else {
        println!("❌ Invalid transaction: missing sender or recipient.");
        return Ok(());
    };

    if sender.balance < tx.amount {
        println!("⚠️ Sender '{}' has insufficient balance.", sender.id);
        return Ok(());
    }

    // 🌐 Call incentive API
    let client = Client::new();
    let incentive_response = client
        .post("http://localhost:8080/incentive")
        .json(tx)
        .send()
        .await
        .expect("Failed to call incentive API");

    let incentive: Incentive = incentive_response
        .json()
        .await
        .expect("Failed to parse incentive response");

    println!(
        "💰 Incentive for this transaction: {:.2} added to recipient '{}'",
        incentive.amount, recipient.id
    );

    // 🔄 Begin atomic DB transaction
    let mut tx_db = conn.begin().await?;

    // Insert transaction with incentive
    sqlx::query("INSERT INTO transactions (sender_id, recipient_id, amount, incentive) VALUES (?, ?, ?, ?)")
        .bind(&tx.sender_id)
        .bind(&tx.recipient_id)
        .bind(tx.amount)
        .bind(incentive.amount)
        .execute(&mut *tx_db)
        .await?;

    // Update sender balance
    sqlx::query("UPDATE users SET balance = ? WHERE id = ?")
        .bind(sender.balance - tx.amount)
        .bind(&sender.id)
        .execute(&mut *tx_db)
        .await?;

    // Update recipient balance (add incentive)
    sqlx::query("UPDATE users SET balance = ? WHERE id = ?")
        .bind(recipient.balance + tx.amount + incentive.amount)
        .bind(&recipient.id)
        .execute(&mut *tx_db)
        .await?;

    tx_db.commit().await?;
    println!(
        "✅ Transaction complete: {} → {} of ${} + incentive ${}",
        sender.id, recipient.id, tx.amount, incentive.amount
    );

    Ok(())
}
