use std::sync::RwLock;

use crate::limiter_interface::{LimiterError, LimiterInterface};

pub struct Limiter {
    pub limiter: RwLock<Box<dyn LimiterInterface>>,
}

impl Limiter {
    pub fn new(limiter: Box<dyn LimiterInterface>) -> Self {
        Limiter {
            limiter: RwLock::new(limiter),
        }
    }

    pub fn set_limiter(&mut self, limiter: Box<dyn LimiterInterface>) {
        self.limiter = RwLock::new(limiter);
    }

    pub async fn process(&self, req: Request) -> Result<(), LimiterError> {
        let mut writer = self
            .limiter
            .write()
            .map_err(|_| LimiterError::GenericError("Failed to acquire write lock".to_string()))?;
        writer.process(&req.id)?;

        println!("Processed input: {}", &req.data);
        Ok(())
    }

    pub fn reset(&self) -> Result<(), LimiterError> {
        let mut writer = self
            .limiter
            .write()
            .map_err(|_| LimiterError::GenericError("Failed to acquire write lock".to_string()))?;
        writer.reset();

        println!("Limiter has been reset.");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub id: String,
    pub data: String,
}

impl Request {
    pub fn new(id: String, data: String) -> Self {
        Request { id, data }
    }
}
