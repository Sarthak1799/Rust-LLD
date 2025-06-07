use crate::models::{message::Message, subscriber::SubscriberInterface};
use std::sync::Arc;
pub struct Topic {
    pub name: String,
    pub subscribers: Vec<Arc<dyn SubscriberInterface>>,
}
impl Topic {
    pub fn new(name: String) -> Self {
        Topic {
            name,
            subscribers: Vec::new(),
        }
    }

    pub fn add_subscriber(&mut self, subscriber: Arc<dyn SubscriberInterface>) {
        self.subscribers.push(subscriber);
    }

    pub fn remove_subscriber(&mut self, subscriber_id: &str) {
        self.subscribers
            .retain(|subscriber| subscriber.get_id() != subscriber_id);
    }

    pub fn publish_message(&self, message: &Message) {
        println!(
            "Publishing message with id {} to topic: {}",
            message.id, self.name
        );
        for subscriber in &self.subscribers {
            subscriber.do_task(message);
        }
    }
}
