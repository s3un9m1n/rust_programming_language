//! Rc의 strong_count는 강한 참조를 만드는 것으로, 0이 된 경우에만 자원이 해제됨
//! 강한 참조는 소유권을 공유할 수 있는 방법
//! 약한 참조를 만드는 것도 가능하며, 약한 참조는 소유권 관계를 표현하지 않음
//! 약한 참조의 개수는 Rc<T> 인스턴스가 제거되는 경우에 영향을 주지 않음
//! 약한 참조가 포함된 순환 참조는 강한 참조 개수를 0으로 만드는 순간 깨지게 되어 순환 참조가 발생하지 않음
//!
//! Rc::downgrade를 통해 Weak<T> 타입의 스마트 포인터를 얻을 수 있음
//! strong_count가 아닌 weak_count를 1 증가시킴
//! Weak<T>가 참조하고 있는 값은 이미 버려졌을 수 있기 때문에 Weak<T>가 가리키는 값으로 어떤 일을 하기 전에는 값이 존재하는지 확인해야 함
//! 이때 upgrade 메서드를 호출하는데, 메서드는 Option<Rc<T>>를 반환함
//! Rc<T>가 버려지지 않은 경우 Some을 얻고 버려진 경우 None을 받음
//! 즉, 유효하지 않은 포인터는 없으며 Option으로 처리됨

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // 자식에서 부모에 대한 참조를 추가
    // Rc<T>를 넣을 경우 branch.children과 leaf.parent 사이 순환 참조 발생 가능
    // 부모는 자식을 소유해야 하지만(부모가 없으면 자식도 없어져야 함), 자식은 부모를 소유해서는 안 됨
    // 따라서 parent는 약한 참조를 사용
    parent: RefCell<Weak<Node>>,
    // 하위 Node를 소유하면서 하위 Node에 대한 소유권 공유 -> Rc<Node>
    // 어떤 노드가 다른 노드의 자식이 되도록 수정이 필요 -> RefCell<Vec<>>
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // leaf에 대한 소유자는 1
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // parent 불변 참조자(borrow)를 가져온 뒤 유효한지 검사(upgarde)
    // -> None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        // leaf에 대한 소유자가 2로 증가
        // branch에서 children을 통해 leaf까지 접근 가능
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // parent 가변 참조자(borrow_mut)에 대해 branch의 약한 참조(downgrade) 추가
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // parent 불변 참조자(borrow)를 가져온 뒤 유효한지 검사(upgarde)
    // -> Some()
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // strong_count와 weak_count 변화 시각화
    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }
        // Weak<T>는 약한 참조로, 자원 해제 시점과 관련이 없어, 순환 참조가 발생하지 않음

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
