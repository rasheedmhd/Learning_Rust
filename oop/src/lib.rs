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
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        //
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
struct InReview {}
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(InReview {})
    }
}

impl State for InReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Self {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
//impl State for Published {}
