fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // modify a borrowed value with mut
    // note: if you have a mut reference, you can have
    // no other references to that value
    // like having
    // let r1 = &mut s;
    // let r2 = &mut s;
    // will fail!
    // helps prevent data races (similar to race condition)
    // can use scopes to make new references, just not simultaneous refs
    // also can't combine mut and immut references
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");

    // note that this below works bc
    // reference's scope starts at introduction
    // continues through last time ref is used
    let mut printme = String::from("hello");

    let r1 = &printme;
    let r2 = &printme;
    println!("{r1} and {r2}");

    let r3 = &mut printme;
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

