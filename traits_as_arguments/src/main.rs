pub trait Summary {
    fn summarize(&self) -> String;
}

// This is shorthand for
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// If we wanted to take two things that implement Summary
// This works well if item1 and item1 were allowed to have different
// types - as long as both implement "Summary"
pub fn notifyTwo(item1: impl Summary, item2: impl Summary) {}

// If you want to force both to have the same exact type
// only possible using a "trait bound"
pub fn notifyThree<T: Summary>(item1: T, item2: T) {}

// ** Specify multiple traits with + **
// If notify needed to display formatting on "item" as well as
// use the "summarize" method, then "item" would need to implement
// two different traits at the same time: "Display" and "Summary"
// pub fn notifyFour<T: Summary + Display>(item: T) {}

// ** "where" clauses for clearer code
// Downsides to using too many trait bounds
// Each generic has its own trait bounds
// so many functions with multiple generic type parameters can
// have lots of trait bound information btwn a function's name
// and its parameter list, making the function signature hard to read

// fn some_function<T: Displa + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

// }

// with a where clause

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
        U: Clone + Debug
{
    
}

fn main() {
    println!("Hello, world!");
}
