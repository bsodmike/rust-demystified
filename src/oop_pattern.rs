pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn review(&self) -> &String {
        &self.content
    }

    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self, changes: &str) -> RequestChangesPost {
        RequestChangesPost {
            content: self.content,
            changes: changes.to_string(),
        }
    }
}

pub struct RequestChangesPost {
    content: String,
    changes: String,
}

impl RequestChangesPost {
    pub fn get_feedback(&self) -> String {
        format!("Make changes to '{}' as {}", &self.content, &self.changes)
    }

    pub fn replace_text(&mut self, text: &str) -> PendingReviewPost {
        PendingReviewPost {
            content: text.to_string(),
        }
    }
}
