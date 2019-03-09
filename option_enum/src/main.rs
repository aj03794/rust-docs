

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // Using none requires telling the compiler the type
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Can't add an i8 to an Option<i8>
    // the trait `std::ops::Add<std::option::Option<i8>>` is not implemented for `i8`
    // you have to convert Option<T> to T

    

    // let sum = x + y;
}
