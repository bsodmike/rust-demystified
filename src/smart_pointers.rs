use std::cell::UnsafeCell;

// Further coverage of Cell, RefCell and Rc (https://youtu.be/8O0Nt9qY_vo):
// https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T>
// https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html#impl-Sync

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self (because !Sync)
        // SAFETY: we're not invalidating any references as we are not sharing any.
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is concurrently mutating this value, since only this thread
        // can mutate (because !Sync) and it is executing this function
        unsafe { *self.value.get() }
    }
}

/// Contrived example storing a String as Vec<u8>
pub struct Message {
    content: String,
    bytes: Vec<u8>,
}

impl Message {
    pub fn update(mut self, content: &str) -> Self {
        let bytes: Vec<u8> = content.to_string().as_bytes().to_vec();
        self.bytes = bytes;

        self
    }

    pub fn content_from_bytes(&self) -> Option<String> {
        Some(String::from_utf8(self.bytes.clone()).ok()?)
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}

pub struct MessageBuilder {
    content: String,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            content: String::default(),
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn build(&self) -> Message {
        Message {
            content: self.content.to_string(),
            bytes: vec![0],
        }
    }
}
