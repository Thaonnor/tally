// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

#[tokio::main]
async fn main() {
    let pool = initialize_database().await;

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![get_accounts, add_account])
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
    match database::get_all_accounts_with_balance(&pool).await {
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
    match database::insert_account_full(
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
