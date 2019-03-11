fn main() {
    // creating a new, empty string
    let s = String::new();

    // this is same as line 9?
    // This is a &static str type
    let data = "initial contents";
    println!("data: {}", data);
    let data_two = String::from("initial contents");

    let s = data.to_string();
    println!("s: {}", s);
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let mut a = String::from("foo");
    a.push_str("bar");
    println!("a: {}", a);

    // **NOTE**
    // q does not take ownership of w from that "push_str"
    // we know this because we can still print w after we push it into q
    let mut q = String::from("lo");
    let w = "l";
    q.push_str(w);
    println!("w is {}", w);
    println!("q: {}", q);

    // **Concatenation with the + operator or the format! Macro**

    // Can only add &str with String
    // So why does htis work? &r is of type &String not &str
    // We're able to because the compiler can "coerce" the &String
    // into a &str
    let e = String::from("Hello, ");
    let r = String::from("world!");
    // can't add r, must used reference to r (&r)
    // can't use &e, must be e
    let t = e + &r; // e has been  moved here and can no longer be used
    // can't use e anymore
    // println!("e: {}", e);
    println!("r: {}", r);
    println!("t: {}", t);

    let y = String::from("tic");
    let u = String::from("tac");
    let i = String::from("toe");
    // Format takes no ownership of parameters
    // lines 49 and 50 are the same - 49 is just easier to read
    let z = format!("{}-{}-{}", y, u, i);
    let p = y + "-" + &u + "-" + &i;
    println!("p: {}", p);
    println!("z: {}", z);

}