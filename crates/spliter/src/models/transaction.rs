use crate::utils::generate_uuid;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from_user: String,
    pub to_user: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(from_user: String, to_user: String, amount: f64) -> Self {
        let id = generate_uuid("txn");
        Self {
            id,
            from_user,
            to_user,
            amount,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_from_user(&self) -> &String {
        &self.from_user
    }

    pub fn get_to_user(&self) -> &String {
        &self.to_user
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}
