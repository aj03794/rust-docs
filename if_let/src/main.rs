// if let syntax lets you combine if and let into a less verboe way
// to handle values that match one pattern while ignoring the rest

fn main() {

    let some_u8_value = Some(3u8);
    // let some_u8_value = Some(0u8);

    // **OPTION 1**
    
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => ()
    // }

    // **OPTION 2**

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not found")
    }
    
}
