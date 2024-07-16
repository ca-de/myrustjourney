fn main() {
    {
        let mut s = String::from("hello");

        s.push_str(", world!");

        println!("{s}");
    }
    // s is no longer valid now that we are out of scope
    // also, setting another variable, say s2, equal to s in the same scope, will make s no longer valid
    // this is also called a 'move', instead of a shallow copy
    
    // deep copy/clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }

    // giving away ownership to functions vs copy
    {
        let s3 = String::from("hello");

        takes_ownership(s3); // s3's value moves into function and is no longer valid

        let x = 5;

        makes_copy(x); // x is an int, which is Copy, so x remains valid
    }

    // returning to move ownership
    {
        let s1 =  gives_ownership();

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);

        // s2 moved to s3, so not valid
        println!("s1 = {s1}, s3 = {s3}");
    }

    // return multiple values using a tuple
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

// creates a string and passes ownership to caller
fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

// takes ownership of string then returns ownership of string
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

