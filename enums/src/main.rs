enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// lets do it a more concise way

enum IpAddr2 {
    V4(String),
    V6(String),
}

// another representation

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// these structs are basically doing the same thing as the enum
// below it, but using structs we couldn't as easily define
// a function to take any of these kinds of messages as we could
// with the Message enum, a single type
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be here
    }
}

// very important enum as part of standard library:
//
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    // option
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {}

