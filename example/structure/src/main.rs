#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};

    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4};

    println!("point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point {x: 0.1, ..point};
    println!("seconde point: ({}, {})", new_point.x, new_point.y);

    let Point {x: my_x, y: my_y} = point;
    let _rectange = Rectangle {
        p1: Point {x: my_y, y: my_x},
        p2 :point,
    };

    let _ni = Nil;

    let pair = Pair(1, 0.1);
    println!("pari contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
