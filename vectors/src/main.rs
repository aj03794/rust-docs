fn main() {
    // When a specfic vector holds a specific type, the type is specified
    // the type is specified withi the angle brackets
    // We are sayint that the type here here will hold elements of i32 type
    let v: Vec<i32> = Vec::new();
    // Don't need the type annotation "Vec<i32>" here because compiler
    // can infer we want i32
    let x = vec![1, 2, 3];

    // Compiler is able to infer type because we push in i32 values below
    let mut a = Vec::new();
    a.push(5);
    a.push(6);
    a.push(7);
    a.push(8);

    let z = vec![1, 2, 3, 4, 5];
    // option 1 to get element
    let third = &z[2];
    println!("The third element is {}", third);
    // option 2 to get element
    match z.get(2) {
        Some(thirdElement) => println!("The third element is {}", thirdElement),
        None => println!("There is no third element."),
    }
    // using if let
    if let Some(thirdElement) = z.get(2) {
        println!("three");
    } else {
        println!("not found")
    }

    let q = vec![1, 2, 3, 4, 5];
    // References nonexistent element
    // let does_not_exist = &v[100];
    // ******When get method is passed an index that is outside the vector,
    // it returns "None" w/o panicking
    let does_not_exist = v.get(100);


    // **** DOESNT COMPILE ****
    // Cannot borrow "w" as mutable b/c it is also borrowed as immutable
    // This error is due to the way vectors work
    // Adding a new element onto the end of the vector might require
    // allocating new memory and copying the old elements to the new space
    // In that case, the reference to the first element would be pointing
    // deallocated memory
    // The borrowing rules prevent programs from ending up in that situation
    // let mut w = vec![1, 2, 3, 4, 5];
    // let first = &w[0];
    // w.push(6);
    // println!("The first element is: {}", first);

    let e = vec![100, 32, 57];
    for i in &e {
        println!("{}", i);
    }

    let mut r = vec![100, 32, 57];
    for i in &mut r {
        // * is the dereference operator
        // have to do this to do the value in "i" before so
        // we can use the "+="" operator
        *i += 50;
    }
    println!("r: {:?}", r);

} // <- a && x && v goes out of scope and is freed here
