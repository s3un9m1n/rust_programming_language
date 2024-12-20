/// Draw 트레이트를 정의함으로써,
/// Draw 트레이트 객체를 담는 백터를 정의할 수 있음
///
/// 트레이트란,
/// 특정 동작(메서드)을 정의한 인터페이스로, 타입이 그 동작을 구현하도록 강제할 수 있음
///
/// 트레이트 객체란,
/// 트레이트를 구현한 값을 동적으로 처리하기 위한 객체로, 런타임에 타입이 결정됨
/// 여러 타입을 공통 처리 가능
///
/// & 참조자, Box<T> 스마트 포인터 등을 dyn 키워드와 함께 사용하여 관련된 트레이트를 특정하면,
/// 트레이트 객체를 생성할 수 있음
///
/// 컴파일 타임에 타입이 결정되는 제네릭 타입이나 구체 타입 대신
/// 트레이트 객체를 이용하면 런타임에 타입을 동적으로 처리가 가능하여
/// 좀 더 유연한 코드 작성이 가능함
///
/// 트레이트 객체를 사용하는 값은 해당 트레이트를 반드시 구현해야 함
/// 즉, 트레이트 객체를 사용하면 그 트레이트에서 정의된 메서드가 구현됨이 보장됨
/// 따라서, 컴파일 타임에 모든 타입을 알 필요가 없음 (트레이트 객체는 런타임에 타입이 결정되기 때문)
///
/// 러스트는 다른 언어의 객체와 구분하기 위해 구조체와 열거형을 객체라 부르는 것을 지향함
/// 러스트의 구조체나 열거형에서는 구조체의 필드와 impl 블록의 동작이 분리됨
///
/// 트레이트 객체는 데이터와 동작을 결합한다는 의미에서 다른 언어의 객체 개념과 더 비슷함
/// 하지만 트레이트 객체는 트레이트 객체에 데이터를 추가할 수 없다는 점에서 전통적인 객체와 다름
/// 다른 언어에서의 객체는 공통된 동작에 대한 추상화가 목적이기 때문에 상속을 사용해 클래스를 확자ㅏㅇ하지만
/// 러스트에서는 트레이트로 유연하게 다양한 타입에서 공통 동작을 정의하는 방식으로 다형성 지원

// draw 메서드를 갖는 Draw 트레이트 정의
pub trait Draw {
    fn draw(&self);
}

// components 벡터를 보유하는 Screen 구조체 정의
pub struct Screen {
    // 벡터 타입은 Box<dyn Draw>
    // Box<dyn Draw>가 트레이트 객체
    // Draw 트레이트를 구현한 Box안의 모든 타입에 대한 영역
    pub components: Vec<Box<dyn Draw>>,
}

// Screen 구조체에 run 메서드 정의
impl Screen {
    // run 메서드는 components 각 요소마다 draw 메서드를 호출
    // run 안에서는 각 컴포넌트가 어떤 구체 타입인지 알 필요가 없음
    // 따라서, 런타임에 어떤 값이 특정 메서드를 구현했는지 검사할 필요가 없음 (컴파일 불가)
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 제네릭을 사용한 경우에는 components의 타입은 모두 다를 수 없음
// 컴파일 타임에 결정되기 때문에 모두 Button 타입이거나 모두 TextField 타입이거나 됨
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Button에서는 Draw 트레이트를 구현
// Button에선는 Draw 트레이트 뿐 아니라 추가적인 impl을 통해 추가 메서드를 가질 수 잇음
impl Draw for Button {
    fn draw(&self) {
        // 실제로 버튼을 그리는 코드
    }
}
