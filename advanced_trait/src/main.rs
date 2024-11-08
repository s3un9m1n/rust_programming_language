//! 여기서의 대부분의 고급 기능은 거의 불필요

/// 연관 타입
/// 타입 자리 표시자와 트레이트를 연결하여,
/// 트레이트 메서드 정의 시,
/// 자리표시자 타입을 시그니처에서 사용할 수 있도록 함
///
/// 트레이트의 구현자는
/// 자리 표시자 타입 대신 사용할 구체적 타입을 지정
///
/// -> 트레이트가 구현될 때까지 해당 타입이 무엇인지 정확히 알 필요 없음
/// -> 임의 타입 사용하는 트레이트 정의 가능
///
/// ex) 표준 라이브러리의 `Iterator` 트레이트
/// - `Iterator` 트레이트의 구현자는 `Item`의 구체적 타입을 지정
/// - `next` 메서드는 구체적 타입의 값을 갖고 있는 `Option`을 반환
///
/// 연관 타입 자체는 제네릭과 비슷한 개념처럼 보일 수 있으나,
/// 제네릭은 처리 가능한 타입을 지정하지 않으면서 함수를 정의할 수 있음
/// 제네릭은 사용하는 쪽에서 타입을 지정해줘야 함
/// 연관 타입은 트레이트 구현 시 타입을 결정함
///
/// 제네릭은 다양한 타입을 지원할 수 있지만, 타입인자가 많아질 수 있음
/// 연관 타입은 특정 타입을 고정하지만 코드가 단순해지고 명확해짐
pub trait Iterator {
    // 연관타입 Item
    // Item은 자리표시자
    type Item;

    // `next` 메서드 정의는 `Option<Self::Item>` 타입의 값을 반환
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
    // 사용할 타입을 외부에서 인자처럼(제네릭) 받지 않고
    // 직접 트레이트를 구현(impl)할 때 타입을 지정
    type Item = u32; // 연관 타입을 u32로 구현

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

/// 제네릭 타입 매개변수를 이용하면, 제네릭 타입에 대해 기본 구체 타입을 지정 가능
/// 기본 타입이 작동하는 경우에는 트레이트의 구현자는 구체적 타입을 지정할 필요가 없음
/// - `<PlaceholderType=ConcreteType>` 문법
/// - 연산자 오버로딩하는 경우 유용하게 함께 사용
///
/// 러스트는 기본적으로 자체 연산자를 만들거나 임의의 연산자를 오버로딩할 수 없으나,
/// `std::ops`에 나열된 연산자와 연관된 트레이트를 구현해 연산자 및 트레이트 오버로딩 가능
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add 트레이트 안에 기본 제네릭 타입 적용
///
/// // `Rhs=Self` : 기본 타입 매개변수(default type parameter)
///
/// trait Add<Rhs=Self> {
/// type Output;
///
/// fn add(self, rhs: Rhs) -> Self::Output;
/// }
impl Add for Point {
    // `Output`이라는 연관타입
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// 뉴타입 패턴
/// 기존 타입을 다른 구조체에서 얇게 감사는 것
/// ex) Millimeters, Meters 구조체 관계
struct Millimeters(u32);
struct Meters(u32);

// Self 대신 Meters로 Rhs 타입 매개변수 설정
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/// 모호성 방지를 위한 정규화 문법
/// - 같은 이름의 메서드 호출하기
///
/// 러스트에서는 어떤 트레이트에 다른 트레이트의 메서드와 같은 이름의 메서드를 막지 ㅇ낳음
/// 한 타입에서 두 트레이트를 모두 구현하는 것도 막지 않음
/// 단, 같은 이름의 메서드를 호출할 때는 어떤 메서드를 사용할지 러스트에게 알려저야 함
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/// 슈퍼트레이트
/// 한 트레이트에서 다른 트레이트의 기능 구현 요구 (다른 트레이트에 의존)
///
/// 첫번째 트레이트를 구현하려면 두번째 트레이트도 구현하도록 요구
/// -> 트레이트 정의가 두번째 트레이트의 연관 아이템을 활용 가능
/// -> 슈퍼트레이트
use std::fmt;

// OutlinePrint 트레이트는 Display 트레이트를 필요하다 지정함
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // Display 트레이트의 to_string()을 OutlinePrint에서도 사용 가능
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Display를 구현하지 않은 Point를 사용할 경우 빌드에 실패함
impl OutlinePrint for Point {}

// 아래의 코드로 Point에서도 Display를 구현했기 때문에 빌드에 성공함
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 뉴타입패턴
/// 튜플 구조체(wrapper)로 새로운 타입을 생성하는 방법
/// 고아 규칙(트레이트나 타입이 우리 크레이트의 것인 경우에만 트레이트 구현 가능)을 우회할 수 있음
///
/// -> wrapper 타입은 우리 크레이트 내에 있게 되어 래퍼에 대한 트레이트 구현 가능
/// 런타임 성능 불이익은 없으며 컴파일 시 제거됨

// Vec<T>에 대해 Display를 구현하고 싶을 경우
// Display 트레이트와 Vec<T> 타입이 크레이트 외부에 정의되어 있으므로 직접 구현은 불가함
// Vec<T> 인스턴스를 보유하는 Wrapper 구조체를 만들 경우, Wrapper에 Display를 구현하고 Vec<T>를 사용 가능
// 즉, Vec<T>에 대한 Display 구현이 아닌, Wrapper에 대한 Display 구현으로 Vec<T>에 대한 Display 구현처럼 우회가능
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Vec<T>의 0번째에 접근함
        write!(f, "[{}]", self.0.join(", "))
    }
}


fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // &self 인자가 주어져있어야만 호출 가능
    // 혹은 정규화 문법을 통해서만 사용 가능
    Pilot::fly(&person);
    Wizard::fly(&person);
    // Human::fly(&person)
    person.fly();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
