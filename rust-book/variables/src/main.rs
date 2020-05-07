#![allow(clippy::many_single_char_names)]
#![allow(unused_variables)]

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points: {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tuple;
    println!("Destructured tuple a: {} b: {} c: {}", a, b, c);
    println!("Another way: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let array = [1, 2, 3, 4, 5];
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let five3s = [3; 5];
    println!("First: {}", array[0]);
    println!("Three threes: {} {} {}", five3s[0], five3s[1], five3s[2]);
}
