fn main() {
    let s = String::from("hello world!");
    // `String` to &str
    let word = first_word(&s[..]);
    println!("{}", word);

    let s = "hello world";
    // &str to slice
    let word = first_word(&s[..]);
    println!("{}", word);

    // &str
    let word = first_word(s);
    println!("{}", word);

    // E0502 cannot borrow `s` as mutable
    // because it is also borrowed as immutable
    // println!("the first world: {}", word);
}

fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
