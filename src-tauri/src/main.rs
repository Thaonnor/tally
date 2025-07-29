// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

#[tokio::main]
async fn main() {
    let pool = initialize_database().await;

    tauri::Builder::default()
        .manage(pool)
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
