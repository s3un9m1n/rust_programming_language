fn main() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);

    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];
    //    v3.push(6); //< immutable first 변수 사용이 완료되지 않았기 때문에 mutable 변수를 넣을 수 없음 -> vector는 값을 넣을 때 메모리에 추가 공간이 없으면 vector 전체를 다른 메로리에 새로 할당하기 때문에 안전성 보장하기 위함
    println!("The first element is: {first}");

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{i}");
    }
}
