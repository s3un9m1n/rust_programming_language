/// 패턴 사용 케이스
fn main() {
    // 1. match 갈래
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }
    //
    // - exhaustive: 모든 경우의 수를 고려해야 함
    // - catchall 패턴 활용 가능
    // - '_': 어떤 값과도 매칭되지만 값을 변수에 할당하지 않음 (마지막 match 갈래에서 주로 활용)

    // 2. if let 조건 표현식
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();
    //
    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {color}, as the background");
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }
    //
    // - match를 보다 짧게 표현 가능
    // - if let 패턴에 값이 매칭되지 않을 때 실행되는 코드는 else로 지정 가능 (if let / else if / else if let)
    // - match와 같이 섀도잉 변수 도입 가능 (`let Ok(age) = age` 부분에서 Ok 배리언트 내 값을 추출해 섀도잉된 새로운 age 변수 도입)
    // - 컴파일러가 match 표현식처럼 모든 구문이 빠졌는지 검사하지 않는다는 단점

    // 3. while let 조건 루프
    // let mut stack = Vec::new();
    //
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    //
    // while let Some(top) = stack.pop() {
    //     println!("{}", top);
    // }

    // 4. for 루프
    // let v = vec!['a', 'b', 'c'];
    //
    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is at index {}", value, index);
    // }
    //
    // - for 바로 뒤에 있는 것이 패턴 (여기서는 (index, value))

    // 5. let 구문
    // let PATTERN = EXPRESSION;
    //
    // - 많이 사용했던 let 구문 역시 패턴을 사용한 것

    // 6. 함수 매개변수
    // fn foo(x: i32) {}
}
