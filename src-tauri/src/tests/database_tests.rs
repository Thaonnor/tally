use crate::database::*;
use sqlx::SqlitePool;

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
    update_account(&pool, account_id, &update_request).await.unwrap();

    // Fetch the updated account and verify changes
    let updated_account = get_account(&pool, account_id).await.unwrap().unwrap();
    
    assert_eq!(updated_account.id, account_id); // ID should remain the same
    assert_eq!(updated_account.name, "Updated Account Name");
    assert_eq!(updated_account.account_type, "savings");
    assert_eq!(updated_account.institution, Some("New Bank".to_string()));
    assert_eq!(updated_account.current_balance, Some(2500.50));
    assert_eq!(updated_account.display_order, Some(3));
    assert_eq!(updated_account.include_in_net_worth, false);
    assert_eq!(updated_account.account_number_last4, Some("9876".to_string()));
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