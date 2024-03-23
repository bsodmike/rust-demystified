#[derive(Debug)]
pub struct TaskManager {
    state: String,
    count: usize,
}

impl TaskManager {
    /// Get task count
    pub fn count(&self) -> &usize {
        &self.count
    }
}

#[derive(Default)]
pub struct TaskManagerBuilder {
    state: String,
    count: usize,
}

impl TaskManagerBuilder {
    /// Creates a new TaskManagerBuilder
    pub fn new() -> Self {
        Self {
            state: "initialized".to_string(),
            count: 0,
        }
    }

    /// Sets the task count
    pub fn count(mut self, value: usize) -> Self {
        self.count = value;
        self
    }

    /// Creates a new TaskManager
    pub fn build(self) -> TaskManager {
        TaskManager {
            state: self.state,
            count: self.count,
        }
    }
}
