fn main() {
    test_string();
    test_ownership();
    test_ownership1();
    test_ownership2();
    test_ownership3();
    test_ownership4();
}

fn test_string() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn test_ownership() {
    let s = String::from("hello");

    takes_ownership(s);

    // E0382 borrow of moved value
    // println!("{}", s);

    let x = 5;

    makes_copy(x);

    // x is Copy
    println!("{}", x);
}

fn test_ownership1() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{} {}", s1, s3);
}

fn test_ownership2() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length2(&s2);
    println!("The length of '{}' is {}.", s2, len);
}

fn test_ownership3() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn test_ownership4() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    // E0502 cannot borrow `s` as mutable because it
    // is also borrowed as imutable
    // println!("{} {} {}", r1, r2, r3);
    println!("{}", r3);
}

fn test_dangling() {
    let reference_to_nothing = dangle1();
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_interger: i32) {
    println!("{}", some_interger);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     // E0106 missing lifetime specifier
//     &s
// }

fn dangle1() -> String {
    let s = String::from("hello");
    s
}
