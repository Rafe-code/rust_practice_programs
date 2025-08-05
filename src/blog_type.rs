//! Want a system which creates a blog post in 3 stages
//! 1. create this as a draft
//! 2. request review
//! 3. create blog post
//! (same functionality as blog_oop)
//!
//! Should be able to add text to the draft version,
//! request and approve review, and should not produce
//! any requested content until approved, so that no accidental publishing
//!
//! Key: Implement this using Rust like Type practices
//! i.e., Draft, Pending, Approved are different types and changing between them consumes the old

struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}
struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn request_review(self) -> Pending {
        Pending {
            content: self.content,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

struct Pending {
    content: String,
}

impl Pending {
    pub fn approve(self) -> Approved {
        Approved {
            content: self.content,
        }
    }
}

struct Approved {
    content: String,
}

impl Approved {
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blog() {
        let mut post = Post::new();
        let blog_text = "This is my blog post";
        post.add_text(blog_text);
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(post.content(), blog_text)
    }
}
