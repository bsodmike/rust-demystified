use std::cell::UnsafeCell;

pub(crate) struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub(crate) fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub(crate) fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self (because !Sync)
        // SAFETY: we're not invalidating any references as we are not sharing any.
        unsafe { *self.value.get() = value };
    }

    pub(crate) fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is concurrently mutating this value, since only this thread
        // can mutate (because !Sync) and it is executing this function
        unsafe { *self.value.get() }
    }
}

/// Contrived example storing a String as Vec<u8>
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

    pub(crate) fn content_from_bytes(&self) -> Option<String> {
        Some(String::from_utf8(self.bytes.clone()).ok()?)
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
