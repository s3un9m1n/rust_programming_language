pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn this_test_will_pass() {
        // stdout 결과를 생략 -> -- --show-output
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        // stdout 결과를 요약부분에서 출력
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        // [test] 함수 이름 중 add 를 지정하면 필터링에 의해 add가 포함된 함수만 호출 가능
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        // [test] 함수 이름 중 add 를 지정하면 필터링에 의해 add가 포함된 함수만 호출 가능
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        // [test] 함수 이름(one_hundred)을 지정해 해당 함수만 호출 가능
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // [ignore] 어노테이션은 특별한 옵션 없이는 테스트를 하지 않음
        // -- --ignored 옵션을 줘야 ignored 어노테이션 테스트들을 실행함
        // -- --include-ignored 를 주면 모든 테스트 실행

        // code that takes an hour to run
    }
}
