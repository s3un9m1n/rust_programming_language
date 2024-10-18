use std::ops::Deref;

/// 자체 스마트포인터 (데이터를 힙에 저장하지는 않음)
struct MyBox<T>(T); //< 하나의 필드를 갖고, 타입만 지정한 튜플 구조체
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/// Deref 트레이트를 구현 - 해당 트레이트가 요구하는 메서드에 대한 구현체 작성
/// self를 빌려와 내부 데이터의 참조자를 반환하도록 함
/// Deref 트레이트는 불변 참조자에 대한 역참조를 오버라이딩, DerefMut은 가변 참조자에 대한 역참조 오버라이딩
impl<T> Deref for MyBox<T> {
    // 연관타입 지정? -> 19장에서 배움
    type Target = T;

    fn deref(&self) -> &Self::Target { // &Self는 &self와 달리 현재 타입(MyBox<T>) 자체를 가리킴
        &self.0 //< 튜플 구조체의 첫 번째 필드를 반환
    }
}

/// 역참조 강제 변환: 어떤 타입의 참조자를 다른 타입의 참조자로 강제 변환(ex. &String -> &str)
/// -> 수많은 명시적 참조, 역참조를 추가하지 않아도 되도록
/// -> 참조자나 스마트포인터 둘 중 어느 경우라도 작동되도록
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // Deref 트레이트를 구현하면 역참조 연산자(*) 동작의 커스터마이징을 가능하게 함
    // 스마트퐁니터를 보통의 참조자처럼 취급될 수 있음(참조자에 작동하도록 작성된 코드가 스마트 포인터에서도 사용 가능)

    // 포인터를 따라가서 값 얻기
    let x = 5;
    let y = &x; // 참조
    assert_eq!(5, x);
    assert_eq!(5, *y); // 역참조
    
    // Box<T>를 참조자처럼 사용
    let x = 5;
    let y = Box::new(x); // 참조가 아닌 x의 복제된 값을 가리킴 (스마트 포인터)
    assert_eq!(5, x);
    assert_eq!(5, *y); // 역참조와 동일한 방식으로 기능

    // MyBox<T>를 참조자처럼 사용
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 역참조 강제 변환
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // MyBox 역참조 -> MyBox 내 값(&String) -> 슬라이스 참조자(&str)
    hello(&(*m)[..]); // 위의 코드와 동작이 동일하나, 역참조 강제 변환으로 간략히 위의 형태로 작성 가능해짐
}
