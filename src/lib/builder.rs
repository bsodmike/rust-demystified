#[derive(Debug)]
pub(crate) struct TaskManager {
    state: String,
    count: usize,
}

impl TaskManager {
    /// Get task count
    pub(crate) fn count(&self) -> &usize {
        &self.count
    }
}

#[derive(Default)]
pub(crate) struct TaskManagerBuilder {
    state: String,
    count: usize,
}

impl TaskManagerBuilder {
    /// Creates a new TaskManagerBuilder
    pub(crate) fn new() -> Self {
        Self {
            state: "initialized".to_string(),
            count: 0,
        }
    }

    /// Sets the task count
    pub(crate) fn count(mut self, value: usize) -> Self {
        self.count = value;
        self
    }

    /// Creates a new TaskManager
    pub(crate) fn build(self) -> TaskManager {
        TaskManager {
            state: self.state,
            count: self.count,
        }
    }
}
