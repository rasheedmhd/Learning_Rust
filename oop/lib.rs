pub struct PostContent {
    text: String,
    image_url: String,
    tags: Vec<String>,
    video_url: Option<String>,
}

pub struct Post {
    state: Option<Box<dyn State>>,
    //    content: PostContent,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_content(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.approve());
        }
    }
}

trait State {
    // only valid when called on a type held in a Box
    // restricting the types that the method call be called on
    // further reducing type errors
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
struct InReview {}
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(InReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for InReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
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
}

/// GOING FOR MORE TYPE SAFETY
pub struct SafePost {
    content: String,
}

pub struct SafeDraftPost {
    content: String,
}

pub struct SafeInReviewPost {
    content: String,
}

impl SafePost {
    pub fn new() -> SafeDraftPost {
        SafeDraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl SafeDraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> SafeInReviewPost {
        SafeInReviewPost {
            content: self.content,
        }
    }
}

impl SafeInReviewPost {
    pub fn approve(self) -> SafePost {
        SafePost {
            content: self.content,
        }
    }
}
