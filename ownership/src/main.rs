// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     let s3 = s1;

//     // println!("{}, world!", s1);
// }

//error[E0382]: use of moved value: `s1`
//  --> src/main.rs:5:28
//   |
// 3 |     let s2 = s1;
//   |         -- value moved here
// 4 |
// 5 |     println!("{}, world!", s1);
//   |                            ^^ value used here after move
//   |
//   = note: move occurs because `s1` has type `std::string::String`, which does
//   not implement the `Copy` trait

fn main() {
    let s = String::from("hello");  // s comes into scope

    // takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    // This doesn't work
    println!("{}", s);

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.