enum Message {
    // No data
    Quit,
    // Anonymous struct
    Move { x: i32, y: i32 },
    // Single string
    Write(String),
    // Includes 3 i32 values
    ChangeColor(i32, i32, i32)
}

// Could do the above enum with 4 different structs
// Doing it this way though you can't define a function that takes
// any kind of these messages
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Defining a method on an enum
impl Message {
    fn call(&self) {
        // method body defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
