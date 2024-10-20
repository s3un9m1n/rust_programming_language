//! 러스트의 안정성 보장은 메모리 누수가 발생하기 어렵게 하지만,
//! 불가능 하게 하는 것은 아님
//! 직전에 했던 Rc<T> 또는 RefCell<T>가 그러한 예시
//! 
//! 이를 이용해 아이템간 순환 참조로 참조자를 만드는 것도 가능
//! 순환 참조는 메모리 누수를 발생
//! 순환 고리 안의 아이템의 참조 카운트는 절대 0이 되지 않기 때문

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

/// Rc<RefCell<List>> vs RefCell<Rc<List>>
/// 전자는 List 안의 i32 값을 변경 가능
/// 후자는 Cons 배리언트가 가리키는 List 값을 변경 가능
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            // Cons 배리언트를 갖고 있다면 두 번째 아이템에 접근하도록 함
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // a의 tail에 Nil이 아닌 b를 가리키도록 변경
    // -> 순환 생성
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}