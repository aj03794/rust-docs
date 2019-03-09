// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
//  --> error.rs:8:5
//   |
// 7 | fn change(some_string: &String) {
//   |                        ------- use `&mut String` here to make mutable
// 8 |     some_string.push_str(", world");
//   |     ^^^^^^^^^^^ cannot borrow as mutable

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}