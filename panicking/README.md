#### To Panic! or Not to Panic!

- When code panics, there's no way to recover
- Could call `panic!` in any error situation, whether there's a possible way to recover or not, bu tthen you're making the decision on behalf of the code calling your code that a situation is unrecoverable
- When you choose to return a `Result` value, you give the calling code options rather than making the decision for it
- Reeturning `Result` is a good default choice when you're defining a function that might

- `unwrap` and `expect` methods are very handy when prototyping before you're ready to device how to handle errors
- They leave clear markers in your cod for when you're ready to make your program more robust

- It would also be appropriate to call `unwrap` whn you have some other logic that ensures the `Result` will still have an `Ok` value, but th elogic isn't something the compiler understands
- If you can ensure by manually inspecting the code that you'll never have an `Err` variant, it's okay to use `unwrap` and leave it

```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

- Creating an `IpAddr` instance by parsing a hardcoded string
- `127.0.0.1` is still a valid IP address, so it's acceptable to use `unwrap` here
- However, the return type of the `parse` method is still a `Result` and the compiler will make us handle the `Result` as if the `Err` variant is a possibility

#### Guidelines for Error Handling

- Advisable to have your code panic when it's possible that your code could end up in a bad state

- The bad state is not something that's *expected* to happen occasionally
- Your code after this point needs to rely on not being in this bad state
- There's not a good way to encode this information in the types you use

- `panic!` is often appropriate if you're calling external code that is out of your control and it returns an invalid state that you have no way of fixing

- When a failure is expected, it is more appropriate to return a `Result` than to make a `panic!` call
- Ex: being given malformed data or an HTTP request
- `Result` indicates that failure is an expected possibility that the calling code must decide how to handle

- Functions often have *contracts*
- Behavior is only guaranteed if the inputs meet particular requirements
- Panicking when the contract is violated makes sense b/c a contract violation always indicates a caller-side bug and it's not a kind of error you want the calling code to have to explicitly handle
- The calling code's *programmers* need to fix the code