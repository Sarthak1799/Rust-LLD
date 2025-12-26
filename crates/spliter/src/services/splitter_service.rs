use std::{collections::HashMap, sync::Arc};

use crate::models::{
    expense::Expense, group::Group, split::Split, transaction::Transaction, user::User,
};

#[derive(Debug)]
pub enum ServiceError {
    UserNotFound,
    UserAlreadyExists,
    GroupNotFound,
    ExpenseNotFound,
    GenericServiceError(&'static str),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::UserNotFound => write!(f, "User not found"),
            ServiceError::UserAlreadyExists => write!(f, "User already exists"),
            ServiceError::GroupNotFound => write!(f, "Group not found"),
            ServiceError::ExpenseNotFound => write!(f, "Expense not found"),
            ServiceError::GenericServiceError(msg) => write!(f, "Service error: {}", msg),
        }
    }
}

pub struct Service {
    pub users: HashMap<String, User>,
    pub groups: HashMap<String, Group>,
    pub expenses: HashMap<String, Expense>,
    pub balances: HashMap<String, HashMap<String, f64>>, // user_id -> Vec<(other_user_id, amount_owed)>
    pub transactions_log: Vec<Transaction>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            expenses: HashMap::new(),
            balances: HashMap::new(),
            transactions_log: Vec::new(),
            groups: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, name: String, email: String) -> String {
        let user = User::new(name, email);
        let user_id = user.get_id().clone();
        self.users.insert(user_id.clone(), user);
        user_id
    }

    pub fn get_user(&self, id: &String) -> Result<&User, ServiceError> {
        self.users.get(id).ok_or(ServiceError::UserNotFound)
    }

    pub fn create_group(&mut self, member_ids: Vec<String>) -> String {
        let group = Group::new(member_ids);
        let group_id = group.get_id().clone();
        self.groups.insert(group_id.clone(), group);
        group_id
    }

    pub fn add_expense_to_group(
        &mut self,
        group_id: &String,
        description: String,
        amount: f64,
        paid_by: String,
        splits: Vec<Arc<dyn Split>>,
    ) -> Result<String, ServiceError> {
        let group = self
            .groups
            .get_mut(group_id)
            .ok_or(ServiceError::GroupNotFound)?;

        println!("Adding expense to group: {}", group_id);

        for split in &splits {
            let shares = split.calculate_shares();
            for (user_id, share_amount) in shares {
                self.users.get(&user_id).ok_or(ServiceError::UserNotFound)?;

                // Skip if the user is the one who paid (they don't owe themselves)
                if user_id == paid_by {
                    continue;
                }

                // Track that user_id owes paid_by the share_amount
                self.balances
                    .entry(user_id.clone())
                    .or_insert_with(HashMap::new)
                    .entry(paid_by.clone())
                    .and_modify(|e| *e += share_amount)
                    .or_insert(share_amount);
            }
        }

        println!("Expense shares calculated and balances updated.");

        let expense = Expense::new(description, amount, paid_by.clone(), splits);
        let expense_id = expense.get_id().clone();
        self.expenses.insert(expense_id.clone(), expense);
        group.add_expense(expense_id.clone());

        Ok(expense_id)
    }

    pub fn get_balance_sheet(
        &self,
        user_id: &String,
    ) -> Result<HashMap<String, f64>, ServiceError> {
        self.users.get(user_id).ok_or(ServiceError::UserNotFound)?;

        let balance_sheet = self
            .balances
            .get(user_id)
            .cloned()
            .unwrap_or_else(HashMap::new);

        Ok(balance_sheet)
    }

    pub fn settle_balance(
        &mut self,
        from_user_id: &String,
        to_user_id: &String,
    ) -> Result<(), ServiceError> {
        self.users
            .get(from_user_id)
            .ok_or(ServiceError::UserNotFound)?;
        self.users
            .get(to_user_id)
            .ok_or(ServiceError::UserNotFound)?;

        let amount = self
            .balances
            .get(from_user_id)
            .ok_or(ServiceError::GenericServiceError("No balance is owed"))?
            .get(to_user_id)
            .ok_or(ServiceError::GenericServiceError("No balance is owed"))?;

        let txn = Transaction::new(from_user_id.clone(), to_user_id.clone(), *amount);

        self.balances
            .get_mut(from_user_id)
            .ok_or(ServiceError::GenericServiceError("No balance is owed"))?
            .remove(to_user_id);

        self.transactions_log.push(txn);
        Ok(())
    }
}
