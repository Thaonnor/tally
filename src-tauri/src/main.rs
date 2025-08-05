// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

/// Entry point for the Tally personal finance application.
/// 
/// Initializes the SQLite database connection, creates required tables,
/// and starts the Tauri desktop application with registered command handlers.
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful application startup, or an error if:
/// - Database initialization fails
/// - Tauri application fails to start
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - SQLite database cannot be created or connected to
/// - Required database tables cannot be created
/// - Tauri runtime fails to initialize
/// - Application configuration is invalid
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = initialize_database().await?;

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            get_account_by_id,
            add_transaction,
            get_transactions
        ])
        .run(tauri::generate_context!())?;
        
    Ok(())
}

/// Initializes the SQLite database connection and creates all required tables.
/// 
/// Creates a connection pool to the SQLite database file (`tally.db`) and ensures
/// all necessary tables exist for the application to function properly.
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(SqlitePool)` - A connection pool ready for database operations
/// - `Err(sqlx::Error)` - Database connection or table creation error
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - Database file cannot be created or accessed
/// - SQLite connection cannot be established
/// - Any required table creation fails (accounts, transactions, categories, transfers)
/// 
/// # Tables Created
/// 
/// - `accounts` - User financial accounts
/// - `transactions` - Financial transactions
/// - `categories` - Transaction categories
/// - `transfers` - Money transfers between accounts
async fn initialize_database() -> Result<sqlx::SqlitePool, sqlx::Error> {
    let pool = database::create_connection().await?;

    database::create_accounts_table(&pool).await?;
    database::create_transactions_table(&pool).await?;
    database::create_categories_table(&pool).await?;
    database::create_transfers_table(&pool).await?;

    Ok(pool)
}

#[tauri::command]
async fn get_accounts(
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Vec<(i64, String, String, Option<f64>)>, String> {
    match database::get_all_accounts(&pool).await {
        Ok(accounts) => Ok(accounts),
        Err(e) => Err(format!("Failed to get accounts: {}", e)),
    }
}

#[tauri::command]
async fn add_account(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    name: String,
    account_type: String,
    institution: Option<String>,
    current_balance: Option<f64>,
) -> Result<i64, String> {
    match database::insert_account(
        &pool,
        &name,
        &account_type,
        institution.as_deref(),
        current_balance,
    )
    .await
    {
        Ok(id) => Ok(id),
        Err(e) => Err(format!("Failed to add account: {}", e)),
    }
}

#[tauri::command]
async fn get_account_by_id(
    id: i64,
    state: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Option<(i64, String, String, Option<f64>)>, String> {
    database::get_account_by_id(&state, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_transaction(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    account_id: i64,
    date: String,
    amount: f64,
    description: Option<String>,
    payee: Option<String>,
    memo: Option<String>,
    category_id: Option<i64>,
    pending: bool,
    cleared: bool,
) -> Result<i64, String> {
    match database::insert_transaction(
        &pool,
        account_id,
        &date,
        amount,
        description.as_deref(),
        payee.as_deref(),
        memo.as_deref(),
        category_id,
        pending,
        cleared,
    )
    .await
    {
        Ok(id) => Ok(id),
        Err(e) => Err(format!("Failed to add transaction: {}", e)),
    }
}

#[tauri::command]
async fn get_transactions(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    account_id: i64,
    limit: i32,
    offset: i32,
) -> Result<Vec<database::Transaction>, String> {
    match database::get_account_transactions(&pool, account_id, limit, offset).await {
        Ok(transactions) => Ok(transactions),
        Err(e) => Err(format!("Failed to get transactions: {}", e)),
    }
}
