#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("li_yunteng@163.com"),
        username: String::from("liyunteng"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("yli@addx.ai");
    println!("{:#?}", user1);

    println!(
        "{:#?}",
        build_user(String::from("abc@163.com"), String::from("abc"))
    );

    let user2 = User {
        email: String::from("yli@addx.ai"),
        ..user1
    };
    println!("{:#?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?} {:?}", black, origin);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
