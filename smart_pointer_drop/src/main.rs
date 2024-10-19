struct CustomSmartPointer {
    data: String,
}

// Drop: 소멸자 느낌
// 스코프를 벗어날 때 drop 함수가 호출되도록 하는 트레이트
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // d의 drop이 c의 drop 보다 먼저 호출됨 (stack)
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // 중간에 메모리 해제가 필요한 경우(ex. 락 관리)
    let a = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    
    // drop 함수는 스코프가 벗어날 때 자동으로 호출되며,
    // 중간에 수동으로 호출할 수 없음 (double free 방지)
    // a.drop();

    // std::mem::drop 함수는 어던 값에 대한 메모리 정리를 강제로 일찍 할 수 있음
    drop(a);
    // 소유권 시스템을 이용해 메모리가 한 번만 해제될 수 있도록 보장함
    // a의 소유권을 시스템으로 넘겨주게 됨
    // drop(a);

    println!("CustomSmartPointer dropped before the end of main.");
}
