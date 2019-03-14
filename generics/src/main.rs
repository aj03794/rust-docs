// We can use generics to create defintions for items like function signatures
// or structs, which we can then use with many diff concrete data types

// ***In Function Definitions**
// Place generics in the signature of the function where we would usually
// specify the data types of the parameters and the return value

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Read this function as:
// the function "largest" is generic over some type "T"
// This function has one parameter named "list", which is a slice of values
// of type "T"
// The largest function with return a value of the same type "T"
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// ***In Struct Defintions**
// x and y have to be of the same type T
struct Point<T> {
    x: T,
    y: T
}
struct PointTwo<T, U> {
    x: T,
    y: U
}

// ***In Enum Definitions***
// This is an enum that is a generic over type T and has two variants
// "Some" which holds one value of type "T"
// "None" variant that doesn't hold any value
// By using the Option enum, we can express the abstract concept of having
// an optional value
enum Option<T> {
    Some(T),
    None
}

// Enums can use multiple generic types as well
// Result enum is generic over two types, "T" and "E"
// It has two variants: "Ok" which holds a value of type "T"
// and "Err" which holds a value of type "E"
enum Result <T, E> {
    Ok(T),
    Err(E)
}

// ***In Method Definitions***
// Have to declare "T" just after "impl" so we can use it to specify that
// we're implementing methods on the type Point<T>
// By declaring "T" as a generic type after "impl", Rust can identify that
// the type in the angle brackets in "Point" is a generic type and not
// a concrete type
impl<T> Point <T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// This is the same as the above
// impl<U> Point <U> {
//     fn x(&self) -> &U {
//         &self.x
//     }
// }

// We could remove the "T" after "impl" by using a concrete type like the below
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer = PointTwo { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // Won't work because x and y are different types
    // let wont_work = Point { x: 5, y: 4.0 }
    // Now x and y can be of different types or the same type
    let will_work = PointTwo { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("=======");
    println!("p.x = {}", p.x());
}
