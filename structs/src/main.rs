// Custom data type that lets you name and package together
// multiple related values that make up a meaningful group

// Structs and enums are the building blocks for creating
// new types in your program

#[derive(Debug)]
struct User {
    // fields
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// Tuple structs
// Just have the types of the fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someusername12"),
        active: true,
        sign_in_count: 1
    };
    println!("{:?}", user1);
    println!("{:?}", user1.email);
    user1.email = String::from("anotheremail");
    println!("{:?}", user1.email);
    println!("-------------");
    let user2 = build_user(String::from("some-email"), String::from("some-username"));
    println!("{:?}", user2);
    let user3 = User {
        email: String::from("some email"),
        username: String::from("some username"),
        ..user1
    };
    println!("==============");
    println!("{:?}", user3);
}
