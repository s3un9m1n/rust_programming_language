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