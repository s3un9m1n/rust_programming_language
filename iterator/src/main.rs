fn main() {
    let v1 = vec![1, 2, 3];

    // 반복자를 생성할뿐 어떤 유용한 동작도 하지 않음
    let v1_iter = v1.iter();

    // 자동으로 다음 반복자를 가리킴
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // 가변으로 만들어야 직접 next 메서드 호출 가능
    // for 루프에서는 가변 iterator를 쓰지 않는 이유는 for 루프가 iterator 소유권을 갖고 내부적으로 가변처리 함
    let mut v1_iter = v1.iter();

    // Iterator 트레이트는 next 메서드 정의를 요구함
    // next 메서드는 직접 호출 가능함
    // next 메서드의 반환값은 값이 있을 경우 Some, 값이 없을 경우 None
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // 소비 어댑터(consuming adaptor)
    // 반복자 소유권을 가져온 다음에 반복자를 소비함
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
