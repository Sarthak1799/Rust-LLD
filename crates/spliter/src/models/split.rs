pub trait Split {
    fn calculate_shares(&self) -> Vec<(String, f64)>; // Returns a vector of (user_id, share_amount)
}

#[derive(Debug, Clone)]
pub struct EqualSplit {
    pub user_ids: Vec<String>,
    pub total_amount: f64,
}

impl EqualSplit {
    pub fn new(user_ids: Vec<String>, total_amount: f64) -> Self {
        Self {
            user_ids,
            total_amount,
        }
    }
}

impl Split for EqualSplit {
    fn calculate_shares(&self) -> Vec<(String, f64)> {
        let share = self.total_amount / self.user_ids.len() as f64;
        self.user_ids
            .iter()
            .map(|user_id| (user_id.clone(), share))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct PercentageSplit {
    pub user_percentages: Vec<(String, f64)>, // (user_id, percentage)
    pub total_amount: f64,
}

impl PercentageSplit {
    pub fn new(user_percentages: Vec<(String, f64)>, total_amount: f64) -> Self {
        Self {
            user_percentages,
            total_amount,
        }
    }
}

impl Split for PercentageSplit {
    fn calculate_shares(&self) -> Vec<(String, f64)> {
        self.user_percentages
            .iter()
            .map(|(user_id, percentage)| {
                let share = (percentage / 100.0) * self.total_amount;
                (user_id.clone(), share)
            })
            .collect()
    }
}
