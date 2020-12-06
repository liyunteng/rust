#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
            println!("{:?}", c);
        }
        println!("count after c gose out of scope = {}", Rc::strong_count(&a));
        println!("{:?}", b);
    }
    println!("count after b gose out of scope = {}", Rc::strong_count(&a));

    println!("{:?}", a);

}
