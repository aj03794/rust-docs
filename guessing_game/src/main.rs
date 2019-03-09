// std is the standard library
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    // println!("Please input your guess.");

    // // mut allows variable to be mutable
    // // :: indicates that that new is an associated function
    // // of the String type
    // // some languages call this a static method
    // let mut guess = String::new();

    // // & indicates that this argument is a references
    // // Returns a value of type io::Result
    // // 
    // io::stdin().read_line(&mut guess)
    //     .expect("Failed to read line");
    
    // // Rust allows us to "shadow" the previous value of guess with a new one
    // // This feature is often used in situations where you want to convert
    // // a value from one type to another
    // // trim method removes \n that is added from the read line
    // let guess: u32 = guess.trim().parse()
    //     .expect("Please type a number!");

    // println!("You guessed: {}", guess);

    // // comparison here means that Rust will infer that secret_number
    // // will be a u32 as well
    // match guess.cmp(&secret_number) {
    //     // These are arms
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!")
    // }

    // Creates infinite loop
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        // Switching from an expet call to a match expression is how you generally
        // move from crashing n an error to handling the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not convertable to a u32");
                continue
            },
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            // These are arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // This exits the loop which also means exiting th eprogram
                break;
            }
        }
    }
    
}
