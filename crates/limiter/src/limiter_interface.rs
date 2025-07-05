use std::collections::{HashMap, VecDeque};

pub trait LimiterInterface: Send + Sync {
    fn process(&mut self, input: &str) -> Result<(), LimiterError>;
    fn reset(&mut self);
}

#[derive(Debug, Clone)]
pub enum LimiterError {
    InvalidInput(String),
    LimitExceeded(String),
    GenericError(String),
}

#[derive(Debug, Clone)]
pub struct SlidingWindowLimiter {
    window_size: usize,
    current_size: usize,
    queue: VecDeque<String>,
}

impl SlidingWindowLimiter {
    pub fn new(window_size: usize) -> Self {
        SlidingWindowLimiter {
            window_size,
            current_size: 0,
            queue: VecDeque::new(),
        }
    }
}

impl LimiterInterface for SlidingWindowLimiter {
    fn process(&mut self, input: &str) -> Result<(), LimiterError> {
        let exists = self.queue.contains(&input.to_string());
        if exists {
            return Err(LimiterError::InvalidInput(format!(
                "Input '{}' already exists in the sliding window.",
                input
            )));
        }
        if self.current_size < self.window_size {
            self.queue.push_back(input.to_string());
            self.current_size += 1;
        } else {
            self.queue.pop_front();
            self.queue.push_back(input.to_string());
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.queue.clear();
        self.current_size = 0;
    }
}

#[derive(Debug, Clone)]
pub struct CounterLimiter {
    limit: usize,
    counter: HashMap<String, usize>,
}

impl CounterLimiter {
    pub fn new(limit: usize) -> Self {
        CounterLimiter {
            limit,
            counter: HashMap::new(),
        }
    }
}

impl LimiterInterface for CounterLimiter {
    fn process(&mut self, input: &str) -> Result<(), LimiterError> {
        let count = self.counter.entry(input.to_string()).or_insert(0);
        if *count >= self.limit {
            return Err(LimiterError::LimitExceeded(format!(
                "Input '{}' has exceeded the limit of {}.",
                input, self.limit
            )));
        }
        *count += 1;
        Ok(())
    }

    fn reset(&mut self) {
        self.counter.clear();
    }
}
