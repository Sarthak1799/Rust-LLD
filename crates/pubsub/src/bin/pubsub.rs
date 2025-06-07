use std::sync::Arc;

use pubsub::models::{message::Message, subscriber::Consumer, topic::Topic};
use pubsub::service::Service;

fn main() {
    let mut service = Service::new("My PubSub Service".to_string());

    // Create a topic
    let topic_name = "my_topic".to_string();
    let topic = Topic::new(topic_name.clone());
    service.register_topic(topic);

    // Create a subscriber
    let consumer = Consumer::new("1".to_string(), "Consumer 1".to_string());
    let another_consumer = Consumer::new("2".to_string(), "Consumer 2".to_string());

    // Add the subscriber to the topic
    service
        .add_subscriber_to_topic(topic_name.clone(), Arc::new(consumer))
        .unwrap();
    service
        .add_subscriber_to_topic(topic_name.clone(), Arc::new(another_consumer))
        .unwrap();

    // Publish a message to the topic
    let message = Message {
        id: "1".to_string(),
        data: "Hello, World!".to_string(),
    };
    service
        .publish_message_to_topic(topic_name.clone(), message)
        .unwrap();
}
