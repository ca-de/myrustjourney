fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // empties the String, index is now invalid!
    
    // this will compile BUT
    // word will have the value 5 here, but there's no more
    // string that we could use the value 5 with. the contents 
    // of s have changed. no guarantee that 5 will be valid when 
    // the two are different values.
    
    // slices
    let mut pizza = String::from("hello world"); // same as [0..len] and [..]

    let hello = &pizza[0..5]; // same as [..5]
    let world = &pizza[6..11]; // same as [6..] and [6..len]

    let sliced_word = first_word_slicer(&pizza);
    println!("{sliced_word}");

    // now if we run a clear and try to print the word
    // we will error, bc clear needs to get a mut ref
    // and we are using immut borrows

    let literal = "literal string";
    let sliced_literal = first_word_slicer_str(&literal);

    println!("{sliced_literal}");

    // other slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// booooo
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// yaaaay
fn first_word_slicer(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// even more yaaay, works on string literal values now too
fn first_word_slicer_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

