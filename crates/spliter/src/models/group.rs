use crate::utils::generate_uuid;

#[derive(Clone)]
pub struct Group {
    pub id: String,
    pub members: Vec<String>,  // user_ids of group members
    pub expenses: Vec<String>, // expense_ids associated with the group
}

impl Group {
    pub fn new(members: Vec<String>) -> Self {
        let id = generate_uuid("group");
        Self {
            id,
            members,
            expenses: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_members(&self) -> &Vec<String> {
        &self.members
    }

    pub fn get_expenses(&self) -> &Vec<String> {
        &self.expenses
    }

    pub fn add_expense(&mut self, expense_id: String) {
        self.expenses.push(expense_id);
    }

    pub fn add_member(&mut self, user_id: String) {
        self.members.push(user_id);
    }
}
