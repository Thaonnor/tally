// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

#[cfg(test)]
mod tests;

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
            get_account,
            update_account,
            archive_account,
            get_categories,
            add_category,
            get_category,
            update_category,
            archive_category,
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
    
    // Seed default system categories
    database::seed_default_categories(&pool).await?;

    Ok(pool)
}

/// Retrieves all non-archived accounts from the database.
/// 
/// This Tauri command fetches all active financial accounts, including their
/// complete metadata such as balances, institution information, and display settings.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Vec<Account>)` - Vector of Account structs with all fields populated
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Database Behavior
/// 
/// - Only returns accounts where `archived = FALSE`
/// - Results are ordered by `display_order` first, then alphabetically by `name`
/// - Currency amounts are automatically converted from database cents to dollars
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const accounts = await invoke('get_accounts');
/// ```
#[tauri::command]
async fn get_accounts(
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Vec<database::Account>, String> {
    database::get_accounts(&pool)
        .await
        .map_err(|e| format!("Failed to get accounts: {}", e))
}

/// Creates a new financial account in the database.
/// 
/// This Tauri command accepts a complete account creation request with all user-settable
/// fields and inserts it into the accounts table. Database-managed fields like timestamps
/// and ID are handled automatically.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// * `request` - Account creation data including name, type, and optional fields
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(i64)` - The ID of the newly created account
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Request Fields
/// 
/// - `name` - Account display name (required)
/// - `account_type` - Account type like "checking", "savings" (required)
/// - `institution` - Bank or financial institution name (optional)
/// - `current_balance` - Starting balance in dollars (optional)
/// - `display_order` - Sort order for account listing (optional)
/// - `include_in_net_worth` - Whether to include in net worth calculations (optional, defaults to true)
/// - `account_number_last4` - Last 4 digits of account number (optional)
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const request = {
///   name: "My Checking",
///   account_type: "checking",
///   institution: "Bank of America",
///   current_balance: 1000.50,
///   display_order: 1,
///   include_in_net_worth: true,
///   account_number_last4: "1234"
/// };
/// const accountId = await invoke('add_account', { request });
/// ```
#[tauri::command]
async fn add_account(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    request: database::CreateAccountRequest,
) -> Result<i64, String> {
    match database::insert_account(&pool, &request).await {
        Ok(id) => Ok(id),
        Err(e) => Err(format!("Failed to add account: {}", e)),
    }
}

/// Retrieves a specific account by its unique ID.
/// 
/// This Tauri command fetches complete account information for a single account,
/// returning the same full Account struct as get_accounts() for consistency.
/// Only returns data for non-archived accounts.
/// 
/// # Arguments
/// 
/// * `id` - The unique ID of the account to retrieve
/// * `state` - Tauri-managed SQLite connection pool state
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Some(Account))` - Complete account information including all fields
/// - `Ok(None)` - No account found with the given ID (or account is archived)
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const account = await invoke('get_account', { id: 123 });
/// if (account) {
///   console.log(`Found: ${account.name} (${account.account_type})`);
///   console.log(`Balance: $${account.current_balance || 0}`);
///   console.log(`Institution: ${account.institution || 'N/A'}`);
/// }
/// ```
#[tauri::command]
async fn get_account(
    id: i64,
    state: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Option<database::Account>, String> {
    database::get_account(&state, id)
        .await
        .map_err(|e| e.to_string())
}

/// Updates an existing account with new information.
/// 
/// This Tauri command modifies an existing account record with the provided data
/// while preserving system-managed fields. Only non-archived accounts can be updated.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// * `account_id` - The ID of the account to update
/// * `request` - Account update data including name, type, and optional fields
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Account successfully updated
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Request Fields
/// 
/// Same as account creation: name, account_type, institution, current_balance,
/// display_order, include_in_net_worth, account_number_last4
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const request = {
///   name: "Updated Account Name",
///   account_type: "savings",
///   institution: "New Bank",
///   current_balance: 2000.50,
///   display_order: 1,
///   include_in_net_worth: true,
///   account_number_last4: "9876"
/// };
/// await invoke('update_account', { accountId: 123, request });
/// ```
#[tauri::command]
async fn update_account(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    account_id: i64,
    request: database::CreateAccountRequest,
) -> Result<(), String> {
    database::update_account(&pool, account_id, &request)
        .await
        .map_err(|e| format!("Failed to update account: {}", e))
}

/// Archives (soft deletes) an account by marking it as archived.
/// 
/// This Tauri command safely removes an account from active use by setting
/// the archived flag to true, while preserving all data and transaction history.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// * `account_id` - The ID of the account to archive
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Account successfully archived
/// - `Err(String)` - Formatted error message if operation fails
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// await invoke('archive_account', { accountId: 123 });
/// ```
#[tauri::command]
async fn archive_account(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    account_id: i64,
) -> Result<(), String> {
    database::archive_account(&pool, account_id)
        .await
        .map_err(|e| format!("Failed to archive account: {}", e))
}

/// Creates a new category in the database.
/// 
/// This Tauri command accepts a category creation request with all user-settable
/// fields and inserts it into the categories table. Database-managed fields like timestamps
/// and ID are handled automatically.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// * `request` - Category creation data including name and optional fields
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(i64)` - The ID of the newly created category
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Request Fields
/// 
/// - `name` - Category display name (required)
/// - `display_order` - Sort order for category listing (optional)
/// - `parent_category_id` - Parent category for hierarchical structure (optional)
/// - `default_discretionary` - Default discretionary spending flag (optional)
/// - `default_fixed` - Default fixed expense flag (optional)
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const request = {
///   name: "Groceries",
///   display_order: 10,
///   parent_category_id: 1, // Food category
///   default_discretionary: true,
///   default_fixed: false
/// };
/// const categoryId = await invoke('add_category', { request });
/// ```
#[tauri::command]
async fn add_category(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    request: database::CreateCategoryRequest,
) -> Result<i64, String> {
    match database::insert_category(&pool, &request).await {
        Ok(id) => Ok(id),
        Err(e) => Err(format!("Failed to add category: {}", e)),
    }
}

/// Retrieves all non-archived categories from the database.
/// 
/// This Tauri command fetches all active categories, including their
/// complete metadata such as hierarchy information, display order, and system flags.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Vec<Category>)` - Vector of Category structs with all fields populated
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Database Behavior
/// 
/// - Only returns categories where `archived = FALSE`
/// - Results are ordered by `display_order` first, then alphabetically by `name`
/// - Includes both system and user-created categories
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const categories = await invoke('get_categories');
/// ```
#[tauri::command]
async fn get_categories(
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Vec<database::Category>, String> {
    database::get_categories(&pool)
        .await
        .map_err(|e| format!("Failed to get categories: {}", e))
}

/// Retrieves a specific category by its unique ID.
/// 
/// This Tauri command fetches complete category information for a single category,
/// returning the same full Category struct as get_categories() for consistency.
/// Only returns data for non-archived categories.
/// 
/// # Arguments
/// 
/// * `id` - The unique ID of the category to retrieve
/// * `pool` - Tauri-managed SQLite connection pool state
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Some(Category))` - Complete category information including all fields
/// - `Ok(None)` - No category found with the given ID (or category is archived)
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const category = await invoke('get_category', { id: 123 });
/// if (category) {
///   console.log(`Found: ${category.name}`);
///   console.log(`System category: ${category.is_system_category}`);
/// }
/// ```
#[tauri::command]
async fn get_category(
    id: i64,
    pool: tauri::State<'_, sqlx::SqlitePool>,
) -> Result<Option<database::Category>, String> {
    database::get_category(&pool, id)
        .await
        .map_err(|e| e.to_string())
}

/// Updates an existing category with new information.
/// 
/// This Tauri command modifies an existing category record with the provided data
/// while preserving system-managed fields. Only non-system categories can be updated.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// * `category_id` - The ID of the category to update
/// * `request` - Category update data including name and optional fields
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Category successfully updated
/// - `Err(String)` - Formatted error message if database operation fails
/// 
/// # Request Fields
/// 
/// Same as category creation: name, display_order, parent_category_id,
/// default_discretionary, default_fixed
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// const request = {
///   name: "Updated Category Name",
///   display_order: 5,
///   parent_category_id: 2,
///   default_discretionary: false,
///   default_fixed: true
/// };
/// await invoke('update_category', { categoryId: 123, request });
/// ```
#[tauri::command]
async fn update_category(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    category_id: i64,
    request: database::CreateCategoryRequest,
) -> Result<(), String> {
    database::update_category(&pool, category_id, &request)
        .await
        .map_err(|e| format!("Failed to update category: {}", e))
}

/// Archives (soft deletes) a category by marking it as archived.
/// 
/// This Tauri command safely removes a category from active use by setting
/// the archived flag to true, while preserving all data and transaction history.
/// System categories cannot be archived.
/// 
/// # Arguments
/// 
/// * `pool` - Tauri-managed SQLite connection pool state
/// * `category_id` - The ID of the category to archive
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Category successfully archived
/// - `Err(String)` - Formatted error message if operation fails
/// 
/// # Frontend Usage
/// 
/// ```javascript
/// await invoke('archive_category', { categoryId: 123 });
/// ```
#[tauri::command]
async fn archive_category(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    category_id: i64,
) -> Result<(), String> {
    database::archive_category(&pool, category_id)
        .await
        .map_err(|e| format!("Failed to archive category: {}", e))
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
