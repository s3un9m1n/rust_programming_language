/// 함수 포인터
/// - 새로운 클로저를 정의하는 대신 이미 정의한 함수를 전달
/// - 함수는 fn 타입이며, Fn 클로저 트레이트와 혼동 주의
/// - fn 타입: 함수 포인터(function pointer)
/// - 함수 포인터는 세 가지 클로저 트레이트(Fn, FnMut, FnOnce)를 모두 구현하므로,
///   클로저를 기대하는 함수에 대한 인수로 함수 포인터를 언제나 전달 가능
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

/// 클로저 반환
/// - 클로저는 트레이트이기 때문에 클로저를 직접 반환할 수 없음
///  > 트레이트를 반환해야하는 경우, 대신 트레이트를 구현하는 구체 타입을 반환에 사용
///  > 하지만, 클로저에는 반환할 수 있는 구체타입이 없음

// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     // 반환 불가
//     // 클로저를 저장하기 위해 얼마나 많은 공간이 필요한지 알 수 없기 때문
//     // 트레이트 객체 사용으로 우회
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // 클로저 사용
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    // 함수 이름 사용
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    // 열거형 배리언트 이름도 초기화 함수가 될 수 있음
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}