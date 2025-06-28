use std::{collections::HashMap, sync::Arc};

use crate::models::{payment_method::PaymentMethodInterface, transaction::Transaction, user::User};

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    GenericUserError(String),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::UserNotFound => write!(f, "User not found"),
            UserError::GenericUserError(msg) => write!(f, "User error: {}", msg),
        }
    }
}

pub struct UserService {
    pub users: HashMap<String, User>,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.get_id().to_string(), user);
    }

    pub fn get_user(&self, user_id: &str) -> Result<&User, UserError> {
        self.users.get(user_id).ok_or(UserError::UserNotFound)
    }

    pub fn remove_user(&mut self, user_id: &str) -> Option<User> {
        self.users.remove(user_id)
    }

    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn add_payment_method_to_user(
        &mut self,
        user_id: &str,
        payment_method: Arc<dyn PaymentMethodInterface>,
    ) -> Result<(), UserError> {
        let user = self.users.get_mut(user_id).ok_or(UserError::UserNotFound)?;
        user.add_payment_method(payment_method);
        Ok(())
    }

    pub fn payment_method_exists_for_user(
        &self,
        user_id: &str,
        payment_method: Arc<dyn PaymentMethodInterface>,
    ) -> Result<(), UserError> {
        let user = self.users.get(user_id).ok_or(UserError::UserNotFound)?;
        if !user.does_payment_method_exist(payment_method) {
            return Err(UserError::GenericUserError(
                "Payment method does not exist for this user".to_string(),
            ));
        }
        Ok(())
    }

    pub fn add_transaction_to_user(
        &mut self,
        user_id: &str,
        transaction: Transaction,
    ) -> Result<(), UserError> {
        let user = self.users.get_mut(user_id).ok_or(UserError::UserNotFound)?;
        user.add_transaction(transaction);
        Ok(())
    }

    pub fn add_account_to_user(
        &mut self,
        user_id: &str,
        account_id: &str,
    ) -> Result<(), UserError> {
        let user = self.users.get_mut(user_id).ok_or(UserError::UserNotFound)?;
        if user.does_account_exist(&account_id) {
            return Err(UserError::GenericUserError(
                "Account already exists for this user".to_string(),
            ));
        }
        user.add_account(account_id.to_string());
        Ok(())
    }
}
