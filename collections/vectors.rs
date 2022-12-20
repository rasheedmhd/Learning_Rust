// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 8 - Common Collections

// === 8.1 - Storing Lists of Values with Vectors

// VECTORS
// vectors allows you to store more than one value of the same type side by side
// in memory in one data structure.

// Definding a vector
fn main() {
    let v: Vec<i32> = Vec::new();
    // notice the type annotation above
    // it is required when creating an empty vector since RUST
    // can't infer to know the data type of the values
    // vec! a macro that creates a new vector with the values given to it
    let v2: Vec<String> = Vec::new();
    let v2 = vec!["Antman", "Captain America", "Black Panther"];
    // we have 2 v2, notice the shadowing?

    // In updating a vector, we use .push()
    // but the Vector signature must be made mutable with the 'mut' keyword
    let mut v = Vec::new();
    v.push(5);
    v.push(8);
    v.push(7);
    // here the numbers we put inside v are all i32, rust is able to infer

    //------------------------------
    // Reading Elements of Vectors
    //------------------------------
    // 1. Indexing
    // 2. get keyword

    // rust behaves differently with the above methods
    // 1. if we try accesing an out of scope index, the program will panic
    // 2. returns an Option type, if we access an out of scope index it returns a None
    // when using the get() method, we can define code to Some(T) and None

    let v = vec![1,2,3,4,5];

    let does_not_exist = &v[100]; // panics
    let does_not_exist = v.get(100); // returns None

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // this won't work because of the way vectors are stored in memory
    // if there is no space to store the new added value, rust will
    // have to allocate a new memory and copy all the vector
    // essentially leaving the reference pointing to
    // dellocated memory so rust prevents this bug

    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first );


    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }

    //Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

