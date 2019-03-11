use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // hread 'main' panicked at 'index out of bounds: 
    // the len is 3 but the index is 99
    // in other languages like C, it will attempt to give you exactly
    // what you asked for in this situation, even tough it isn't
    // what you want
    // This is called a "buffer overread"

    // 12: error_handling::main
    // at src/main.rs:14
    // v[99];

    // **Recoverable Results with Result

    // We know File::open results a Result b/c of docs and compiler will
    // also tell us

    // FROM COMPILER
    // note: expected type `u32`
    // found type `std::result::Result<std::fs::File, std::io::Error>`
    // This tells us the return type of the File::open function is a
    // Result<T, E>
    // The generic parameter T has been filled in here with the type of the
    // success value, std::fs::File
    // The type of E used in the error value is std::io::Error
    // let f: u32 = File::open("hello.txt");

    let f = File::open("hello.txt");

    // thread 'main' panicked at 'There was a problem opening the file Os 
    //  code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:41:13
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file {:?}", error);
    //     }
    // };

    // **Matching on Different Errors**

    // Code above will `panic!` no matter why `File::open` failed

    // let f = match f {
    //     Ok(file) => file,
    //     // This returns an io::Error which is a struct provided by std lib
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
    //     }
    // };

    // shortcut instead of using match
    // If the Result value is the Ok varaint, unwrap will return teh value inside the Ok
    // If the Result is the Error variant, unwrap will call the panic! macro for us
    // let z = File::open("hello.txt").unwrap();

    // **expect**
    // Can use expect in the same way as unwrap
    // We can control error message in this case
    // If we use unwrap in multiple places, it can take more time to figure out
    // exaclty which unwarp is causing the panic
    // let x = File::open("hello.txt").expect("Failed to open hello.txt");

    // **Propogating errors**

    // When you're writing a function whose implementation calls something
    // that might fail, instead of handling the error within this function,
    // you can return the error to the calling code so that it can decide what to do
    // This is know as "propagating" the error and gives more control
    // to the calling code, where there might be more information or
    // logic that dictates how to handle the error

    // Reads a username from a file
    // If the file doesn't exist or can't be read
    // this function will return thsoe errors to the code that
    // called this function

    // This function is returning a value of type Result<T, E>
    // Generic T is fulfilled with String
    // E is fulfilled with io::Error
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            // Return here b/c there is nothing else we can do
            Err(e) => return Err(e)
        };

        let mut s = String::new();

        // Don't need to return here because it is the last thing to run
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e)
        }
    }

}
