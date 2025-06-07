use crate::models::message::Message;

pub trait SubscriberInterface {
    fn do_task(&self, message: &Message);

    fn get_id(&self) -> String;
}

pub struct Consumer {
    pub id: String,
    pub name: String,
}
impl Consumer {
    pub fn new(id: String, name: String) -> Self {
        Consumer { id, name }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl SubscriberInterface for Consumer {
    fn do_task(&self, message: &Message) {
        println!("Consumer {} received message: {}", self.name, message.data);
    }

    fn get_id(&self) -> String {
        self.get_id()
    }
}
