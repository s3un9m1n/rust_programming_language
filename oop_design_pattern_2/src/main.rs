//! 상태 패턴(state pattern)의 절충안
//! 상태와 전환을 완전 캡슐화하며, 상태를 다른 타입들로 인코딩

mod blog;
use blog::*;

fn main() {
    // 빈 초안으로 게시물 시작
    let mut post = Post::new();

    // 초안 작성
    post.add_text("I ate a salad for lunch today");
    // 상태와 동작을 타입으로 인코딩해보기
    // - 초안(draft) 게시물이라면 content() 메서드가 빈 문자열을 반환하는 것이 아닌,
    //   content() 메서드가 없더록 해보자
    // assert_eq!("", post.content());

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
