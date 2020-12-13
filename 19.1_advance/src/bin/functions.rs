fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn test4() {
    let x = returns_closure()(1);
    println!("{}", x);
}
//////////////////////////////////////////////////

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn test3() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

//////////////////////////////////////////////////
fn test2() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);
}

//////////////////////////////////////////////////
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test1() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}
