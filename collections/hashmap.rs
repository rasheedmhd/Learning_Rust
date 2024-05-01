use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // HashMaps are homogeneous
    // all keys = same type
    // all values = same type
    scores.insert("Blue", 11);
    scores.insert("Red", 151);
}
