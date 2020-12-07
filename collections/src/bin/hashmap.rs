use std::collections::HashMap;

fn test_update() {
    let mut m = HashMap::new();

    m.insert(String::from("Blue"), 50);
    println!("{:?}", m);

    m.insert(String::from("Blue"), 10);
    println!("{:?}", m);

    m.entry(String::from("Yellow")).or_insert(50);
    m.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", m);

    let text = "hello world wonderful world";
    let mut m = HashMap::new();
    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", m);
}

fn test_read() {
    let mut m = HashMap::new();

    m.insert(String::from("Blue"), 10);
    m.insert(String::from("Yellow"), 50);

    let k = String::from("Yellow");
    let v = m.get(&k);
    println!("{} = {:?}", k, v);

    for (k, v) in &m {
        println!("{}: {}", k, v);
    }
}

fn test_create() {
    let mut m = HashMap::new();

    m.insert(String::from("Blue"), 10);
    m.insert(String::from("Yellow"), 50);

    println!("{:#?}", m);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let scores = vec![10, 50];

    let m: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("{:#?}", m);

    let name = String::from("Favorite color");
    let value = String::from("Blue");
    let mut m = HashMap::new();
    m.insert(name, value);
    // `name` and `value` has moved
    println!("{:#?}", m);
}

fn main() {
    test_create();
    test_read();
    test_update();
}
