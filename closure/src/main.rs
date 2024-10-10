use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preferene: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_else() 인자에 클로저 함수 삽입 (표준 라이브러리에 우리가 정의한 함수를 전달)
        // 인자가 없기 때문에 || 사이에 아무런 값이 없음
        user_preferene.unwrap_or_else(|| self.most_stocked())

        // unwrap_or_else()는 FnOnce() 트레이트 바운드를 클로저 함수에 적용
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_blue < num_red {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // 클로저의 타입 추론기능 사용
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); //< 위에서 String 타입을 이용해 example_closure 의 x 에 대한 타입 추론이 되었기 때문에 i32 변수는 사용 불가

    // 클로저의 매개변수
    let list1 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list1);

    // 값(list1)을 캡처해서 사용
    // 불변 참조자는 동시에 여럿 가질 수 있기 때문에 접근 가능
    let only_borrows = || println!("From closure: {:?}", list1);

    println!("Before calling closure: {:?}", list1);
    // 클로저 또한 함수처럼 호출 가능
    only_borrows();
    println!("After calling closure: {:?}", list1);

    // 가변 참조자 캡처
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);

    // 클로저 정의 시 list2에 대한 가변 참조자를 캡처함
    // 따라서 클로저 정의 이후 이전처럼 "Before calling closure" 출력 사용 시 가변 참조자를 사용해서는 안 됨
    // 즉, 클로저 정의 이후 캡처된 list2는 더이상 클로저가 호출되지 않는 시점에 대여가 끝나게 되는 것으로,
    // 클로저 정의와 호출 사이에는 다른 대여가 허용되지 않음
    let mut boorws_mutably = || list2.push(7);
    boorws_mutably();
    println!("After calling closure: {:?}", list2);

    // 소유권 이전
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);

    // move 키워드는 클로저로 소유권을 넘기는 것
    // 메인 스레드와 소유권 참조하는 스레드 중 누가 먼저 죽을지 모르기 때문에 확실하게 이전하는게 나음
    thread::spawn(move || println!("from thread: {:?}", list3))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut num_sort_operations = 0;
    // sort_by_key()는 FnMut 트레이트 바운드의 클로저 사용
    // FnOnce는 캡처된 값을 클로저 밖으로 보내버리기 때문에 단 한번만 호출이 가능함
    // FnMut은 캡처된 값을 클로저 밖으로 보내지 않기 때문에 여러번 호출이 가능하고 캡처된 값을 변경시킬 수 있음
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
