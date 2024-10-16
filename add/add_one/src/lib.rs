pub fn add_one(x: i32) -> i32 {
    x + 1
}

// 작업공간에서 cargo test는 모든 하위 크레이트의 모든 테스트를 수행
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
