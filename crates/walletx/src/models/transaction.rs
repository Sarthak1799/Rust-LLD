use crate::utils::generate_random_id;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from_account: String,
    pub to_account: String,
    pub amount: f64,
    pub payment_method: String,
}

impl Transaction {
    pub fn new(
        from_account: String,
        to_account: String,
        amount: f64,
        payment_method: String,
    ) -> Self {
        let id = generate_random_id("transaction");
        Transaction {
            id,
            from_account,
            to_account,
            amount,
            payment_method,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_from_account(&self) -> &str {
        &self.from_account
    }

    pub fn get_to_account(&self) -> &str {
        &self.to_account
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_payment_method(&self) -> &str {
        &self.payment_method
    }
}
