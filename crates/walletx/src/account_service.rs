use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::models::account::Account;

#[derive(Debug)]
pub enum AccountError {
    AccountNotFound,
    GenericAccountError(String),
}
impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountError::AccountNotFound => write!(f, "Account not found"),
            AccountError::GenericAccountError(msg) => write!(f, "Account error: {}", msg),
        }
    }
}

pub struct AccountService {
    pub accounts: HashMap<String, RwLock<Account>>,
}

impl AccountService {
    pub fn new() -> Self {
        AccountService {
            accounts: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts
            .insert(account.get_id().to_string(), RwLock::new(account));
    }

    pub fn get_account_read(
        &self,
        account_id: &str,
    ) -> Result<RwLockReadGuard<Account>, AccountError> {
        let account = self
            .accounts
            .get(account_id)
            .ok_or(AccountError::AccountNotFound)?;
        let read_lock = account
            .read()
            .map_err(|err| AccountError::GenericAccountError(err.to_string()))?;
        Ok(read_lock)
    }

    pub fn remove_account(&mut self, account_id: &str) -> Option<RwLock<Account>> {
        self.accounts.remove(account_id)
    }

    pub fn get_account_write(
        &self,
        account_id: &str,
    ) -> Result<RwLockWriteGuard<Account>, AccountError> {
        let account = self
            .accounts
            .get(account_id)
            .ok_or(AccountError::AccountNotFound)?;
        let write_lock = account
            .write()
            .map_err(|err| AccountError::GenericAccountError(err.to_string()))?;
        Ok(write_lock)
    }
}
