#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        println!("####");
        &self.y
    }
}

fn main() {
    let integer = Point { x: 5, y: 'c' };
    let float = Point { x: 1.0, y: 4 };
    println!("interger = {:?} float = {:?}", integer, float);
    println!("integer.x = {} integer.y = {}", integer.x(), integer.y());
}
