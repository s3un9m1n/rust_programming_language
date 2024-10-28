/// 더이상 state를 갖지 않고, 각각의 상태를 구조체로 만듬
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

    // self 인자: self 소유권을 가져와 이전 DraftPost 객체는 자원 해제하고 PendingReviewPost 생성
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
    // self 인자: self 소유권을 가져와 이전 PendingReviewPost 객체는 자원 해제하고 Post 생성
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
