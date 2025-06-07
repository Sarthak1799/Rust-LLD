pub struct Message {
    pub id: String,
    pub data: String,
}

impl Message {
    pub fn new(id: String, data: String) -> Self {
        Message { id, data }
    }
}
