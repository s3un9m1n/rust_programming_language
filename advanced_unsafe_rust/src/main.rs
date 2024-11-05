//! 지금까지는 컴파일 타임에 메모리 안전 보장이 이뤄졌음
//! 러스트에서는 메모리 안전 보장을 하지 않을 수도 있음 (unsafe Rust)
//!
//! 러스트 컴파일러는
//! 일부 유효하지 않은 프로그램을 허용하는 것보다
//! 일부 유효한 프로그램을 거부하는 것이 더 낫다는 입장
//!
//! 러스트 컴파일러는 확신이 없다면 거부할 것임
//! unsafe Rust는 순전히 사용자의 책임하에 사용되는 것
//!
//! - `unsafe` 키워드로 unsafe Rust 전환 가능
//! - 다음과 같은 기능 포함
//!     - 원시 포인터(raw pointer) 역참조
//!     - 안전하지 않은 함수 또는 메서드 호출
//!     - 가변 정적 변수에 접근 및 수정
//!     - 안전하지 않은 트레이트 구현
//!     - `union`의 필드 접근
//! - 따라서, `unsafe`일지라도 어느정도의 안전성 확보 가능
//! - 또한, `unsafe`일지라도 내부가 반드시 위험하거나 메모리 안전에 문제가 있는 것도 아님
//! - 제공 기능과 관련된 위험은 항상 `unsafe` 블록 안에 있게 됨
//! - `unsafe` 블록은 최대한 작게 유지해야 함
//! - 안전하지 않은 코드는 최대한 분리 필요하며, 안전하지 않은 코드를 안전한 추상화 안에 넣고 안전한 API를 제공하는 것이 바람직
//! - 안전하지 않은 코드를 안전 추상화로 감싸면 `unsafe` 코드가 구현된 기능을 사용하려는 모든 곳에 `unsafe` 라고 쓰는 것을 방지할 수 있음
//!     (추상화 사용 시 안전함)
fn main() {
    // 1. 원시 포인터(raw pointer) 역참조
    // - 원시 포인터: 참조와 유사하며, 불변과 가변이 각각 `*const T`와 `*mut T`로 작성됨
    // - '*': 역참조 연산자가 아닌 타입 이름의 일부
    // - 불변 원시 포인터: 역참조된 후에 직접 할당 불가
    // - 특징
    //  > 대여 규칙을 무시할 수 있음 -> 같은 위치에 대해 불변/가변 포인터를 동시에 여러개 가질 수 있음
    //  > 유효한 메모리를 가리키는 것을 보장하지 못함 + null이 가능
    //  > 자동 메모리 정리를 구현하지 않음
    // - 즉, 러스트에서의 안전성을 포기하고, 다른 언어 또는 하드웨어와 인터페이싱할 수 있는 기능이나 더 나은 성능 확보 가능
    // - 또한, 안전한 코드(`unsafe` 블럭이 아닌 곳)에서 생성 가능하며, 원시 포인터 역참조만 불가능함
    let mut num = 5;

    // 불변 참조자를 원시 포인터 타입으로 캐스팅
    let r1 = &num as *const i32;
    // 가변 참조자를 원시 포인터 타입으로 캐스팅
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 유효성을 확신할 수 없는 원시 포인터
    // 임의의 메모리 주소에 접근
    // - 참조는 그 대상이 값이 항상 있음을 보장함
    // - 아래 address에 할당된 값(0x012345)은 숫자일뿐, 러스트 입장에서 역참조시 유효한 값인지를 보장하지 않음
    // - 즉, address가 더이상 할당된 메모리를 가리키지 않거나 접근 권한이 없는 영역도 가리킬 수 있음
    // 위의 num의 참조자에 대한 원시 포인터는 num이 위차한 곳의 메모리 주소를 반환하지만,
    // 아래 address의 원시 포인터는 address가 위차한 곳의 메모리 주소가 아닌, 0x012345 주소값 자체를 가리킴
    let address = 0x012345usize;
    let r = address as *const i32;

    // 원시 포인터는 가변 포인터와 불변 포인터를 생성해 잠재적인 데이터 경합을 일으킬 수도 있음
    // 그럼에도 C 코드와 상호작용, 대여 검사기가 이해 못하는 안전한 추상화 구축과 같은 경우 필요

    // unsafe 함수는 unsafe 블록 안에서만 호출 가능
    // unsafe 블록 내에서 호출한다는 것은 해당 함수의 문서를 읽고 계약서를 준수할 책임이 있음을 의미
    unsafe {
        dangerous();
    }

    // 함수 안에 안전하지 않은 코드가 포함되어있다고 해서 함수 전체가 unsafe로 표시할 필요는 없음
    // 안전하지 않은 코드를 안전한 함수로 감싸는 것이 일반적인 추상화
    // ex. 안전하지 않은 코드가 약간 필요한 표준 라이브러리의 split_at_mut()
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    // extern 블록 내 선언된 함수는 unsafe
    // 다른 언어가 러스트의 규착과 보증을 적용하지 않고, 러스트는 이를 확인할 수도 없어,
    // 프로그래머가 안전을 보장할 책임이 있음
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// unsafe 함수
unsafe fn dangerous() {}

use std::slice;

// 예시를 위해 직접 함수로 구현
// unsafe 함수가 아니며 안전한 코드에서도 호출할 수 있음
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // i32에 대한 가변 슬라이스의 길이
    let len = values.len();
    // 슬라이스의 원시 포인터(*mut i32)
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // 러스트 대여 검사기에서는 values의 서로 다른 부분을 대여하는 것을 이해하지 못함
    // 다만 values를 두 번 대여한다는 것만 알고 있음
    // 따라서 unsafe 동작 정의가 필요함
    // (&mut values[..mid], &mut values[mid..])

    // 원시포인터를 이용하기 때문에 포인터의 유효성 신뢰에 대해 안전하지 않음
    unsafe {
        (
            // 원시포인터와 길이를 입력받아 슬라이스를 반환
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// extern
/// - 외래 함수 인터페이스(Foreign Function Interface, FFI)의 생성과 사용을 용이하게 하는 키워드
/// - "C" 부분은 외부 함수가 사용하는 ABI(Application Binary Interface)를 정의 (어셈블리 수준에서 함수 호출하는 방법을 정의)
///
extern "C" {
    fn abs(input: i32) -> i32;
}

/// 반대로 다른 언어세러 러스트 함수를 호출하기 위한 인터페이스를 만들 때
/// - extern 키워드를 추가하고, 관련 함수에 대한 fn 키워드 앞에 사용할 ABI 지정
/// - #[no_mangle] 어노테이션을 통해 컴파일러가 맹글링하지 않도록 지시
///     > 맹글링: 함수에 부여한 이름을 컴파일러가 컴파일 과정에서 다른 부분에서 사용할 수 있도록 더 많은 정보를 포함하지만 사람이 읽기 불편한 이름으로 변경하는 것
/// - 아래와 같이 외부 함수를 만들게 되면, 러스트 크레이트를 공유 라이브러리로 빌드하여, C 코드에서는 링크해 사용 가능
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


// 러스트는 정적 변수를 지원하지만, 소유권 규칙과 문제(데이터 경합)가 발생할 수 있음
// 전역 변수는 정적(static) 변수라 부름
// 상수와 달리 정적 변수는 메모리에 고정된 주소를 가짐
// 상수와 달리 정적 변수는 가변일 수 있음 (단, 가변 정적 변수에 접근 및 수정은 안전하지 ㅇ낳음)
static HELLO_WORLD: &str = "Hello, world!";
fn static_variable() {
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn add_to_count_caller() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

/// 안전하지 않은 트레이트
/// - 하나 이상의 메서드가 컴파일러가 확인할 수 없는 불변성(invariant)이 있는 경우
unsafe trait Foo {
    // 여기에 메소드가 작성됩니다
}

/// unsafe impl
/// 컴파일러가 확인할 수 없는 불변성은 우리가 지키겠다는 약속
unsafe impl Foo for i32 {
    // 여기에 메소드 구현이 작성됩니다
}