//! 내부 가변성(interior mutability)은 어떤 데이터의 불변 참조자가 있을 때라도 데이터를 변경할 수 있게 해주는 러스트의 디자인 패턴
//! 대여 규칙에 의해 허용되지 않지만, unsafe 코드를 사용해 변경과 대여에 대한 일반적인 규칙을 우회함
//! unsafe 코드는 규칙을 지키고 검사하는 컴파일러에게 수동으로 검사를 하겠다고 알리는 방식임
//! 
//! 컴파일러는 대여 규칙 준수가 보장되지 않더라도 런타임 내에 보장할 수 있다면 허용할 수 있음
//! RefCell<T> 타입 역시 내부 가변성 패턴을 따름
//!
//! RefCell<T>는 Rc<T>와 달리 단일 소유권을 가짐
//! Box<T>는 대여 규칙의 불변성은 컴파일 타임에 집행되나,
//! RefCell<T>는 런타임에 집행됨
//! 전자는 규칙을 어기면 컴파일러 에러를 얻지만, 후자는 규칙을 어기면 panic!을 일으킴
//! 전자의 경우가 에러를 일찍 잡을 수 있고 런타임 성능에 영향이 없다는 장점이 있어 좋은 선택(기본 동작)
//! 
//! 대여 규칙
//! - 어떤 경우이든 간에, 하나의 가변 참조자 또는 여러 개의 불변 참조자 중 하나만 가질 수 있음
//! - 참조자는 항상 유효해야 함
//! 
//! RefCell<T>는 Rc<T>와 마찬가지로 싱글 스레드 시나리오 내에서만 사용 가능
//! 
//! 정리
//! - Rc<T>: 동일한 데이터에 대해 복수 소유자를 가능하게 함
//! - Box<T>: 컴파일 타임에 검사되는 불변 혹은 가변 대여를 허용 (Rc<T>: 컴파일 타임 불변, RefCell<T>: 런타임 불변 혹은 가변)
//! - RefCell<T>: 런타임에 검사되는 가변 대여를 허용. RefCell<T>이 불변일 때도 내부의 값 변경 가능
//! 

/// Rc<T>와 RefCell<T> 조합해 가변 데이터 복수 소유자 만들기
/// - Rc<T>는 복수의 불변 소유자 가능
/// - RefCell<T>는 불변 소유자를 런타임에 가변으로 사용 가능
/// - 따라서, RefCell<T>를 소유한 Rc<T>라면 복수의 가변 소유자 가능
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    // Rc::clone을 통해 value의 소유권을 넘기거나 대여하는 것이 아닌,
    // value와 a 모두 value에 대한 소유권을 가질 수 있도록 함
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // b와 c에서 모두 a에 대한 콘스 리스트 소유권을 가질 수 있음
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // 가변 대여 + 역참조
    // borrow_mut()은 RefMut<T> 스마트 포인터를 반환하는데, *를 통해 자동 역참조가 적용되어 내부 값을 접근
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}