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
            current_balance DECIMAL(15,2),
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
            amount DECIMAL(15, 2) NOT NULL,
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
