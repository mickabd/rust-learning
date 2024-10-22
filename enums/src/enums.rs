fn main() {
    let _four = IpAddrKind::V4;

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home = IpAddrWithValues::V4(String::from("127.0.0.1"));

    let _loopback = IpAddrWithValues::V6(String::from("::1"));

    let _home = IpAddrWithDifferentInputs::V4(127, 0, 0, 1);

    let _loopback = IpAddrWithDifferentInputs::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    // let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrWithValues {
    V4(String),
    V6(String),
}

enum IpAddrWithDifferentInputs {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// instead of
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
