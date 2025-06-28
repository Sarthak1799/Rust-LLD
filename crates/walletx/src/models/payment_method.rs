pub trait PaymentMethodInterface {
    fn get_name(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Card;

impl PaymentMethodInterface for Card {
    fn get_name(&self) -> String {
        "Card".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BankAccount;
impl PaymentMethodInterface for BankAccount {
    fn get_name(&self) -> String {
        "Bank Account".to_string()
    }
}
