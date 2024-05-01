use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // HashMaps are homogeneous
    // all keys = same type
    // all values = same type
    scores.insert("Blue", 11);
    scores.insert("Red", 151);

    // Getting values from a HashMap using their keys
    // .get() -> Option(Value), if the key doesn't exist
    // we gracefully handle it by returning 0 [.unwrap_or(0)]
    let blue_score = scores.get("Blue").copied().unwrap_or(0);

    println!(" blue score: {:#?} ", blue_score);

    // looping through a HashMap to print value pairs
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
