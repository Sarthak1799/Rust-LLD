use std::sync::Arc;

use crate::{models::split::Split, utils::generate_uuid};

#[derive(Clone)]
pub struct Expense {
    id: String,
    description: String,
    amount: f64,
    paid_by: String,             // user_id of the payer
    splits: Vec<Arc<dyn Split>>, // splits does not include paid_by's share
}

impl Expense {
    pub fn new(
        description: String,
        amount: f64,
        paid_by: String,
        splits: Vec<Arc<dyn Split>>,
    ) -> Self {
        let id = generate_uuid("expense");
        Self {
            id,
            description,
            amount,
            paid_by,
            splits,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_paid_by(&self) -> &String {
        &self.paid_by
    }

    pub fn get_splits(&self) -> &Vec<Arc<dyn Split>> {
        &self.splits
    }
}
