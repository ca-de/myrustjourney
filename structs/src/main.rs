fn main() {
    // rust doesnt allow only certain fields to be mutable, whole thing only
    let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("someone@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@email.com");
    let email = user1.email;

    println!("{email}");

//    let user2 = User {
//        active: user1.active,
//        username: user1.username,
//        email: String::from("yetanotheremail@email.com"),
//        sign_in_count: user1.sign_in_count,
//    };

    // the above struct is the same as doing
    
    let user2 = User {
        email: String::from("yetanotheremail@email.com"),
        ..user1
    };

    // can no longer use user1 as a whole after crating user2 bc the String in username
    // field was moved. if we had given user2 new String values for both email and
    // username fields, then user1 would still be valid. active and sign_in_count
    // both implement the Copy trait, so they are fine
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs, no names associated with fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs no fields, like (), the unit type
// used for implementing a trait on some type but don't have any
// data that we want to store in the type itself
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,          // same as username: username,
        email,             // same as email: email,
        sign_in_count: 1,
    }
}

