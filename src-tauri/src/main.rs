// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

#[tokio::main]
async fn main() {
    let pool = initialize_database().await;

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            get_account_by_id,
            add_transaction,
            get_transactions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_database() -> sqlx::SqlitePool {
    let pool = database::create_connection()
        .await
        .expect("Failed to create database connection.");

    database::create_accounts_table(&pool)
        .await
        .expect("Failed to create accounts table.");
    database::create_transactions_table(&pool)
        .await
        .expect("Failed to create transactions table.");
    database::create_categories_table(&pool)
        .await
        .expect("Failed to create categories table.");
    database::create_transfers_table(&pool)
        .await
        .expect("Failed to create transfers table.");

    pool
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
) -> Result<
    Vec<(
        i64,
        String,
        f64,
        Option<i64>,
        Option<String>,
        Option<String>,
        bool,
        bool,
        String,
    )>,
    String,
> {
    match database::get_account_transactions(&pool, account_id, limit, offset).await {
        Ok(transactions) => {
            // Convert amount from cents to dollars
            let result = transactions
                .into_iter()
                .map(
                    |(
                        id,
                        date,
                        amount_cents,
                        category_id,
                        description,
                        payee,
                        pending,
                        cleared,
                        transaction_type,
                    )| {
                        (
                            id,
                            date,
                            database::cents_to_dollars(amount_cents),
                            category_id,
                            description,
                            payee,
                            pending,
                            cleared,
                            transaction_type,
                        )
                    },
                )
                .collect();
            Ok(result)
        },
        Err(e) => Err(format!("Failed to get transactions: {}", e)),
    }
}
