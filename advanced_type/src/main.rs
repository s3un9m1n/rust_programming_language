/// 뉴타입 패턴
/// - 값이 혼동되지 않도록 정적으로 강제하는 것과 값의 단위를 표시하는 것들이 포함됨
/// - 어떤 타입의 구현 세부사항을 추상화 가능 (비공개 내부 API와 다른 공개 API 노출 가능)

/// 타입 별칭
/// - 기존 타입 대신 다른 이름으로 타입 사용
/// - 단, 타입 검사 이점을 얻을 수 없음 (`Kilometers`와 `i32`를 혼용해서 사용하더라도 컴파일러는 에러를 표시하지 않음)
/// - 주로 사용되는 용도는 긴 타입의 반복을 줄이는 것
/// - `std::io`의 `Result<T, E>`도 `Result<T>`로 타입 별칭 선언이 되어있음
fn main() {
    // 타입 별칭 #1
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // 타입 별칭 #2-1 -> #2-2
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type1(f: Box<dyn Fn() + Send + 'static>) {}

    // fn returns_long_type1() -> Box<dyn Fn() + Send + 'static> {}

    // 타입 별칭 #2-2
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type2(f: Thunk) {}

    // fn returns_long_type2() -> Thunk {}
}