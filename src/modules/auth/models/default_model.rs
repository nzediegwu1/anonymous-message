#[derive(serde::Serialize)]
pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new(self) -> Self {
        Message {
            message: self.message,
        }
    }
}
