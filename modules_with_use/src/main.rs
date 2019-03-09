mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("hello from clarinet")
        }
    }
}

// With absolute path
// use crate::sound::instrument;

// With relative path
// use self::sound::instrument;


// **UNIDIOMATIC WAY TO BRING FUNCTION INTO SCOPE**
// Should always bring in the function's parent module
// use crate::sound::instrument::clarinet

mod performance_group {
    // self::sound::instrument doens't work
    // IDIOMATIC WAY TO BRING FUNCTION INTO SCOPE
    // If you want to enable code calling your code to be able to refer
    // to the type as if it was defined in the scope just as your code does,
    // you can combine `pub` and `use`
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

// Idiomatic way to bring in HashMap
use std::collections::HashMap;
// Unidiomatic way to bring in HashMap
// use std::collections

// Exception to this rule is if the `use` statements would bring two items
// with the same name into scope

// Renaming types brough into scope with the `as` keyword
// Idiomatic way to bring in two items with the same name into the same scope
use std::fmt::Result;
use std::io::Result as IoResult;

// This is the same as the below
use std::{ cmp::Ordering, io };
// use std::cmp::Ordering;
// use std::io;

// Another example
use std::io::{ self, Write };
// use std::io;
// use std::io::Write;

// Bring all public definitions into scope
// use std::collections::*;

use rand::Rng;

fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
    let mut map = HashMap::new();
    map.insert(1, 2);
    for key in map.keys() {
        println!("{}", key);
    }
    for values in map.values() {
        println!("{}", values);
    }
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
