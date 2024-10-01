pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
    // format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } else if 100 < value {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // assert는 인자로 주어진 값이 true일 경우 정상, false일 경우 panic 호출
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // false를 확인해야하기 때문에 assert!() 안에 !를 붙여 false값이 true로 전달될 수 있도록 함
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_to() {
        // assert 안에 == 를 두는 것보다 정확하게 알 수 있음
        assert_eq!(5, add_two(2));

        // assert_eq, assert_ne 매크로로 비교할 값은 PartialEq(같은 비교용), Debug(panic 출력용) 트레이트를 구현해야 함
        // -> #[derive(PartialEq, Debug)
    }

    // custom message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // greeting 전체 메시지가 변경 가능할 수 있기 때문에, 테스트는 정확한 값 검사가 아닌 입력 매개변수 값만 검사하도록 가정
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", //< 필수 인자 뒤에 커스텀 메시지 추가 가능
            result
        );
    }

    // 함수가 에러 조건을 처리하는지 확인하기 위함(panic 발생을 체크)
    #[test]
    // 의도한 panic이 아니더라도 테스트가 통과할 수 있어 정확하다고 볼 수 없음
    // -> expected 매개변수로 실패 메시지를 지정할 경우 보다 꼼꼼해짐
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Result<T, E> 테스트 시에는 should_panic 어노테이션 사용 불가능
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            // panic 대신 Err를 이용해 assert_eq! 매크로를 대신함
            Err(String::from("two plus two does not equal four"))
        }
    }
}
