#[allow(unused_variables)]
fn main() {
    // error: `let` statement have no return value
    // let x = (let y = 6);

    let x = 5;
    let y = {
        let x = 3;
        // if add ;, will be a statement
        x + 1
    };
    println!("The value of y is: {}", y);

    another_function(five(), plus_one(five()));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
