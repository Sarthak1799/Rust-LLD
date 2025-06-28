use std::collections::HashMap;
use std::sync::Arc;

use crate::models::payment_method::PaymentMethodInterface;
use crate::models::transaction::Transaction;
use crate::utils::generate_random_id;

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub account_ids: Vec<String>,
    pub payment_methods: Vec<Arc<dyn PaymentMethodInterface>>,
    pub transactions: HashMap<String, Transaction>,
}

impl User {
    pub fn new(name: String, email: String, account_ids: Vec<String>) -> Self {
        let id = generate_random_id("user");
        User {
            id,
            name,
            email,
            account_ids,
            payment_methods: Vec::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn add_payment_method(&mut self, payment_method: Arc<dyn PaymentMethodInterface>) {
        self.payment_methods.push(payment_method);
    }

    pub fn does_payment_method_exist(
        &self,
        payment_method: Arc<dyn PaymentMethodInterface>,
    ) -> bool {
        self.payment_methods
            .iter()
            .any(|pm| pm.get_name() == payment_method.get_name())
    }

    pub fn does_account_exist(&self, account_id: &str) -> bool {
        self.account_ids.contains(&account_id.to_string())
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions
            .insert(transaction.get_id().to_string(), transaction);
    }

    pub fn list_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    pub fn add_account(&mut self, account_id: String) {
        if !self.does_account_exist(&account_id) {
            self.account_ids.push(account_id);
        }
    }
    pub fn remove_account(&mut self, account_id: &str) {
        self.account_ids.retain(|id| id != account_id);
    }
}
