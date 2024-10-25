pub struct Post {
    // 동적 상태 타입 (Option<T>로 감싼)
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            // 상태 시작은 초안(Draft)
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 텍스트 저장
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 테스트 반환
    pub fn content(&self) -> &str {
        // 우선 항상 빈 문자열 반환
        ""
    }

    pub fn request_review(&mut self) {
        // take는 state의 Some 값을 빼내고 state는 None으로 변경
        // 러스트는 값이 없는 필드는 불가능하기 때문에 이런 방식을 사용했고, state의 타입이 Option이었던 이유
        if let Some(s) = self.state.take() {
            // 현재 상태에 대한 리뷰 요청으로 새로운 상태로 변경
            self.state = Some(s.request_review())
        }
    }
}

// 상태 트레이트
trait State {
    // 상태 트레이트에서 리뷰 요청이 왔을 때 상태가 변경되어야 함
    // 인자 self의 타입이 Box<Self> ==> 스마트포인터(Box)를 이용해 소유권을 Box로 이동
    // ==> 즉, 메서드가 self의 소유권을 가져갈 수 있음
    // 반환값이 Box<dyn State>인 이유 역시 새로운 상태를 반환하기 위함
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

// Draft 구조체는 상태 트레이트인 State를 구현
impl State for Draft {
    // Draft -> PendingReview 상태 변경
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    // PendingReview -> PendingReview 상태 유지
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
