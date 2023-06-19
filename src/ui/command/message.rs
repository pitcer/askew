#[derive(Debug)]
pub struct Message {
    text: String,
    message_type: MessageType,
}

impl Message {
    #[must_use]
    pub fn new(text: String, message_type: MessageType) -> Self {
        Self { text, message_type }
    }

    #[must_use]
    pub fn info(message: String) -> Self {
        Self::new(message, MessageType::Info)
    }

    #[must_use]
    pub fn into_text(self) -> String {
        self.text
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
}

#[derive(Debug)]
pub enum MessageType {
    Info,
    Error,
}
