#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y:i64}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pated \"{}\"", s),
        WebEvent::Click {x, y} => println!("clicked at x={} y={}", x, y),
    }
}


enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x+ y,
            Self::Substract => x - y,
        }
    }
}


enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!{"{}", x.run(1, 2)};


    println!("zero is {}", Number::Zero as i32);
    println!("on is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
