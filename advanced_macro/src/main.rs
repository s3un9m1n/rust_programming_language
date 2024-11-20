//! 매크로
//! 1. 선언적(declarative) 매크로
//! 2. 절차적(procedural) 매크로
//! 2-1. #[derive] 매크로: 구조체와 열거형에 사용되는 derive 속성
//! 2-2. 속성형(attribute-like) 매크로: 커스텀 속성 정의
//! 2-3. 함수형(function-like) 매크로: 함수 호출처럼 보이지만 토큰을 인수로 조작
//!
//! 매크로란, 다른 코드를 작성하는 코드를 작성, 메타프로그래밍(metaprogramming)이라고 함
//! 매크로는 수동으로 작성한 코드보다 더 많은 코드를 생성하기 위해 확장됨
//! 메타프로그래밍은 작성 및 유지 관리해야 하는 코드의 양을 줄이기 용이
//!
//! 매크로에는 함수의 차이
//! - 함수 시그니처는 매개변수의 개수와 타입을 선언하지만, 매크로는 가변적인 매개변수 사용
//!   ex) println!("hello"), println!("hello {}", name)
//! - 함수는 런타임에 호출되고 트레이트는 컴파일 타임에 구현되지만,
//!   매크로는 컴파일러가 코드의 의미를 해석하기 전에 확장되기 때문에,
//!   주어진 타입에 대한 트레이트 구현 가능
//! - 매크로는 러스트 코드를 작성하는 러스트 코드를 작성해야 하기 때문에,
//!   함수보다 정의가 어려우며, 읽고, 이해하고, 유지관리하기 더 어려움
//! - 함수는 어디서나 정의하고 어디서나 호출할 수 있지만,
//!   매크로는 파일에서 호출하기 전에 매크로를 스코프로 가져와야 함
//!
//! 선언적 매크로
//! - 예제 매크로(macros by example), `macro_rules!` 매크로, 아니면 그냥 '매크로'라고도 불림
//! - 가장 널리 사용되는 매크로 형태
//! - 컴파일 타임에 이루어짐
//! - 핵심은 `match` 표현식과 비슷한 무언가를 작성할 수 있다는 것
//!  > 특정 코드와 연관된 패턴과 값을 비교
//!  > 값은 리터럴 러스트 소스 코드, 패턴은 소스코드의 구조와 비교, 매칭되면 전달된 코드를 패턴과 연관된 코드로 대체
//! - `macro_rules!`: 매크로 정의 시 사용
fn main() {
    println!("Hello, world!");
    callDeriveMacro();
}

/// 커스텀 파생(derive) 매크로
/// `HelloMacro` 이름의 트레이트와 `hello_macro` 연관 함수를 정의하는 `hello_macro`라는 이름의 크레이트를 만들자
/// 사용자가 자신의 타입에 대해 `HelloMacro` 트레이트를 구현하도록 하는 대신,
/// 절차적 매크로를 제공해 `#[derive(HelloMacro)]`라고 명시해 `hello_macro`함수의 기본 구현을 가져올 수 있도록 함
/// 아래 예시의 기본 구현에서, `TypeName`은 이 트레이트가 정의된 타입의 이름임
use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn callDeriveMacro() {
    Pancakes::hello_macro();
}

// 속성형 매크로
// - 커스텀 파생 매크로와 비슷하지만 `derive` 속성에 대한 코드를 생성하는 대신 새 속성을 생성
// - 즉, 커스텀 파생 매크로 보다 더 유연함(`derive` 속성은 구조체와 열거형에만 작동)
// - 함수와 같은 다른 아이템에도 적용 가능
// - 커스텀 파생 매크로와 동일하게 작동: `proc-macro` 크레이트 타입으로 크레이트 생성 및 원하는 코드 생성하는 함수 구현
//
// - `#[route]` 속성은 절차적 매크로로써 프레임워크에 의해 정의됨
// #[route(GET, "/")]
// fn index() {...}
//
// 매크로 정의 함수의 시그니처
// 첫 번째 매개변수: 속성 <- `GET, "/"` 부분
// 두 번째 매개변수: 속성이 연결된 아이템의 본문 <- `fn index() {...}`
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {...}

// 함수형 매크로
// - 함수 호출처럼 보이는 매크로를 정의
// - `macro_rules!` 매크로와 유사
// - 함수형 매크로는 함수보다 더 유연 (ex. 임의 개수의 인수 등)
// - `TokenStream` 매개변수를 취하고, 정의는 다른 두 가지 종류의 절차적 매크로와 마찬가지로 러스트 코드를 사용
//
// ex. `sql!` 매크로
// let sql = sql!(SELECT * FROm posts WHERE id=1);
// - 위 매크로는 `macro_rules!` 매크로 보다 훨씬 더 복잡한 처리 가능 (SQL 파싱, 문법 검사)
//
// `sql!` 정의
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {...}