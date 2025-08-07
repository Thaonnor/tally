use crate::database::*;
use sqlx::SqlitePool;

// Helper function to create a test database with all tables
async fn create_test_pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    create_accounts_table(&pool).await.unwrap();
    create_categories_table(&pool).await.unwrap();
    create_transactions_table(&pool).await.unwrap();
    create_transfers_table(&pool).await.unwrap();
    seed_default_categories(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_insert_account() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // Set up database schema
    create_accounts_table(&pool).await.unwrap();

    // Create test account request
    let request = CreateAccountRequest {
        name: "Test Checking Account".to_string(),
        account_type: "checking".to_string(),
        institution: Some("Test Bank".to_string()),
        current_balance: Some(1000.50),
        display_order: Some(1),
        include_in_net_worth: Some(true),
        account_number_last4: Some("1234".to_string()),
    };

    // Insert account and verify we get an ID back
    let account_id = insert_account(&pool, &request).await.unwrap();
    assert!(account_id > 0);

    // Verify the account was inserted correctly by fetching it
    let account = get_account(&pool, account_id).await.unwrap().unwrap();

    assert_eq!(account.name, "Test Checking Account");
    assert_eq!(account.account_type, "checking");
    assert_eq!(account.institution, Some("Test Bank".to_string()));
    assert_eq!(account.current_balance, Some(1000.50));
    assert_eq!(account.display_order, Some(1));
    assert_eq!(account.include_in_net_worth, true);
    assert_eq!(account.account_number_last4, Some("1234".to_string()));
    assert_eq!(account.archived, false);
}

#[tokio::test]
async fn test_get_account() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // Set up database schema
    create_accounts_table(&pool).await.unwrap();

    // Insert a test account first
    let request = CreateAccountRequest {
        name: "Savings Account".to_string(),
        account_type: "savings".to_string(),
        institution: Some("Credit Union".to_string()),
        current_balance: Some(2500.75),
        display_order: Some(2),
        include_in_net_worth: Some(true),
        account_number_last4: Some("5678".to_string()),
    };
    let account_id = insert_account(&pool, &request).await.unwrap();

    // Test getting the account by ID
    let account = get_account(&pool, account_id).await.unwrap().unwrap();

    assert_eq!(account.id, account_id);
    assert_eq!(account.name, "Savings Account");
    assert_eq!(account.account_type, "savings");
    assert_eq!(account.institution, Some("Credit Union".to_string()));
    assert_eq!(account.current_balance, Some(2500.75));
    assert_eq!(account.display_order, Some(2));
    assert_eq!(account.include_in_net_worth, true);
    assert_eq!(account.account_number_last4, Some("5678".to_string()));
    assert_eq!(account.archived, false);

    // Test getting a non-existent account
    let non_existent = get_account(&pool, 99999).await.unwrap();
    assert!(non_existent.is_none());
}

#[tokio::test]
async fn test_get_accounts() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // Set up database schema
    create_accounts_table(&pool).await.unwrap();

    // Initially should have no accounts
    let accounts = get_accounts(&pool).await.unwrap();
    assert_eq!(accounts.len(), 0);

    // Insert multiple test accounts
    let request1 = CreateAccountRequest {
        name: "Checking Account".to_string(),
        account_type: "checking".to_string(),
        institution: Some("Bank A".to_string()),
        current_balance: Some(1000.00),
        display_order: Some(1),
        include_in_net_worth: Some(true),
        account_number_last4: Some("1111".to_string()),
    };

    let request2 = CreateAccountRequest {
        name: "Savings Account".to_string(),
        account_type: "savings".to_string(),
        institution: Some("Bank B".to_string()),
        current_balance: Some(5000.00),
        display_order: Some(2),
        include_in_net_worth: Some(true),
        account_number_last4: Some("2222".to_string()),
    };

    let request3 = CreateAccountRequest {
        name: "Credit Card".to_string(),
        account_type: "credit".to_string(),
        institution: Some("Bank C".to_string()),
        current_balance: Some(-250.50),
        display_order: Some(3),
        include_in_net_worth: Some(true),
        account_number_last4: Some("3333".to_string()),
    };

    insert_account(&pool, &request1).await.unwrap();
    insert_account(&pool, &request2).await.unwrap();
    insert_account(&pool, &request3).await.unwrap();

    // Test getting all accounts
    let accounts = get_accounts(&pool).await.unwrap();
    assert_eq!(accounts.len(), 3);

    // Verify accounts are ordered by display_order, then name
    assert_eq!(accounts[0].name, "Checking Account");
    assert_eq!(accounts[0].display_order, Some(1));

    assert_eq!(accounts[1].name, "Savings Account");
    assert_eq!(accounts[1].display_order, Some(2));

    assert_eq!(accounts[2].name, "Credit Card");
    assert_eq!(accounts[2].display_order, Some(3));

    // Verify all accounts are non-archived
    for account in &accounts {
        assert_eq!(account.archived, false);
    }
}

#[tokio::test]
async fn test_update_account() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // Set up database schema
    create_accounts_table(&pool).await.unwrap();

    // Insert a test account first
    let original_request = CreateAccountRequest {
        name: "Original Name".to_string(),
        account_type: "checking".to_string(),
        institution: Some("Original Bank".to_string()),
        current_balance: Some(1000.00),
        display_order: Some(1),
        include_in_net_worth: Some(true),
        account_number_last4: Some("1234".to_string()),
    };
    let account_id = insert_account(&pool, &original_request).await.unwrap();

    // Update the account with new values
    let update_request = CreateAccountRequest {
        name: "Updated Account Name".to_string(),
        account_type: "savings".to_string(),
        institution: Some("New Bank".to_string()),
        current_balance: Some(2500.50),
        display_order: Some(3),
        include_in_net_worth: Some(false),
        account_number_last4: Some("9876".to_string()),
    };

    // Perform the update
    update_account(&pool, account_id, &update_request)
        .await
        .unwrap();

    // Fetch the updated account and verify changes
    let updated_account = get_account(&pool, account_id).await.unwrap().unwrap();

    assert_eq!(updated_account.id, account_id); // ID should remain the same
    assert_eq!(updated_account.name, "Updated Account Name");
    assert_eq!(updated_account.account_type, "savings");
    assert_eq!(updated_account.institution, Some("New Bank".to_string()));
    assert_eq!(updated_account.current_balance, Some(2500.50));
    assert_eq!(updated_account.display_order, Some(3));
    assert_eq!(updated_account.include_in_net_worth, false);
    assert_eq!(
        updated_account.account_number_last4,
        Some("9876".to_string())
    );
    assert_eq!(updated_account.archived, false); // Should still be false

    // Test updating a non-existent account (should not error but have no effect)
    let result = update_account(&pool, 99999, &update_request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_archive_account() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // Set up database schema
    create_accounts_table(&pool).await.unwrap();

    // Insert test accounts
    let request1 = CreateAccountRequest {
        name: "Account to Archive".to_string(),
        account_type: "checking".to_string(),
        institution: Some("Test Bank".to_string()),
        current_balance: Some(1000.00),
        display_order: Some(1),
        include_in_net_worth: Some(true),
        account_number_last4: Some("1111".to_string()),
    };

    let request2 = CreateAccountRequest {
        name: "Account to Keep".to_string(),
        account_type: "savings".to_string(),
        institution: Some("Test Bank".to_string()),
        current_balance: Some(2000.00),
        display_order: Some(2),
        include_in_net_worth: Some(true),
        account_number_last4: Some("2222".to_string()),
    };

    let account_id_1 = insert_account(&pool, &request1).await.unwrap();
    let account_id_2 = insert_account(&pool, &request2).await.unwrap();

    // Verify both accounts exist in get_accounts (non-archived)
    let accounts_before = get_accounts(&pool).await.unwrap();
    assert_eq!(accounts_before.len(), 2);

    // Archive the first account
    archive_account(&pool, account_id_1).await.unwrap();

    // Verify get_accounts now only returns the non-archived account
    let accounts_after = get_accounts(&pool).await.unwrap();
    assert_eq!(accounts_after.len(), 1);
    assert_eq!(accounts_after[0].name, "Account to Keep");

    // Verify get_account returns None for archived account
    let archived_account = get_account(&pool, account_id_1).await.unwrap();
    assert!(archived_account.is_none());

    // Verify the non-archived account is still accessible
    let active_account = get_account(&pool, account_id_2).await.unwrap().unwrap();
    assert_eq!(active_account.name, "Account to Keep");

    // Test archiving a non-existent account (should not error but have no effect)
    let result = archive_account(&pool, 99999).await;
    assert!(result.is_ok());

    // Test archiving an already archived account (should not error but have no effect)
    let result = archive_account(&pool, account_id_1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_insert_category() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Set up database schema
    create_categories_table(&pool).await.unwrap();
    
    // Create test category request
    let request = CreateCategoryRequest {
        name: "Test Groceries".to_string(),
        display_order: Some(10),
        parent_category_id: None,
        default_discretionary: Some(true),
        default_fixed: Some(false),
    };

    // Insert category and verify we get an ID back
    let category_id = insert_category(&pool, &request).await.unwrap();
    assert!(category_id > 0);

    // Verify the category was inserted correctly by fetching it
    let category = get_category(&pool, category_id).await.unwrap().unwrap();
    
    assert_eq!(category.name, "Test Groceries");
    assert_eq!(category.display_order, Some(10));
    assert_eq!(category.parent_category_id, None);
    assert_eq!(category.default_discretionary, Some(true));
    assert_eq!(category.default_fixed, Some(false));
    assert_eq!(category.archived, false);
    assert_eq!(category.is_system_category, false);
}

#[tokio::test]
async fn test_get_category() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Set up database schema
    create_categories_table(&pool).await.unwrap();
    seed_default_categories(&pool).await.unwrap();
    
    // Insert a test category first
    let request = CreateCategoryRequest {
        name: "Entertainment".to_string(),
        display_order: Some(20),
        parent_category_id: None,
        default_discretionary: Some(true),
        default_fixed: Some(false),
    };
    let category_id = insert_category(&pool, &request).await.unwrap();

    // Test getting the category by ID
    let category = get_category(&pool, category_id).await.unwrap().unwrap();
    
    assert_eq!(category.id, category_id);
    assert_eq!(category.name, "Entertainment");
    assert_eq!(category.display_order, Some(20));
    assert_eq!(category.parent_category_id, None);
    assert_eq!(category.default_discretionary, Some(true));
    assert_eq!(category.default_fixed, Some(false));
    assert_eq!(category.archived, false);
    assert_eq!(category.is_system_category, false);

    // Test getting a non-existent category
    let non_existent = get_category(&pool, 99999).await.unwrap();
    assert!(non_existent.is_none());

    // Test getting the seeded system category
    let system_categories = get_categories(&pool).await.unwrap();
    let uncategorized = system_categories.iter().find(|c| c.name == "Uncategorized").unwrap();
    let system_category = get_category(&pool, uncategorized.id).await.unwrap().unwrap();
    assert_eq!(system_category.name, "Uncategorized");
    assert_eq!(system_category.is_system_category, true);
}

#[tokio::test]
async fn test_get_categories() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Set up database schema
    create_categories_table(&pool).await.unwrap();
    seed_default_categories(&pool).await.unwrap();
    
    // Initially should have one seeded category (Uncategorized)
    let categories = get_categories(&pool).await.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0].name, "Uncategorized");
    assert_eq!(categories[0].is_system_category, true);
    
    // Insert multiple test categories
    let request1 = CreateCategoryRequest {
        name: "Food".to_string(),
        display_order: Some(1),
        parent_category_id: None,
        default_discretionary: Some(false),
        default_fixed: Some(false),
    };
    
    let request2 = CreateCategoryRequest {
        name: "Transportation".to_string(),
        display_order: Some(2),
        parent_category_id: None,
        default_discretionary: Some(false),
        default_fixed: Some(true),
    };

    let request3 = CreateCategoryRequest {
        name: "Entertainment".to_string(),
        display_order: Some(3),
        parent_category_id: None,
        default_discretionary: Some(true),
        default_fixed: Some(false),
    };

    insert_category(&pool, &request1).await.unwrap();
    insert_category(&pool, &request2).await.unwrap();
    insert_category(&pool, &request3).await.unwrap();

    // Test getting all categories
    let categories = get_categories(&pool).await.unwrap();
    assert_eq!(categories.len(), 4); // 3 user + 1 system category
    
    // Verify categories are ordered by display_order, then name
    // Uncategorized has display_order 0, so it should be first
    assert_eq!(categories[0].name, "Uncategorized");
    assert_eq!(categories[0].display_order, Some(0));
    assert_eq!(categories[0].is_system_category, true);
    
    assert_eq!(categories[1].name, "Food");
    assert_eq!(categories[1].display_order, Some(1));
    
    assert_eq!(categories[2].name, "Transportation");
    assert_eq!(categories[2].display_order, Some(2));
    
    assert_eq!(categories[3].name, "Entertainment");
    assert_eq!(categories[3].display_order, Some(3));

    // Verify all categories are non-archived
    for category in &categories {
        assert_eq!(category.archived, false);
    }
}

#[tokio::test]
async fn test_update_category() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Set up database schema
    create_categories_table(&pool).await.unwrap();
    seed_default_categories(&pool).await.unwrap();
    
    // Insert a test category first
    let original_request = CreateCategoryRequest {
        name: "Original Category".to_string(),
        display_order: Some(5),
        parent_category_id: None,
        default_discretionary: Some(false),
        default_fixed: Some(false),
    };
    let category_id = insert_category(&pool, &original_request).await.unwrap();

    // Update the category with new values
    let update_request = CreateCategoryRequest {
        name: "Updated Category Name".to_string(),
        display_order: Some(15),
        parent_category_id: None,
        default_discretionary: Some(true),
        default_fixed: Some(true),
    };
    
    // Perform the update
    update_category(&pool, category_id, &update_request).await.unwrap();

    // Fetch the updated category and verify changes
    let updated_category = get_category(&pool, category_id).await.unwrap().unwrap();
    
    assert_eq!(updated_category.id, category_id); // ID should remain the same
    assert_eq!(updated_category.name, "Updated Category Name");
    assert_eq!(updated_category.display_order, Some(15));
    assert_eq!(updated_category.parent_category_id, None);
    assert_eq!(updated_category.default_discretionary, Some(true));
    assert_eq!(updated_category.default_fixed, Some(true));
    assert_eq!(updated_category.archived, false); // Should still be false
    assert_eq!(updated_category.is_system_category, false); // Should remain false

    // Test that system categories cannot be updated
    let categories = get_categories(&pool).await.unwrap();
    let system_category = categories.iter().find(|c| c.is_system_category).unwrap();
    let system_update = CreateCategoryRequest {
        name: "Should Not Update".to_string(),
        display_order: Some(99),
        parent_category_id: None,
        default_discretionary: Some(false),
        default_fixed: Some(false),
    };
    
    // This should succeed but have no effect (system category protection)
    update_category(&pool, system_category.id, &system_update).await.unwrap();
    
    // Verify system category was not changed
    let unchanged_system = get_category(&pool, system_category.id).await.unwrap().unwrap();
    assert_eq!(unchanged_system.name, "Uncategorized");
    assert_eq!(unchanged_system.display_order, Some(0));

    // Test updating a non-existent category (should not error but have no effect)
    let result = update_category(&pool, 99999, &update_request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_archive_category() {
    // Create in-memory database for testing
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Set up database schema
    create_categories_table(&pool).await.unwrap();
    seed_default_categories(&pool).await.unwrap();
    
    // Insert test categories
    let request1 = CreateCategoryRequest {
        name: "Category to Archive".to_string(),
        display_order: Some(10),
        parent_category_id: None,
        default_discretionary: Some(true),
        default_fixed: Some(false),
    };
    
    let request2 = CreateCategoryRequest {
        name: "Category to Keep".to_string(),
        display_order: Some(20),
        parent_category_id: None,
        default_discretionary: Some(false),
        default_fixed: Some(true),
    };
    
    let category_id_1 = insert_category(&pool, &request1).await.unwrap();
    let category_id_2 = insert_category(&pool, &request2).await.unwrap();

    // Verify we have 3 categories (2 user + 1 system)
    let categories_before = get_categories(&pool).await.unwrap();
    assert_eq!(categories_before.len(), 3);

    // Archive the first user category
    archive_category(&pool, category_id_1).await.unwrap();

    // Verify get_categories now only returns 2 categories (1 user + 1 system)
    let categories_after = get_categories(&pool).await.unwrap();
    assert_eq!(categories_after.len(), 2);
    
    // Verify the archived category is not in the list
    assert!(!categories_after.iter().any(|c| c.name == "Category to Archive"));
    
    // Verify the non-archived category is still there
    assert!(categories_after.iter().any(|c| c.name == "Category to Keep"));

    // Verify get_category returns None for archived category
    let archived_category = get_category(&pool, category_id_1).await.unwrap();
    assert!(archived_category.is_none());

    // Verify the non-archived category is still accessible
    let active_category = get_category(&pool, category_id_2).await.unwrap().unwrap();
    assert_eq!(active_category.name, "Category to Keep");

    // Test that system categories cannot be archived
    let system_categories = get_categories(&pool).await.unwrap();
    let system_category = system_categories.iter().find(|c| c.is_system_category).unwrap();
    
    // This should succeed but have no effect (system category protection)
    archive_category(&pool, system_category.id).await.unwrap();
    
    // Verify system category is still there
    let categories_final = get_categories(&pool).await.unwrap();
    assert!(categories_final.iter().any(|c| c.is_system_category && c.name == "Uncategorized"));

    // Test archiving a non-existent category (should not error but have no effect)
    let result = archive_category(&pool, 99999).await;
    assert!(result.is_ok());

    // Test archiving an already archived category (should not error but have no effect)
    let result = archive_category(&pool, category_id_1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_insert_transaction() {
    let pool = create_test_pool().await;

    // First, create an account for the transaction
    let account_request = CreateAccountRequest {
        name: "Test Account".to_string(),
        account_type: "checking".to_string(),
        institution: Some("Test Bank".to_string()),
        current_balance: Some(1000.0),
        display_order: Some(1),
        include_in_net_worth: Some(true),
        account_number_last4: Some("1234".to_string()),
    };
    let account_id = insert_account(&pool, &account_request).await.unwrap();

    // Create a category for the transaction
    let category_request = CreateCategoryRequest {
        name: "Test Category".to_string(),
        display_order: Some(1),
        parent_category_id: None,
        default_discretionary: Some(true),
        default_fixed: Some(false),
    };
    let category_id = insert_category(&pool, &category_request).await.unwrap();

    // Create a transaction
    let transaction_request = CreateTransactionRequest {
        account_id,
        date: "2024-01-15".to_string(),
        amount: 25.50,
        description: Some("Test transaction".to_string()),
        payee: Some("Test Payee".to_string()),
        memo: Some("Test memo".to_string()),
        category_id: Some(category_id),
        pending: false,
        cleared: true,
    };

    let transaction_id = insert_transaction(&pool, &transaction_request).await.unwrap();
    assert!(transaction_id > 0);

    // Verify the transaction was inserted correctly
    let transaction = get_transaction(&pool, transaction_id).await.unwrap();
    assert!(transaction.is_some());
    let transaction = transaction.unwrap();

    assert_eq!(transaction.account_id, account_id);
    assert_eq!(transaction.date, "2024-01-15");
    assert_eq!(transaction.amount, 25.50);
    assert_eq!(transaction.description, Some("Test transaction".to_string()));
    assert_eq!(transaction.payee, Some("Test Payee".to_string()));
    assert_eq!(transaction.memo, Some("Test memo".to_string()));
    assert_eq!(transaction.category_id, Some(category_id));
    assert_eq!(transaction.pending, false);
    assert_eq!(transaction.transaction_type, "expense");
}

#[tokio::test]
async fn test_get_transactions() {
    let pool = create_test_pool().await;

    // Create an account
    let account_request = CreateAccountRequest {
        name: "Test Account".to_string(),
        account_type: "checking".to_string(),
        institution: None,
        current_balance: Some(1000.0),
        display_order: None,
        include_in_net_worth: Some(true),
        account_number_last4: None,
    };
    let account_id = insert_account(&pool, &account_request).await.unwrap();

    // Create multiple transactions for the account
    let transaction1_request = CreateTransactionRequest {
        account_id,
        date: "2024-01-15".to_string(),
        amount: 25.50,
        description: Some("First transaction".to_string()),
        payee: None,
        memo: None,
        category_id: None,
        pending: false,
        cleared: true,
    };
    let transaction2_request = CreateTransactionRequest {
        account_id,
        date: "2024-01-16".to_string(),
        amount: 50.00,
        description: Some("Second transaction".to_string()),
        payee: None,
        memo: None,
        category_id: None,
        pending: true,
        cleared: false,
    };

    let _tx1_id = insert_transaction(&pool, &transaction1_request).await.unwrap();
    let _tx2_id = insert_transaction(&pool, &transaction2_request).await.unwrap();

    // Test retrieving transactions
    let transactions = get_transactions(&pool, account_id, 10, 0).await.unwrap();
    assert_eq!(transactions.len(), 2);

    // Should be ordered by date DESC, so second transaction comes first
    assert_eq!(transactions[0].description, Some("Second transaction".to_string()));
    assert_eq!(transactions[0].amount, 50.00);
    assert_eq!(transactions[0].pending, true);

    assert_eq!(transactions[1].description, Some("First transaction".to_string()));
    assert_eq!(transactions[1].amount, 25.50);
    assert_eq!(transactions[1].pending, false);

    // Test pagination
    let limited_transactions = get_transactions(&pool, account_id, 1, 0).await.unwrap();
    assert_eq!(limited_transactions.len(), 1);
    assert_eq!(limited_transactions[0].description, Some("Second transaction".to_string()));

    let offset_transactions = get_transactions(&pool, account_id, 1, 1).await.unwrap();
    assert_eq!(offset_transactions.len(), 1);
    assert_eq!(offset_transactions[0].description, Some("First transaction".to_string()));
}

#[tokio::test]
async fn test_seed_default_categories() {
    let pool = create_test_pool().await;

    // The create_test_pool already calls seed_default_categories, so let's verify it worked
    let categories = get_categories(&pool).await.unwrap();
    assert_eq!(categories.len(), 1);
    
    let uncategorized = &categories[0];
    assert_eq!(uncategorized.name, "Uncategorized");
    assert_eq!(uncategorized.is_system_category, true);
    assert_eq!(uncategorized.archived, false);
    assert_eq!(uncategorized.display_order, Some(0));

    // Test that calling seed again doesn't create a duplicate
    seed_default_categories(&pool).await.unwrap();
    
    let categories_after_second_call = get_categories(&pool).await.unwrap();
    assert_eq!(categories_after_second_call.len(), 1);
    assert_eq!(categories_after_second_call[0].id, uncategorized.id);

    // Test on fresh database (without using create_test_pool)
    let fresh_pool = SqlitePool::connect(":memory:").await.unwrap();
    create_categories_table(&fresh_pool).await.unwrap();
    
    // Should have no categories initially
    let empty_categories = get_categories(&fresh_pool).await.unwrap();
    assert_eq!(empty_categories.len(), 0);
    
    // Seed should create the Uncategorized category
    seed_default_categories(&fresh_pool).await.unwrap();
    
    let seeded_categories = get_categories(&fresh_pool).await.unwrap();
    assert_eq!(seeded_categories.len(), 1);
    assert_eq!(seeded_categories[0].name, "Uncategorized");
    assert_eq!(seeded_categories[0].is_system_category, true);
}
