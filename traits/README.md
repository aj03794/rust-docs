- A trait tells the Rust compiler about the functionality a particular type has and can share with other types
- Can use traits to defined shared behavior in an abstract way
- Use `trait bounds` to specify that a generic can be any type that has certain behavior

```
Traits are similar to interfaces in other languages, although with some differences
```

- A type's behavior consists of the methods we can call on that type
- Different types share the same behavior if we can call the same methods on all of those types
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose