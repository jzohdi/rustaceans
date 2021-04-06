//use oop::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("Learning rust is hard");
    post.add_text("\nIt's really fun though!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_empty() {
        let mut post = Post::new();
        post.add_text("Hello world");
        post.add_text("\nThis is OOP in rust");
        assert_eq!("", post.content());
    }

    #[test]
    pub fn test_approve() {
        
        let mut post = Post::new();
        post.add_text("Learning OOP in rust");
        
        let post = post.request_review();
        
        assert_eq!("", post.content());
        
        let post = post.approve();
        assert_eq!("Learning OOP in rust", post.content());
    }
}

// this is an example of OOP in rust using transfer of types
struct Post {
    content: String,
}

impl Post {
    pub fn new() -> Draft {
        Draft {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
       &self.content 
    }
}


struct Draft {
    content: String,
}

impl Draft {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content
        }
    }
    pub fn content(&self) -> &'static str {
        ""
    }
}

struct PendingReview {
    content: String,
}

impl PendingReview {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
    pub fn content(&self) -> &'static str {
        ""
    }
}

