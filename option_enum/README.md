- `Option` - enum defined by std library
- Handles very common scenario in which a value could be something or it could be nothing
- Expressing this concept in terms of the type system means the compiler can check whether you've handled all the cases you should be handling

- Rust doesn't have the null feature

- `Option<T>`

- `<T>` is the generic type parameter
- `<T>` means the `Some` variant of the `Option` enum can hold one piece of data of any type

- In order to use a `Option<T>` value, you want to have code that will handle each variant
- You want some code that will run only when you have `Some(T)` value, and this cide is allowed to use the inner `T`
- You want some other code to run if you have a `None` value, and that code doesn't have a `T` value available
- `Match` construct is useful for handling all the variants of an enum just like in this case