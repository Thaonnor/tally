use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, sqlite::SqlitePool, Row};
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub account_id: i64,
    pub date: String,
    pub amount: f64,
    pub description: String,
    pub category_id: i64,
    pub pending: bool,
    pub transaction_type: String,
    pub created_at: String,
    pub reconciled: bool,
    pub import_id: Option<String>,
    pub source: Option<String>,
    pub payee: Option<String>,
    pub original_description: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub account_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub current_balance: Option<f64>,
    pub institution: Option<String>,
    pub display_order: Option<i32>,
    pub archived: bool,
    pub include_in_net_worth: bool,
    pub account_number_last4: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub archived: bool,
    pub created_at: String,
    pub display_order: Option<i32>,
    pub parent_category_id: Option<i64>,
    pub default_discretionary: Option<bool>,
    pub default_fixed: Option<bool>,
    pub last_used_date: Option<String>,
    pub is_system_category: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub display_order: Option<i32>,
    pub parent_category_id: Option<i64>,
    pub default_discretionary: Option<bool>,
    pub default_fixed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub account_type: String,
    pub institution: Option<String>,
    pub current_balance: Option<f64>,
    pub display_order: Option<i32>,
    pub include_in_net_worth: Option<bool>,
    pub account_number_last4: Option<String>,
}

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

/// Creates the accounts table if it doesn't already exist.
/// 
/// Initializes the accounts table schema with all necessary columns including
/// account metadata, balance tracking, and soft delete support.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Table created or already exists
/// - `Err(sqlx::Error)` - Database schema creation error
/// 
/// # Database Schema
/// 
/// Creates table with columns: id, name, type, created_at, updated_at,
/// current_balance (in cents), institution, display_order, archived,
/// include_in_net_worth, account_number_last4
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
            last_used_date DATETIME,
            is_system_category BOOLEAN DEFAULT FALSE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Seeds the default "Uncategorized" system category if it doesn't exist.
/// 
/// Creates the system category that serves as a fallback for transactions
/// without explicit categorization. This category cannot be deleted or archived
/// by users, ensuring data integrity.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - System category seeded or already exists
/// - `Err(sqlx::Error)` - Database operation error
pub async fn seed_default_categories(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Check if Uncategorized category already exists
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM categories WHERE name = 'Uncategorized' AND is_system_category = TRUE"
    )
    .fetch_one(pool)
    .await?;

    if existing == 0 {
        sqlx::query(
            r#"INSERT INTO categories (name, is_system_category, display_order, archived) 
               VALUES ('Uncategorized', TRUE, 0, FALSE)"#
        )
        .execute(pool)
        .await?;
    }

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

/// Retrieves a specific account by its ID.
/// 
/// Fetches complete account information for a single non-archived account,
/// returning the same full Account struct as get_accounts() for consistency.
/// Currency amounts are automatically converted from cents to dollars.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `id` - The unique ID of the account to retrieve
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Some(Account))` - Complete account information
/// - `Ok(None)` - No account found with given ID (or account is archived)
/// - `Err(sqlx::Error)` - Database query error
/// 
/// # Examples
/// 
/// ```rust
/// let account = get_account(&pool, 123).await?;
/// if let Some(account) = account {
///     println!("Found account: {} ({})", account.name, account.account_type);
///     println!("Balance: ${:.2}", account.current_balance.unwrap_or(0.0));
/// }
/// ```
pub async fn get_account(
    pool: &Pool<Sqlite>,
    id: i64,
) -> Result<Option<Account>, sqlx::Error> {
    let account = sqlx::query_as::<_, (i64, String, String, String, String, Option<i64>, Option<String>, Option<i32>, bool, bool, Option<String>)>(
        "SELECT id, name, type, created_at, updated_at, current_balance, institution, display_order, archived, include_in_net_worth, account_number_last4 FROM accounts WHERE id = ? AND archived = FALSE",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let result = account.map(|(id, name, account_type, created_at, updated_at, balance_cents, institution, display_order, archived, include_in_net_worth, account_number_last4)| {
        Account {
            id,
            name,
            account_type,
            created_at,
            updated_at,
            current_balance: cents_to_dollars_option(balance_cents),
            institution,
            display_order,
            archived,
            include_in_net_worth,
            account_number_last4,
        }
    });

    Ok(result)
}

/// Retrieves all non-archived accounts from the database.
/// 
/// Returns a vector of Account structs with all fields populated, including
/// currency conversion from integer cents to floating-point dollars for the balance.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Vec<Account>)` - Vector of Account structs with all database fields
/// - `Err(sqlx::Error)` - Database query or connection error
/// 
/// # Ordering
/// 
/// Results are ordered by `display_order` first, then by `name` alphabetically.
/// 
/// # Examples
/// 
/// ```rust
/// let accounts = get_accounts(&pool).await?;
/// for account in accounts {
///     println!("{}: ${:.2}", account.name, account.current_balance.unwrap_or(0.0));
/// }
/// ```
pub async fn get_accounts(
    pool: &Pool<Sqlite>,
) -> Result<Vec<Account>, sqlx::Error> {
    let accounts = sqlx::query_as::<_, (i64, String, String, String, String, Option<i64>, Option<String>, Option<i32>, bool, bool, Option<String>)>(
       "SELECT id, name, type, created_at, updated_at, current_balance, institution, display_order, archived, include_in_net_worth, account_number_last4 FROM accounts WHERE archived = FALSE ORDER BY display_order, name",
   ).fetch_all(pool).await?;

    let result = accounts
        .into_iter()
        .map(|(id, name, account_type, created_at, updated_at, balance_cents, institution, display_order, archived, include_in_net_worth, account_number_last4)| {
            Account {
                id,
                name,
                account_type,
                created_at,
                updated_at, 
                current_balance: cents_to_dollars_option(balance_cents),
                institution,
                display_order,
                archived,
                include_in_net_worth,
                account_number_last4,
            }
        })
        .collect();

    Ok(result)
}

/// Inserts a new account into the database with all user-provided fields.
/// 
/// Creates a new account record with automatic timestamp generation and default values
/// for system-managed fields. Currency amounts are automatically converted from dollars
/// to integer cents for precise storage.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `request` - Account creation request containing all user-settable fields
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(i64)` - The auto-generated ID of the newly inserted account
/// - `Err(sqlx::Error)` - Database operation error (constraint violations, connection issues, etc.)
/// 
/// # Database Behavior
/// 
/// - `created_at` and `updated_at` are set to CURRENT_TIMESTAMP automatically
/// - `archived` defaults to FALSE for new accounts
/// - `include_in_net_worth` defaults to TRUE if not specified in request
/// - `current_balance` is converted from dollars to cents before storage
/// - Returns the `last_insert_rowid()` as the new account ID
/// 
/// # Examples
/// 
/// ```rust
/// let request = CreateAccountRequest {
///     name: "Checking Account".to_string(),
///     account_type: "checking".to_string(),
///     institution: Some("Bank of America".to_string()),
///     current_balance: Some(1000.50),
///     display_order: Some(1),
///     include_in_net_worth: Some(true),
///     account_number_last4: Some("1234".to_string()),
/// };
/// let account_id = insert_account(&pool, &request).await?;
/// ```
pub async fn insert_account(
    pool: &Pool<Sqlite>,
    request: &CreateAccountRequest,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"INSERT INTO accounts (name, type, institution, current_balance, display_order, include_in_net_worth, account_number_last4) 
           VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&request.name)
    .bind(&request.account_type)
    .bind(&request.institution)
    .bind(dollars_to_cents_option(request.current_balance))
    .bind(request.display_order)
    .bind(request.include_in_net_worth.unwrap_or(true))
    .bind(&request.account_number_last4)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Inserts a new category into the database.
/// 
/// Creates a new category record with automatic timestamp generation and default values
/// for system-managed fields. User categories are created with is_system_category = FALSE.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `request` - Category creation request containing all user-settable fields
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(i64)` - The auto-generated ID of the newly inserted category
/// - `Err(sqlx::Error)` - Database operation error (constraint violations, connection issues, etc.)
/// 
/// # Database Behavior
/// 
/// - `created_at` is set to CURRENT_TIMESTAMP automatically
/// - `archived` defaults to FALSE for new categories
/// - `is_system_category` defaults to FALSE for user-created categories
/// - Returns the `last_insert_rowid()` as the new category ID
/// 
/// # Examples
/// 
/// ```rust
/// let request = CreateCategoryRequest {
///     name: "Groceries".to_string(),
///     display_order: Some(10),
///     parent_category_id: Some(1), // Food category
///     default_discretionary: Some(true),
///     default_fixed: Some(false),
/// };
/// let category_id = insert_category(&pool, &request).await?;
/// ```
pub async fn insert_category(
    pool: &Pool<Sqlite>,
    request: &CreateCategoryRequest,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"INSERT INTO categories (name, display_order, parent_category_id, default_discretionary, default_fixed) 
           VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&request.name)
    .bind(request.display_order)
    .bind(request.parent_category_id)
    .bind(request.default_discretionary)
    .bind(request.default_fixed)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Retrieves all non-archived categories from the database.
/// 
/// Returns a vector of Category structs with all fields populated, ordered by
/// display_order first, then alphabetically by name for consistent listing.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Vec<Category>)` - Vector of Category structs with all database fields
/// - `Err(sqlx::Error)` - Database query or connection error
/// 
/// # Ordering
/// 
/// Results are ordered by `display_order` first (NULL values last), then by `name` alphabetically.
/// This ensures system categories and user-ordered categories appear in logical order.
/// 
/// # Examples
/// 
/// ```rust
/// let categories = get_categories(&pool).await?;
/// for category in categories {
///     println!("{}: {}", category.id, category.name);
///     if category.is_system_category {
///         println!("  (System category - cannot be deleted)");
///     }
/// }
/// ```
pub async fn get_categories(
    pool: &Pool<Sqlite>,
) -> Result<Vec<Category>, sqlx::Error> {
    let categories = sqlx::query_as::<_, (i64, String, bool, String, Option<i32>, Option<i64>, Option<bool>, Option<bool>, Option<String>, bool)>(
        "SELECT id, name, archived, created_at, display_order, parent_category_id, default_discretionary, default_fixed, last_used_date, is_system_category FROM categories WHERE archived = FALSE ORDER BY display_order, name",
    ).fetch_all(pool).await?;

    let result = categories
        .into_iter()
        .map(|(id, name, archived, created_at, display_order, parent_category_id, default_discretionary, default_fixed, last_used_date, is_system_category)| {
            Category {
                id,
                name,
                archived,
                created_at,
                display_order,
                parent_category_id,
                default_discretionary,
                default_fixed,
                last_used_date,
                is_system_category,
            }
        })
        .collect();

    Ok(result)
}

/// Retrieves a specific category by its ID.
/// 
/// Fetches complete category information for a single non-archived category,
/// returning the same full Category struct as get_categories() for consistency.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `id` - The unique ID of the category to retrieve
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(Some(Category))` - Complete category information
/// - `Ok(None)` - No category found with given ID (or category is archived)
/// - `Err(sqlx::Error)` - Database query error
/// 
/// # Examples
/// 
/// ```rust
/// let category = get_category(&pool, 123).await?;
/// if let Some(category) = category {
///     println!("Found category: {}", category.name);
///     if category.is_system_category {
///         println!("This is a system category");
///     }
/// }
/// ```
pub async fn get_category(
    pool: &Pool<Sqlite>,
    id: i64,
) -> Result<Option<Category>, sqlx::Error> {
    let category = sqlx::query_as::<_, (i64, String, bool, String, Option<i32>, Option<i64>, Option<bool>, Option<bool>, Option<String>, bool)>(
        "SELECT id, name, archived, created_at, display_order, parent_category_id, default_discretionary, default_fixed, last_used_date, is_system_category FROM categories WHERE id = ? AND archived = FALSE",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    let result = category.map(|(id, name, archived, created_at, display_order, parent_category_id, default_discretionary, default_fixed, last_used_date, is_system_category)| {
        Category {
            id,
            name,
            archived,
            created_at,
            display_order,
            parent_category_id,
            default_discretionary,
            default_fixed,
            last_used_date,
            is_system_category,
        }
    });

    Ok(result)
}

/// Updates an existing category with new information.
/// 
/// Modifies an existing category record with the provided data while preserving
/// system-managed fields like timestamps and ID. System categories cannot be updated
/// to prevent accidental modification of critical categories like "Uncategorized".
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `category_id` - The ID of the category to update
/// * `request` - Category update request containing new field values
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Category successfully updated
/// - `Err(sqlx::Error)` - Database operation error (constraint violations, connection issues, category not found, etc.)
/// 
/// # Database Behavior
/// 
/// - Updates all user-settable fields with new values
/// - Only updates non-archived, non-system categories (`WHERE archived = FALSE AND is_system_category = FALSE`)
/// - Preserves `id`, `created_at`, `archived`, and `is_system_category` fields
/// 
/// # Examples
/// 
/// ```rust
/// let request = CreateCategoryRequest {
///     name: "Updated Category Name".to_string(),
///     display_order: Some(5),
///     parent_category_id: Some(2),
///     default_discretionary: Some(false),
///     default_fixed: Some(true),
/// };
/// update_category(&pool, 123, &request).await?;
/// ```
pub async fn update_category(
    pool: &Pool<Sqlite>,
    category_id: i64,
    request: &CreateCategoryRequest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE categories 
           SET name = ?, display_order = ?, parent_category_id = ?, 
               default_discretionary = ?, default_fixed = ?
           WHERE id = ? AND archived = FALSE AND is_system_category = FALSE"#,
    )
    .bind(&request.name)
    .bind(request.display_order)
    .bind(request.parent_category_id)
    .bind(request.default_discretionary)
    .bind(request.default_fixed)
    .bind(category_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Archives a category by setting its archived flag to true.
/// 
/// Soft deletes a category by marking it as archived, which will hide it from
/// standard category listings while preserving all data and transaction history.
/// System categories cannot be archived to protect critical categories like "Uncategorized".
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `category_id` - The ID of the category to archive
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Category successfully archived
/// - `Err(sqlx::Error)` - Database operation error or category not found/is system category
/// 
/// # Database Behavior
/// 
/// - Sets `archived = TRUE` for the specified category
/// - Only archives non-system categories (`WHERE is_system_category = FALSE`)
/// - Category will no longer appear in `get_categories()` results
/// - All associated transactions remain intact but may show "Uncategorized" in UI
/// 
/// # Examples
/// 
/// ```rust
/// archive_category(&pool, 123).await?;
/// ```
pub async fn archive_category(
    pool: &Pool<Sqlite>,
    category_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE categories SET archived = TRUE WHERE id = ? AND archived = FALSE AND is_system_category = FALSE",
    )
    .bind(category_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Updates an existing account with new information.
/// 
/// Modifies an existing account record with the provided data while preserving
/// system-managed fields like timestamps and ID. Currency amounts are automatically
/// converted from dollars to integer cents for precise storage.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `account_id` - The ID of the account to update
/// * `request` - Account update request containing new field values
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Account successfully updated
/// - `Err(sqlx::Error)` - Database operation error (constraint violations, connection issues, account not found, etc.)
/// 
/// # Database Behavior
/// 
/// - Updates all user-settable fields with new values
/// - `updated_at` is set to CURRENT_TIMESTAMP automatically
/// - Only updates non-archived accounts (`WHERE archived = FALSE`)
/// - Currency amounts are converted from dollars to cents before storage
/// - Preserves `id`, `created_at`, and `archived` fields
/// 
/// # Examples
/// 
/// ```rust
/// let request = CreateAccountRequest {
///     name: "Updated Account Name".to_string(),
///     account_type: "savings".to_string(),
///     institution: Some("New Bank".to_string()),
///     current_balance: Some(2000.75),
///     display_order: Some(3),
///     include_in_net_worth: Some(false),
///     account_number_last4: Some("9876".to_string()),
/// };
/// update_account(&pool, 123, &request).await?;
/// ```
pub async fn update_account(
    pool: &Pool<Sqlite>,
    account_id: i64,
    request: &CreateAccountRequest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE accounts 
           SET name = ?, type = ?, institution = ?, current_balance = ?, 
               display_order = ?, include_in_net_worth = ?, account_number_last4 = ?,
               updated_at = CURRENT_TIMESTAMP
           WHERE id = ? AND archived = FALSE"#,
    )
    .bind(&request.name)
    .bind(&request.account_type)
    .bind(&request.institution)
    .bind(dollars_to_cents_option(request.current_balance))
    .bind(request.display_order)
    .bind(request.include_in_net_worth.unwrap_or(true))
    .bind(&request.account_number_last4)
    .bind(account_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Archives an account by setting its archived flag to true.
/// 
/// Soft deletes an account by marking it as archived, which will hide it from
/// the standard account listings while preserving all data and transaction history.
/// 
/// # Arguments
/// 
/// * `pool` - SQLite connection pool reference
/// * `account_id` - The ID of the account to archive
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(())` - Account successfully archived
/// - `Err(sqlx::Error)` - Database operation error or account not found
/// 
/// # Database Behavior
/// 
/// - Sets `archived = TRUE` for the specified account
/// - Updates `updated_at` timestamp automatically
/// - Account will no longer appear in `get_accounts()` results
/// - All associated transactions remain intact
/// 
/// # Examples
/// 
/// ```rust
/// archive_account(&pool, 123).await?;
/// ```
pub async fn archive_account(
    pool: &Pool<Sqlite>,
    account_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE accounts SET archived = TRUE, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND archived = FALSE",
    )
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

pub async fn insert_transaction(
    pool: &Pool<Sqlite>,
    account_id: i64,
    date: &str,
    amount: f64,
    description: Option<&str>,
    payee: Option<&str>,
    memo: Option<&str>,
    category_id: Option<i64>,
    pending: bool,
    cleared: bool,
) -> Result<i64, sqlx::Error> {
    let amount_cents = dollars_to_cents(amount);

    let result = sqlx::query(
        r#"INSERT INTO transactions (account_id, date, amount, description, payee, memo, category_id, pending, cleared, transaction_type, source) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'expense', 'manual')"#,
    ).bind(account_id).bind(date).bind(amount_cents).bind(description).bind(payee).bind(memo).bind(category_id).bind(pending).bind(cleared).execute(pool).await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_account_transactions(
    pool: &Pool<Sqlite>,
    account_id: i64,
    limit: i32,
    offset: i32,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
            SELECT
                id, account_id, date, amount, description, category_id, pending, transaction_type, created_at, reconciled, import_id, source, payee, original_description, memo
            FROM transactions
            WHERE account_id = ?
            ORDER BY date DESC, id DESC
            LIMIT ? OFFSET ?
        "#,
    )
    .bind(account_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    
    let mut transactions = Vec::new();
    for row in rows {
        transactions.push(Transaction {
            id: row.get("id"),
            account_id: row.get("account_id"),
            date: row.get("date"),
            amount: cents_to_dollars(row.get("amount")),
            description: row.get("description"),
            category_id: row.get("category_id"),
            pending: row.get("pending"),
            transaction_type: row.get("transaction_type"),
            created_at: row.get("created_at"),
            reconciled: row.get("reconciled"),
            import_id: row.get("import_id"),
            source: row.get("source"),
            payee: row.get("payee"),
            original_description: row.get("original_description"),
            memo: row.get("memo")
        });
    }

    Ok(transactions)
}
