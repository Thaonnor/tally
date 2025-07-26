// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

#[tokio::main]
async fn main() {
    let pool = database::create_connection()
        .await
        .expect("Failed to create database connection");

    database::create_accounts_table(&pool)
        .await
        .expect("Failed to create accounts table");

    database::create_transactions_table(&pool)
        .await
        .expect("Failed to create transactions table");

    database::create_categories_table(&pool)
        .await
        .expect("Failed to create categories table");

    database::create_transfers_table(&pool)
        .await
        .expect("Failed to create transfers table");

    // Test inserting an account
    let account_id = database::insert_account(&pool, "Chase Checking", "checking")
        .await
        .expect("Failed to insert account");
    println!("Created account with ID: {}", account_id);

    // Test reading accounts back
    let accounts = database::get_all_accounts(&pool)
        .await
        .expect("Failed to get accounts");

    for (id, name, account_type) in accounts {
        println!("Account {}: {} ({})", id, name, account_type);
    }

    // Test getting account by ID
    let account = database::get_account_by_id(&pool, 1)
        .await
        .expect("Failed to get account by ID");

    match account {
        Some((id, name, account_type)) => {
            println!("Found account: {} - {} ({})", id, name, account_type);
        }
        None => {
            println!("No account found with ID 1");
        }
    }

    // Test updating account balance
    let updated = database::update_account_balance(&pool, 1, 1250.75)
        .await
        .expect("Failed to update account balance");

    if updated {
        println!("Successfully updated account balance");

        // Check the updated account
        let account = database::get_account_by_id(&pool, 1)
            .await
            .expect("Failed to get updated account");
        if let Some((id, name, _)) = account {
            println!("Account {} ({}) balance updated", id, name);
        }
    } else {
        println!("No account was updated");
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
