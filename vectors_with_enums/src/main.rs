// Rust needs to know what types will be in the vector at compile time
// so it knows exactly how much memory on the heap will be needed
// to store each element

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    // Can use an enum to let a vector essentially hold different types
    // It has a i32, f64, and string but each of those are held in a variant
    // that are of type SpreadsheetCell
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row)
}
