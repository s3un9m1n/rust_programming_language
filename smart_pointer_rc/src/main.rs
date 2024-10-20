// 참조 카운팅은 카운트가 0이 될 시 자원을 해제하는 개념
// 즉, Rc<T> 타입은 여러 부분에서 읽을 데이터를 힙에 할당하며,
// 컴파일 때는 어디에서 그 데이터를 마지막에 이용할지 알 수 없는 경우에 사용
// 마지막에 사용하는 부분을 안다면 해당 부분을 데이터의 소유자로 만들면 됨
// 또한, Rc<T>는 싱글스레드 시나리오용으로, 멀티스레드 참조 카운팅은 동시성을 고려해야 함

use crate::List::{Cons, Nil};
use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // b와 c의 콘스 리스트는 a 콘스 리스트를 공유하고자 함
    // 하지만 b를 생성할 때 b 안으로 a의 소유권이 이동함
    // 따라서 c는 a를 사용할 수 없음
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // 방안 1. Cons 정의를 변경해 참조자를 대신 드록 있도록 함 (라잉프타임 매개변수 필요)
    // 방안 2. Box 대신 Rc를 이용함

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rc::clone() 대신 a.clone()도 호출가능하지만 관례
    // deep copy가 아니며 오직 참조 카운트만 증가
    // a.clone()은 deep copy일 때 사용하기 때문에 코드를 읽기 편하게 함
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // Rc<T> 값이 스코프를 벗어나면 Drop 트레이트 구현체가 자동으로 참조 카운트를 감소
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // Rc<T>는 불변 참조자로 읽기 전용이기 때문에 데이터 공유가 편함
}
