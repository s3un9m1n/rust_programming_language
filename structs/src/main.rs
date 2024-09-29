struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(&user1.email, &user1.username);
    println!(
        "user1:{} {} {} {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );
    println!(
        "user2-email:{}, user2-username:{}",
        user2.email, user2.username
    );

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    //    println!(
    //       "user2:{} {} {} {}",
    //        user2.active, user2.username, user2.email, user2.sign_in_count
    //    );
    println!(
        "user3:{} {} {} {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }
}
