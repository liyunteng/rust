use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pencakes;

fn test2() {
    Pencakes::hello_macro();
}
//////////////////////////////////////////////////
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
fn test1() {
    let v: Vec<u32> = my_vec![1, 2, 3];
    println!("{:?}", v);
}

fn main() {
    test1();
    test2();
}
