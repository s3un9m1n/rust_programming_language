mod front_of_house; // 다른 언어에서의 include 와는 다름

fn deliver_order() {}

// 모듈
mod back_of_house {
    // 구조체는 공개더라도 내부 필드는 각기 다른 공개 범위
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 구조체 비공개 필드가 있기 때문에 생성해주는 공개 연관 함수(summer()) 제공 필요
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 열거형은 구조체와 달리 공개 설정 시 모든 배리언트가 공개
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incoreect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting; // use 대신 pub use 를 사용하면 외부에서 restaurant 라이브러리를 사용할 때 restaurant::front_of_house::hosting::add_to_waitlist() 대신 restaurant::hosting::add_to_waitlist() 사용 가능

pub fn eat_at_restaurant() {
    // 호밀(Rye) 토스트 조식
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}
