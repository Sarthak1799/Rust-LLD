use std::sync::Arc;

use walletx::{
    models::{
        account::Account,
        payment_method::{BankAccount, Card},
        user::User,
    },
    wallet_service::WalletService,
};
fn main() {
    let mut wallet_service = WalletService::new();
    let user1 = User::new(
        "Alice".to_string(),
        "something@example.com".to_string(),
        vec![],
    );
    let user2 = User::new(
        "Bob".to_string(),
        "something@example.com".to_string(),
        vec![],
    );
    let user_id = user1.get_id().to_string();
    let user_id2 = user2.get_id().to_string();

    let account1 = Account::new(user_id.clone(), 1000.0);
    let account2 = Account::new(user_id2.clone(), 2000.0);

    let account_id = account1.get_id().to_string();

    let account_id2 = account2.get_id().to_string();

    wallet_service.add_user(user1);
    wallet_service.add_user(user2);

    wallet_service.add_account(account1);
    wallet_service.add_account(account2);

    wallet_service
        .add_account_to_user(&user_id, &account_id)
        .expect("Failed to add account to user");
    wallet_service
        .add_account_to_user(&user_id2, &account_id2)
        .expect("Failed to add account to user");

    wallet_service
        .add_payment_method_to_user(&user_id, Arc::new(Card))
        .expect("Failed to add payment method to user");
    wallet_service
        .add_payment_method_to_user(&user_id2, Arc::new(BankAccount))
        .expect("Failed to add payment method to user");

    wallet_service
        .create_transaction(&account_id, &account_id2, 500.0, Arc::new(Card))
        .expect("Failed to create transaction");

    wallet_service
        .create_transaction(&account_id, &account_id2, 600.0, Arc::new(Card))
        .expect("Failed to create transaction");
}
