//! 주석을 담고 있는 아이템을 문서화

/// 문서화 주석
/// - 기존의 두 개의 슬래쉬로 구성된 주석이 아님
/// - HTML 문서를 생성할 수 잇음
/// - 어떻게 구현되었는지가 아닌 어떻게 사용하는지에 대해 공개 아이템들의 API 문서

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
