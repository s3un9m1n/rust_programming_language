#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangels is {} square pixels.",
        rec1.area()
    );

    println!("rec1 is {:?}", rec1);

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
