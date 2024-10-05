use test_organization;

mod common;

// 통합테스트는 유닛테스트가 성공해야 진행됨

#[test]
fn it_adds_to() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}