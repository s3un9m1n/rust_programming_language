//! 다른 OOP 언어에서는 상속이 있어 런타임에 타입을 알 수 있는 다양한 경우에 유용함
//! 예를 들어, GUI 도구를 만들 때 draw 메서드를 갖는 Component를 상속시켜 Button, Imange, SelectBox 같은 것들을 만들 수 있음
//! 각자 draw를 오버라이딩해 고유 동작을 정의하지만 프레임워크에서는 모든 타입을 Component인 것처럼 처리함
//!
//! 러스트에는 상속이 없기 때문에 새로운 타입을 정의하고 확장할 수 있도록 구조화하는 다른 방법이 필요

fn main() {
    println!("Hello, world!");
}
