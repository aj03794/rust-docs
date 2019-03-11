- Rust requires you to acknowledgeg the possibility of an error and take some action before your code will compile

- `Recoverable` and `unrecoverable` errors
- For a recoverable error (such as a file not found error), it's reasonable to report the problem to the user and retry
- Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array

- Rust doesn't have exceptions
- Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macros that stops execution when the program encounters an unrecoverable error


##### Unrecoverable Errors with panic!

- When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit

- By default, when a panic occurs, the program starts `unwinding`, which means Rust walks back up the stack and cleans up the data from each function it encounters
- This walking back and cleanup is a lot of work
- Alternative is to immediately `abort` which ends the program w/o cleaning up
- Memory that the program was using wll then need to be cleaned up by OS

Can add `panic = 'abort'` to the `[profile]` section in your `Cargo.toml` file

##### Recoverable Errors with Result

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- The `T` and `E` are generic type parameters
- `T` represents the type of the value that will be returned in the success case within the `Ok` variant
- The `E` represents the type of the error that will be returned in teh failure case within the `Err` variant