use std::{collections::HashMap, sync::Arc};

use crate::models::{message::Message, subscriber::SubscriberInterface, topic::Topic};
pub struct Service {
    pub name: String,
    pub topics: HashMap<String, Topic>,
}

#[derive(Debug)]
pub enum ServiceError {
    TopicNotFound,
    SubscriberNotFound,
}

impl Service {
    pub fn new(name: String) -> Self {
        Service {
            name,
            topics: HashMap::new(),
        }
    }

    pub fn register_topic(&mut self, topic: Topic) {
        self.topics.insert(topic.name.clone(), topic);
    }
    pub fn get_topic(&self, topic_name: &str) -> Option<&Topic> {
        self.topics.get(topic_name)
    }

    pub fn add_subscriber_to_topic(
        &mut self,
        topic: String,
        subscriber: Arc<dyn SubscriberInterface>,
    ) -> Result<(), ServiceError> {
        let topic = self
            .topics
            .get_mut(&topic)
            .ok_or(ServiceError::TopicNotFound)?;
        topic.add_subscriber(subscriber);

        Ok(())
    }

    pub fn remove_subscriber_from_topic(
        &mut self,
        topic: String,
        subscriber_id: String,
    ) -> Result<(), ServiceError> {
        let topic = self
            .topics
            .get_mut(&topic)
            .ok_or(ServiceError::TopicNotFound)?;
        topic.remove_subscriber(subscriber_id.as_str());
        Ok(())
    }

    pub fn publish_message_to_topic(
        &self,
        topic: String,
        message: Message,
    ) -> Result<(), ServiceError> {
        let topic = self.topics.get(&topic).ok_or(ServiceError::TopicNotFound)?;
        topic.publish_message(&message);
        Ok(())
    }
}
