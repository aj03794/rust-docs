use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Needed here because it's possible to collect into many different
    // data structures and Rust doesn't know which you want unless you specify
    let scoresTwo: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scoresTwo: {:?}", scoresTwo);

    // **Hash Maps and Ownership**
    // For types that implement the "Copy" trait, like i32, the values are
    // copied into the hash map
    // For owned values like String, the values will be moved and the hash map
    // will be the owner of those values

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // can't use field_name and field_value anymore
    map.insert(field_name, field_value);

    // can do this if we want to use the field_value again
    // We can't do this for field_name because it is a string
    // however, the values that the references point to must be valid for
    // at least as long as the hash map is valid
    // map.insert(field_name, &field_value);
    // println!("field_name: {}", field_value);

    // **Accessing Values in a Hash Map**

    let mut scoresThree = HashMap::new();

    scoresThree.insert(String::from("Blue"), 10);
    scoresThree.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let scoresThree = scores.get(&team_name);
    // Result is Some(10) which is really Some(&10) b/c of the reference
    // Wrappin gin SOme b/c get returns an Option<&V>
    println!("scoresThree: {:?}", scoresThree);

    // **Iterating over hash maps**
    let mut scoresFour = HashMap::new();

    scoresFour.insert(String::from("Blue"), 10);
    scoresFour.insert(String::from("Yellow"), 50);

    // These pairs get printed in an arbitrary order
    for (key, value) in &scoresFour {
        println!("{}: {}", key, value);
    }
}


