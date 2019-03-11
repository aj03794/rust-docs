use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Only prints { "Blue": 25 }
    // Original value of 10 has been overwritten
    println!("scores: {:?}", scores);


    // **Only inserting a Value If the Key Has No Value

    let mut scoresTwo = HashMap::new();
    scoresTwo.insert(String::from("Blue"), 10);

    scoresTwo.entry(String::from("Yellow")).or_insert(50);
    scoresTwo.entry(String::from("Blue")).or_insert(50);

    // prints: "scoresTwo: {"Blue": 10, "Yellow": 50}"
    println!("scoresTwo: {:?}", scoresTwo);


    // **Updating a Value Based on the Old Value**

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // the "or_insert" method actually returns a mutable reference
        // (*mut V) for the value of this key
        // Here we store that mutable reference in the count variable
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    //map: {"world": 2, "wonderful": 1, "hello": 1}
    println!("map: {:?}", map);
}
