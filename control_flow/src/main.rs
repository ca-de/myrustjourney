fn main() {
    let x = 6;

    if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 { 
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    
    println!("The value of num is: {num}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The end result is: {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // this will run until we manually interrupt it
    // loop {
    //    println!("again!");
    //}

    let mut y = 3;

    while y != 0 {
        println!("{y}!");

        y -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // prettier way
    for element in a {
        println!("the value is: {element}");
    }
}

