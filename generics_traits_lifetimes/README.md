- `Generics` are abstract stand-ins for concerete types or other properties

- Similar to how the way a function takes parameters w/o unknown values to run the same code on multiple concerete values, functions can take parameters of some generic type instead of a concerete type (like `i32` or `String`)
- `Option<T>` is an example of a generic
- `Vec<T>` and `HashMap<K, V>` are other examples

- You can combine `traits` with generic types to constrain a generic type to only those types that have a particular behavior, as opposted to just any type

- `Lifetimes` are a variety of generics that give the compiler information about how references relate to each other
- Allow us to borrow values in many situations