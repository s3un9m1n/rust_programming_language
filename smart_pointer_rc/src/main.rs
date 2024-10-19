// 참조 카운팅은 카운트가 0이 될 시 자원을 해제하는 개념
// 즉, Rc<T> 타입은 여러 부분에서 읽을 데이터를 힙에 할당하며,
// 컴파일 때는 어디에서 그 데이터를 마지막에 이용할지 알 수 없는 경우에 사용
// 마지막에 사용하는 부분을 안다면 해당 부분을 데이터의 소유자로 만들면 됨
// 또한, Rc<T>는 싱글스레드 시나리오용으로, 멀티스레드 참조 카운팅은 동시성을 고려해야 함

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
