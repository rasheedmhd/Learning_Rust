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

    // Ownership and HashMaps
    let key = String::from("Purple");
    let value = 12;

    let mut scores = HashMap::new();
    // If we insert a key and a value into a hash map and then
    // insert that same key with a different value, the value
    // associated with that key will be replaced.
    // This means that the key's type will change to the type
    // of the new key if it were a different type.
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert
    scores.insert(key, value);
    // key is invalid at this point,
    // scores takes ownership of it.
    // value however is a scaler value
    // and implements the Copy Trait
    // therefore scores doesn't take
    // ownership of it but rather copies it.
    println!("{key}: {value}");

    // Assuming we wanted to add a
    // key value pair into a HashMap
    // and didn't know whether the
    // key already existed or not
    // Or we wanted to see if there
    // is a value assoc with a key
    // already and if not we assign
    // a new value to the key
    scores.entry("Blue").or_insert(56);
    scores.entry("Blu").or_insert(56);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // Rewriting the for loop with a closure
    for word in text.split_whitespace() {
        map.entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    println!("{:?}", map);
}
