//! 상태 패턴(state pattern)
//! 어떤 값이 내부적으로 가질 수 있는 상태 집합을 정의함
//!
//! 러스트는 객체와 상속이 아닌 구조체와 트레이트를 사용해 상태 패턴을 보여줌
//! 외부에서는 동일하게 호출하지만, 내부적인 상태에 따라 동작이 달라지는 방식
//!
//! 전통적인 객체 지향 방식으로 상태 패턴을 구현한 뒤, 러스트스러운 접근법을 적용할 것

mod blog;
use blog::Post;

fn main() {
    // 빈 초안으로 게시물 시작
    let mut post = Post::new();

    // 초안 작성
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // 게시물 승인 요청
    post.request_review();
    assert_eq!("", post.content());

    // 승인되면 게시
    post.approve();
    // 게시 되어야 content() 응답값이 나옴
    assert_eq!("I ate a salad for lunch today", post.content());
}
