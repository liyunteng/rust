#[derive(Debug)]
enum X {
    Int(i32),
    Float(f64),
    Text(String),
}

fn test_x() {
    let row = vec![
        X::Int(3),
        X::Float(10.12),
        X::Text(String::from("blue"))
    ];
    for i in &row {
        println!("{:?}", i);
    }
}

fn test_read() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // panic
    // let x: &i32 = &v[100];
    let x = v.get(100);

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn test_create() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1,2,3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}

fn main () {
    test_create();
    test_read();
    test_x();
}
