use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use std::fs::File;

pub async fn create_connection() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_path = "./tally.db";

    // Create the file if it doesn't exist
    if !std::path::Path::new(db_path).exists() {
        File::create(db_path).expect("Failed to create database file");
    }

    let database_url = "sqlite:./tally.db";
    let pool = SqlitePool::connect(database_url).await?;

    Ok(pool)
}

pub async fn create_accounts_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            current_balance INTEGER,
            institution TEXT,
            display_order INTEGER,
            archived BOOLEAN DEFAULT FALSE,
            include_in_net_worth BOOLEAN DEFAULT TRUE,
            account_number_last4 TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_transactions_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL REFERENCES accounts(id),
            date DATE NOT NULL,
            amount INTEGER NOT NULL,
            description TEXT,
            category_id INTEGER REFERENCES categories(id),
            pending BOOLEAN DEFAULT FALSE,
            transaction_type TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            cleared BOOLEAN DEFAULT FALSE,
            reconciled BOOLEAN DEFAULT FALSE,
            import_id TEXT,
            source TEXT,
            payee TEXT,
            original_description TEXT,
            memo TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_categories_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            archived BOOLEAN DEFAULT FALSE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            display_order INTEGER,
            parent_category_id INTEGER REFERENCES categories(id),
            default_discretionary BOOLEAN,
            default_fixed BOOLEAN,
            last_used_date DATETIME
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_transfers_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transfers (
            id INTEGER PRIMARY KEY,
            from_transaction_id INTEGER NOT NULL REFERENCES transactions(id),
            to_transaction_id INTEGER NOT NULL REFERENCES transactions(id),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            transfer_type TEXT,
            auto_created BOOLEAN DEFAULT FALSE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_account_by_id(
    pool: &Pool<Sqlite>,
    id: i64,
) -> Result<Option<(i64, String, String, Option<f64>)>, sqlx::Error> {
    let account = sqlx::query_as::<_, (i64, String, String, Option<i64>)>(
        "SELECT id, name, type, current_balance FROM accounts WHERE id = ? AND archived = FALSE",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let result = account.map(|(id, name, type_, balance_cents)| {
        (id, name, type_, cents_to_dollars_option(balance_cents))
    });

    Ok(result)
}

pub async fn get_all_accounts(
    pool: &Pool<Sqlite>,
) -> Result<Vec<(i64, String, String, Option<f64>)>, sqlx::Error> {
    let accounts = sqlx::query_as::<_, (i64, String, String, Option<i64>)>(
       "SELECT id, name, type, current_balance FROM accounts WHERE archived = FALSE ORDER BY display_order, name",
   ).fetch_all(pool).await?;

    let result = accounts
        .into_iter()
        .map(|(id, name, type_, balance_cents)| {
            (id, name, type_, cents_to_dollars_option(balance_cents))
        })
        .collect();

    Ok(result)
}

pub async fn insert_account(
    pool: &Pool<Sqlite>,
    name: &str,
    account_type: &str,
    institution: Option<&str>,
    current_balance: Option<f64>,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO accounts (name, type, institution, current_balance) VALUES (?, ?, ?, ?)",
    )
    .bind(name)
    .bind(account_type)
    .bind(institution)
    .bind(dollars_to_cents_option(current_balance))
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

// Get balance for a specific account (returns dollars)
pub async fn get_account_balance(
    pool: &Pool<Sqlite>,
    account_id: i64,
) -> Result<Option<f64>, sqlx::Error> {
    let balance_cents = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT current_balance FROM accounts WHERE id = ? AND archived = FALSE",
    )
    .bind(account_id)
    .fetch_optional(pool)
    .await?;

    Ok(balance_cents.flatten().map(|cents| cents as f64 / 100.0))
}

// Set balance for a specific account (accepts dollars)
pub async fn set_account_balance(
    pool: &Pool<Sqlite>,
    account_id: i64,
    balance_dollars: f64,
) -> Result<(), sqlx::Error> {
    let balance_cents = (balance_dollars * 100.0).round() as i64;

    sqlx::query(
        "UPDATE accounts SET current_balance = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(balance_cents)
    .bind(account_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub fn dollars_to_cents(dollars: f64) -> i64 {
    (dollars * 100.0).round() as i64
}

pub fn cents_to_dollars(cents: i64) -> f64 {
    cents as f64 / 100.0
}

pub fn dollars_to_cents_option(dollars: Option<f64>) -> Option<i64> {
    dollars.map(dollars_to_cents)
}

pub fn cents_to_dollars_option(cents: Option<i64>) -> Option<f64> {
    cents.map(cents_to_dollars)
}
