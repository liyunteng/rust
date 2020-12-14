use ansi_term::Colour;
use ansi_term::Style;

fn test3() {
    println!("{}, {} and {}",
            Colour::Yellow.paint("This is colored"),
            Style::new().bold().paint("this is bold"),
    Colour::Yellow.bold().paint("this is bold and colored"));
}

fn test2() {
    println!("{} and this is not",
    Style::new().bold().paint("This is Bold"));
}

fn test1() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}

fn main() {
    test1();
    test2();
    test3();
}
