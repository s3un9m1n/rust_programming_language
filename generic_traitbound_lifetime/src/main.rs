use std::fmt::Display;

fn longest_with_an_announcment<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if y.len() < x.len() {
        x
    } else {
        y
    }
}

fn main() {
	
}