fn main() {
    // **Indexing into Strings**
    // Rust doesn't support indexing strings
    // But why doesn't it?

    // let d = String::from("hello");
    // let f = d[0];
    // println!("f: {}", f);

    // ***Internal Representation***
    // A string is a wrapper over a Vec<u8>
    let len = String::from("Hola").len();
    println!("len: {}", len);
    //len is 4 which means the vector storing the string "Hola" is
    // 4 bytes long
    // Each of these letters is 1 byte when encoded in UTF-8

    let lenTwo = String::from("Здравствуйте").len();
    println!("lenTwo: {}", lenTwo);
    // this is 24
    // Each unicode scalar value in that string takes 2 bytes of storage
    // Therefore an index into the string's bytes will not always correlate
    // to a valid Unicode scalar value

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    // What should the value of answer be?
    // Should it be 3?
    // When encoded in UTF-8, the first byte of 3 is 208
    // The second is 151
    // So answer should in fact be 208, but 208 is not a valid character
    // on its own, however that's the only data that Rust has at byte index 0
    // Users generally don't want the byte value returned

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // Prints 65 66 67 (on different lines)
    // These are the utf-8 decimal values of A, B, and C respectively
    for c in "ABC".bytes() {
        println!("{}", c);
    }
}
