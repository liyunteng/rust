fn main() {
    // mut
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // operator
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!(
        "sum = {}\ndifference = {}\nproduct = {}\nquotient = {}\nremainder = {}",
        sum, difference, product, quotient, remainder
    );

    // char
    let c = 'z';
    let z = 'Z';
    let heart = '♡';
    println!("c={} z={} heart={}", c, z, heart);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup.0={} tup.1={} tup.2={}", tup.0, tup.1, tup.2);
    println!("tup = {:?}, x={} y={} z={}", tup, x, y, z);
    println!("tup = {:#?}", tup);

    // array
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    // let index = 6;
    // println!("a[6] = {}", a[index]);

    let mounths = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "Octoberf",
        "November",
        "December",
    ];
    println!("mounths = {:?}", mounths);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    let a = [3; 5];
    println!("a = {:?}", a);
}
