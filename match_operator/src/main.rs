#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let mut x = value_in_cents(Coin::Penny);
    x = value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

    // prints 7
    let some_u8_value = 7u8;
    // Prints nothign because it goes into _ arm
    let some_other_u8_vaue = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // This is a catch all
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // these are called match "arms"
        // First part is the "pattern" and after the "=>" is the code
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// #[derive(Debug)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // Both of these variants are required
        // Matches in Rust are exhaustive
        None => None,
        Some(i) => Some(i + 1)
    }
}
