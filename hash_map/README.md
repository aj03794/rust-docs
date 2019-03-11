- `HashMap<K, >` stores a mapping of keys of type `K` and of type `V`
- It does this via a *hashing function* which determines how it places these keys and values into memory

- Useful when you want to look up data not only by using an index, as you can with vectors, but by using a key that can be of any type

- Store their data on the heap
- Hash maps are homogenous: all of the keys must have the same type, and all of the values must have the same type

- Each key can only have one value associated with it at a time

- `HashMap` usses a cryptographically strong hashing function that can provide resistance to DoS attacks