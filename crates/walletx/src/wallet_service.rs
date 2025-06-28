use std::sync::Arc;

use crate::{
    account_service::AccountService,
    models::{
        account::Account, payment_method::PaymentMethodInterface, transaction::Transaction,
        user::User,
    },
    user_service::UserService,
};

pub struct WalletService {
    pub account_service: AccountService,
    pub user_service: UserService,
}

#[derive(Debug)]
pub enum ServiceError {
    GenericServiceError(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::GenericServiceError(msg) => write!(f, "{}", msg),
        }
    }
}

impl WalletService {
    pub fn new() -> Self {
        WalletService {
            account_service: AccountService::new(),
            user_service: UserService::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.user_service.add_user(user);
    }

    pub fn get_user(&self, user_id: &str) -> Result<&User, ServiceError> {
        self.user_service
            .get_user(user_id)
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))
    }

    pub fn remove_user(&mut self, user_id: &str) -> Option<User> {
        self.user_service.remove_user(user_id)
    }

    pub fn list_users(&self) -> Vec<&User> {
        self.user_service.list_users()
    }

    pub fn add_account(&mut self, account: Account) {
        self.account_service.add_account(account);
    }

    pub fn add_account_to_user(
        &mut self,
        user_id: &str,
        account_id: &str,
    ) -> Result<(), ServiceError> {
        self.user_service
            .add_account_to_user(user_id, account_id)
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;

        Ok(())
    }

    pub fn add_payment_method_to_user(
        &mut self,
        user_id: &str,
        payment_method: Arc<dyn PaymentMethodInterface>,
    ) -> Result<(), ServiceError> {
        self.user_service
            .add_payment_method_to_user(user_id, payment_method)
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;

        Ok(())
    }

    pub fn create_transaction(
        &mut self,
        from_account: &str,
        to_account: &str,
        amount: f64,
        payment_method: Arc<dyn PaymentMethodInterface>,
    ) -> Result<(), ServiceError> {
        let mut from_account = self
            .account_service
            .get_account_write(from_account)
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;
        let mut to_account = self
            .account_service
            .get_account_write(to_account)
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;

        if from_account.get_balance() < amount {
            return Err(ServiceError::GenericServiceError(
                "Insufficient funds".to_string(),
            ));
        }

        let user1 = from_account.get_user_id().to_string();
        let user2 = to_account.get_user_id().to_string();

        self.user_service
            .payment_method_exists_for_user(&user1, payment_method.clone())
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;

        let transaction = Transaction::new(
            from_account.get_id().to_string(),
            to_account.get_id().to_string(),
            amount,
            payment_method.get_name(),
        );

        from_account.withdraw(amount);
        to_account.deposit(amount);

        self.user_service
            .add_transaction_to_user(&user1, transaction.clone())
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;
        self.user_service
            .add_transaction_to_user(&user2, transaction)
            .map_err(|err| ServiceError::GenericServiceError(err.to_string()))?;

        Ok(())
    }
}
