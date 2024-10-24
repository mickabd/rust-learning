fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    build_user(
        String::from("again@example.com"),
        String::from("anotherusername"),
    );

    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    colors();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // This is shorthand for username: username,
        email,    // This is shorthand for email: email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn colors() {
    let _black = Color(0, 0, 0);
    println!("Black: {}, {}, {}", _black.0, _black.1, _black.2);
    let _origin = Point(0, 0, 0);
    println!("Origin: {}, {}, {}", _origin.0, _origin.1, _origin.2);
}
