/// Want a system which creates a blog post in 3 stages
/// 1. create this as a draft
/// 2. request review
/// 3. create blog post
///
/// Should be able to add text to the draft version,
/// request and approve review, and should not produce
/// any requested content until approved, so that no accidental publishing
/// Implement this using standard OOP practices here, and Rust Type like practices elsewhere
use std::any::Any;
pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if let Some(ref state) = self.state {
            if state.as_any().is::<Draft>() {
                self.content.push_str(text)
            }
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn as_any(&self) -> &dyn Any;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending { num_approvals: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Pending {
    num_approvals: i32,
}

impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        match self.num_approvals {
            0 => {
                self.num_approvals += 1;
                return self;
            }
            1 => return Box::new(Published {}),
            _ => panic!(
                "Pending state existed with {} approvals. Should only be 0 or 1",
                self.num_approvals
            ),
        };
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_no_content() {
        let mut post = Post::new();
        post.add_text("This is a blog post.");
        assert_eq!(post.content(), "");

        post.request_review();
        assert_eq!(post.content(), "");

        post.reject();
        assert_eq!(post.content(), "");

        post.request_review();
        post.approve();
        assert_eq!(post.content(), "");
        post.approve();
        post.add_text("This text should not appear.");
        assert_eq!(post.content(), "This is a blog post.");
    }
}
