#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Ip {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?} {:#?}", home, loopback);

    let home = Ip::V4(String::from("127.0.0.1"));
    let loopback = Ip::V6(String::from("::1"));
    println!("{:#?} {:#?}", home, loopback);

    let m = Message::Quit;
    m.call();

    let m = Message::Move { x: 1, y: 2 };
    m.call();

    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::ChangeColor(1, 2, 3);
    m.call();
}
