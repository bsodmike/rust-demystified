pub(crate) struct Message {
    content: String,
    bytes: Vec<u8>,
}

impl Message {
    pub(crate) fn update(mut self, content: &str) -> Self {
        let bytes: Vec<u8> = content.to_string().as_bytes().to_vec();
        self.bytes = bytes;

        self
    }

    pub(crate) fn content(&self) -> &String {
        &self.content
    }

    pub(crate) fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

pub(crate) struct MessageBuilder {
    content: String,
}

impl MessageBuilder {
    pub(crate) fn new() -> Self {
        Self {
            content: String::default(),
        }
    }

    pub(crate) fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub(crate) fn build(&self) -> Message {
        Message {
            content: self.content.to_string(),
            bytes: vec![0],
        }
    }
}
