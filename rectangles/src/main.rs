// ****BASIC IMPLEMENTATION****

// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!(
//         "The area of the rectange is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }



// ****WITH TUPLES****
// Tuple lets us add some structure, just passing one argument
// This version is a bit less clear b/c tuples don't name their elements

// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectanlge is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }




// ****STRUCTS****

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//     println!(
//         "The area of the rectanlge is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// **ADDING USEFUL FUNCTIONALITY WITH DERIVED TRAITS**
// Rust doesn't guess how it should display more advanced types like structs
// Due to this, structs don't have a provided implementation of `Display`

// Need line below to work println below
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//     // :? (could also do :#?) tells us we want to use an output
//     // format called `Debug`
//     // Debug trait enables us to print our struct in a way that is useful
//     // for developers so we can see its value while we're debugging
//     // our code
//     println!("rect1 is {:#?}", rect1);
// }


// ****METHODS****
// Defined within the context of a struct (or an enum or a trait object)
// Their first parameter is always `self` which represents the instance
// of the struct the method is being called on

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Could break these into separate impl blocks if we wanted
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

impl Rectangle {
    // associated function, these are not TECHNICALLY methods
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let square = Rectangle::square(30);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square: {:?}", square);
}