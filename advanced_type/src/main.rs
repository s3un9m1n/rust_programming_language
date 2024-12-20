/// 뉴타입 패턴
/// - 값이 혼동되지 않도록 정적으로 강제하는 것과 값의 단위를 표시하는 것들이 포함됨
/// - 어떤 타입의 구현 세부사항을 추상화 가능 (비공개 내부 API와 다른 공개 API 노출 가능)

/// 타입 별칭
/// - 기존 타입 대신 다른 이름으로 타입 사용
/// - 단, 타입 검사 이점을 얻을 수 없음 (`Kilometers`와 `i32`를 혼용해서 사용하더라도 컴파일러는 에러를 표시하지 않음)
/// - 주로 사용되는 용도는 긴 타입의 반복을 줄이는 것
/// - `std::io`의 `Result<T, E>`도 `Result<T>`로 타입 별칭 선언이 되어있음

/// 부정 타입
/// - `!` 특수 타입
/// - 값이 없기 때문에 이론 용어로는 `빈 타입(empty type)`이라고도 함
/// - 함수가 절대 반환하지 않을 때 대신하기 때문에 `부정 타입(never type)`이라고 부르는 것이 선호됨

/// 동적 크기 타입
/// - 러스트는 특정 타입의 값에 할당할 공간의 크기 등 타입에 대한 특정 세부사항을 알아야 함
/// - 동적 크기 타입(Dynamically Sized Type, DST)이란, 크기가 지정되지 않은 타입(unsized type)
/// - 런타임에만 크기를 알 수 있는 값을 사용
/// - `str`이 대표적인 동적 크기 타입(`&str`)
/// - 러스트는 특정 타입의 모든 값에 동일한 크기를 사용해야 하기 때문에, 두 `str`타입은 같은 크기여야 함
/// - 따라서, 러스트에서 동적 크기를 갖는 변수를 생성할 수 없음
/// - DST로 작업하기 위해서는 컴파일 시점에 타입의 크기를 알 수 있도록 해주면 됨
///   --> `Sized` 트레이트 사용
/// - 러스트에서 실제로 제네릭 함수는 `Sized`바운드가 추가됨
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

// 부정 타입
fn bar() -> ! {
    // --생략--
    // `bar()`는 절대로 반환하지 않는다는 뜻
    // 반대는 `발산 함수(diverging functions)`

    let mut guess = String::new();

    loop {
        // 생략

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 여기서 `continue`가 `!`를 반환
            // 러스트에서 `!`를 보고 `guess`의 타입이 `u32(num)`라 결정함
            Err(_) => continue,
        };

        // 사실은 `loop` 또한 `!` 타입을 가질 수 있음
        // 무한 루프일 경우 `!`가 표현식의 값이 되지만,
        // `break`를 포함하면 종료되기 때문에 아님
    }
}

// 동적 크기 타입: 일반 제네릭
fn generic1<T>(t: T) {
    // --생략--
}

// 동적 크기 타입: 원래는 `Sized` 바운드 추가된 것과 같음 (고정 타입)
fn generic2<T: Sized>(t: T) {
    // --생략--
}

// 동적 크기 타입: `T`가 `Sized`가 아닐 수도 있다는 의미로, 컴파일 시점에 제네릭 타입의 크기를 모를 수도 있음
// 인자도 `&T`로 바뀌었음 (`Sized`가 아닐 수 있기 때문에 포인터를 사용해야 함)
fn generic3<T: ?Sized>(t: &T) {
    // --생략--
}