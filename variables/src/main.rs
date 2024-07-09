use std::io;

fn main() {
    // variable basics
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours");


    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");


    let spaces = "   ";
    let spaces = spaces.len();

    println!("There are {spaces} spaces.");

    // float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // operators
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    let remainder = 43 % 5;

    // bool
    let t = true;

    let f: bool = false; // explicit

    //char
    let c = 'z';
    let z: char = 'ℤ'; // explicit
    let heart_eyed_cat = '😻';

    // compound types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    
    // WILL THROW INDEX OUT OF BOUNDS IF NOT VALID INDEX
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is {element}");
}

