use crate::utils::generate_random_id;

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub balance: f64,
}

impl Account {
    pub fn new(user_id: String, balance: f64) -> Self {
        let id = generate_random_id("account");
        Account {
            id,
            user_id,
            balance,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }
}
