// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }


// fn five() -> i32 {
//     // return 5
//     5
// }

// fn main() {
//     // This is same as let x = 5;
//     let x = five();

//     println!("The value of x is: {}", x);
// }


fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 2147483643
}
