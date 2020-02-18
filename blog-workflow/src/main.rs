pub struct Post {
    pub state: Option<Box<dyn State>>, // public just for debugging
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content = self.state.as_ref().unwrap().add_text(&self, text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn name(&self) -> &'static str;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn add_text(&self, post: &Post, _new_text: &str) -> String {
        println!("Sorry, adding text not allowed in the current stage.");
        String::from(post.content())
    }
}

struct Draft {}
struct PendingReview {
    approve_count: i32,
}
struct Approved {}

impl State for Draft {
    fn name(&self) -> &'static str {
        "Draft"
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text(&self, post: &Post, new_text: &str) -> String {
        let mut s = String::from(post.content());
        s.push_str(new_text);
        s
    }
}

impl State for PendingReview {
    fn name(&self) -> &'static str {
        "PendingReview"
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approve_count < 1 {
            Box::new (PendingReview { approve_count: self.approve_count + 1 })
        } else {
            Box::new (Approved {})
        }
    }
}

impl State for Approved {
    fn name(&self) -> &'static str {
        "Approved"
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

use std::any::TypeId;
fn type_id<T: 'static + ?Sized>(_: &T) -> TypeId {
    TypeId::of::<T>()
}

fn main() {
    let mut post = Post::new();
    post.add_text("foobar");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.reject(); // goes back to draft
    // assert_eq!(type_id(&post.state), TypeId::of::<Draft>()); // false - ref
    assert_eq!(post.state.as_ref().unwrap().name(), "Draft");
    post.request_review(); // pending
    assert_eq!(post.state.as_ref().unwrap().name(), "PendingReview");
    post.approve();
    assert_eq!("", post.content()); // content changes when approved...
    assert_eq!(post.state.as_ref().unwrap().name(), "PendingReview"); // still pending, needs 2 approves
    post.approve();
    assert_eq!(post.state.as_ref().unwrap().name(), "Approved"); // still pending, needs 2 approves
    assert_eq!("foobar", post.content());
    post.add_text("baz"); // won't change text value
    assert_eq!("foobar", post.content());
}
