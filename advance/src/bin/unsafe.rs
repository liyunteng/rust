unsafe fn dangerous() {}

static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

union Abc {
    a: i32,
    b: f64,
}

fn test6() {
    let x = Abc { a: 1 };
    // access union
    unsafe {
        println!("{:?}", x.a);
    }
}

unsafe trait Foo {}

// impl unsafe trait
unsafe impl Foo for i32 {}
fn test5() {
    println!("name is : {}", HELLO_WORLD);
    // change mut static
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn test4() {
    // call C
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn test3() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    println!("{:?}", a);
    println!("{:?}", b);
}

fn test2() {
    // call unsafe function
    unsafe {
        dangerous();
    }
}

fn test1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // deref raw pointer
    unsafe {
        *r2 = 6;
        println!("{} {}", *r1, *r2);
    }

    let address = 0x12345usize;
    println!("{}", address);
    let _r = address as *const i32;
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}
