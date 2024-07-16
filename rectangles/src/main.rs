#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// multiple impl blocks per struct are allowed
// use case will be learned later, supposedly
impl Rectangle {
    // Self is first parameter of all methods. can take ownership,
    // borrow immutably as below, or borrow mutably (&mut self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 49,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // dbg! macro prints to stderr, prints line number, takes ownership of expression
    // and returns ownership of the value
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width, it is {}", rect1.width);
    }

    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // use associated function
    let sq = Rectangle::square(3);

    println!("sq is {sq:#?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

