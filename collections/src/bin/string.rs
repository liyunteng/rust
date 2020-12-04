fn test_index() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }
    for c in hello.bytes() {
        println!("{}", c)
    }
}

fn test_update() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 is moved
    let s3 = s1 + &s2;
    println!("{}",  s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    let ss = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    println!("{}", ss);
}

fn test_create() {
    let mut s = String::new();

    let data = "initial conents";
    let s = data.to_string();
    println!("{}", s);
    let s = String::from(data);
    println!("{}", s);
    let s = String::from("你好");
    println!("{}", s);
}

fn main() {
    test_create();
    test_update();
    test_index();
}
