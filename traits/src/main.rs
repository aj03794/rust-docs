// Example
// Multiple structs that hold various kinds and amounts of text
// "NewArticle" struct that holds a news story filled in
// a particular location and a "Tweet" that can have at most
// 280 characters along with metadata whether it was a new
// tweet, a retweet, or a reply to another tweet

// Media aggregator library that can display summaries of data
// that might be stored in a "NewArticle" or "Tweet" instance
// To do this, we need a sumary from each type, and we need
// to request that summary ne called a "summarize" method on
// an instance

// Name of trait is "Summary"
// Inside the curly brackets, we declare the method signatures
// that describe the behaviors of the types that implement this trait
// which in this case is "fn summarize(&self) -> String"

// Each type implementing this trait must provide its own custom
// behavior for the body of the method

// A trait can have multiple methods in its body
// The method signatures are listed one per line and each line
// ends in a semicolon

// Summary trait needs to be a public trait for another
// crate to implement it
// so if this was a trait in a library, it would need to be public

// We can implement a trait on a type only if either the trait
// or the type is local to our crate
// We can't implement the "Display" trait on "Vec<T>" b/c
// "Display" and "Vec<T>" are defined in the std lib and they
// aren't local to our "aggregator" crate

// This restriction is part of a property of programs called
// "coherence" and more specifically the "orphan rule"
// it is named this b/c the parent type is not present
// This rule ensures that other people's code can't break your
// code and vice versa
// W/o the rule, two crates could implement the same trait for the
// same type, and Rust wouldn't know which implementation to use
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
}
