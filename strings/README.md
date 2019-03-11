- Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text

##### What is a string?

- Only one string type in the `core language`, which is the string slice `str`
- `String slices` are references to some UTF-8 encoded sring data stored elsewhere
- String literals are stored in the binary output of the program and are therefore string slices

- The `String` type which is provided by Rust's standard library rather than coded into the core language, is a growable, mutabled, owned, UTF-8 encoded string type

- We can only add a `&str` to a `String`