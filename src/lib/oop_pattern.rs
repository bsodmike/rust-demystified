pub(crate) struct Post {
    content: String,
}

pub(crate) struct DraftPost {
    content: String,
}

impl Post {
    pub(crate) fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub(crate) fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub(crate) fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub(crate) fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub(crate) struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub(crate) fn review(&self) -> &String {
        &self.content
    }

    pub(crate) fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub(crate) fn reject(self, changes: &str) -> RequestChangesPost {
        RequestChangesPost {
            content: self.content,
            changes: changes.to_string(),
        }
    }
}

pub(crate) struct RequestChangesPost {
    content: String,
    changes: String,
}

impl RequestChangesPost {
    pub(crate) fn get_feedback(&self) -> String {
        format!("Make changes to '{}' as {}", &self.content, &self.changes)
    }

    pub(crate) fn replace_text(&mut self, text: &str) -> PendingReviewPost {
        PendingReviewPost {
            content: text.to_string(),
        }
    }
}
