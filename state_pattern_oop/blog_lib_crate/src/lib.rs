pub struct Post {
    state: Option<Box<dyn State>>,
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
        if let Some(ref state) = self.state {
            state.add_text(&mut self.content, text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
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
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn add_text(&self, _content: &mut String, _text: &str) {}
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingSecondReview {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct PendingSecondReview {}

impl State for PendingSecondReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
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

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


// implementing the state pattern using enum
/*
pub struct Post {
    state: PostState,
    content: String,
}

enum PostState {
    Draft,
    PendingReview,
    PendingSecondReview,
    Published,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let PostState::Draft = self.state {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        match self.state {
            PostState::Published => &self.content,
            _ => "",
        }
    }

    pub fn request_review(&mut self) {
        if let PostState::Draft = self.state {
            self.state = PostState::PendingReview;
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            PostState::PendingReview => {
                self.state = PostState::PendingSecondReview;
            }
            PostState::PendingSecondReview => {
                self.state = PostState::Published;
            }
            _ => {}
        }
    }

    pub fn reject(&mut self) {
        match self.state {
            PostState::PendingReview | PostState::PendingSecondReview => {
                self.state = PostState::Draft;
            }
            _ => {}
        }
    }
}
*/