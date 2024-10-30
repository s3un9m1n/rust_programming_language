struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message3 {
    Hello { id: i32 },
}

fn main() {
    // 리터럴 매칭
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 변수 매칭
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 위의 y와 다름
        // 바인딩되어 x Some 내부 값이 y에 맵핑되어 결정됨
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // 다중 패턴
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 범위 매칭 #1
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 범위 매칭 #2
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 값 해체해서 분리
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 필드 축약
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        // y가 0인 경우
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // x가 0인 경우
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // 모든 경우
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // 열거형 해체
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // 중첩된 열거형 매칭
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // 복잡한 구조체와 튜플 해체
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 중첩된 _로 값의 일부 무시
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // _로 시작하는 이름으로, 사용하지 않는 변수 무시
    // _x는 경고 안나옴
    let _x = 5;
    let y = 10;

    // ..로 값의 나머지 무시 #1
    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    // ..로 값의 나머지 무시 #2
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // 매치 가드(match guard) 사용
    // match 갈래 패턴 뒤에 지정되는 추가 if 조건
    // 매칭되려면 if 조건도 만족해야 함
    // 패턴가지고는 표현할 수 없는 더 복잡한 아이디어 표현 유용
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // @ 바인딩
    let msg = Message3::Hello { id: 5 };

    match msg {
        // id가 3~7 사이에 있는지
        // @를 사용하면 패턴 매칭 여부 테스트를 함과 동시에 해당 값을 갖는 변수를 만들 수 있음
        // 여기서는 3~7 사이에 있으면 id_variable 변수에 바인딩하여 사용 가능
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // id가 10~12 사이에 있는지
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
