// VARIABLES
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//CONSTANTS
//const VALUE_OF_PI: u32 = 3.14;
// 1. are immutable data types
// 2. cannot be made mutable with the 'mut' keyword
// 3. convention is to WRITE_IN_CAPS_WITH_UNDERSCORES
// 4. start with the 'const' keyword
// 5. must be type annotated with ':'
// 6. must be bounded before runtime
// 7. binding value can be an expression like 60*60*3


// SHADOWING
fn main() {
    let x = 5;
    let x = x +1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

// shadowing is used to change a variable's type
// shadowing is different from making a variable mutable
// in shadowing you use the 'let' keyword but in mutating,
// you just rebind the variable name to a new value
// 'like x = 8;' instead of 'let x = 8;'

// DATA TYPES
// rust has two types of data types
// 1. scalar => integers, booleans, floating-point numbers and characters
// 2. compound
// INTEGERS default to i32
fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");
}

// FLOATING POINTS
// are f64 for 64bits and f32 for 32 bits - both unsigned
// floating points default to f64 bc in modern os they are
// as fast as f32 and supports more precision
// supports type annotation. let y: f32 = 3.0; 32bits
 // let x = 1.0; defaults to f64, 64bits

// NUMERIC OPERATIONS
// includes
// 1. addition
// 2. substraction
// 3. multiplication
// 4. division
// 5. remainder

//BOOLEAN TYPE (bool) 1 byte in size => true or false
// supports type annotation let status: bool = true;

// CHARACTERS => 4 bytes in size
fn main() {
    let c = 'z';
    let c: char = 'ℤ'; // supports type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, heart_eyed_cat: {heart_eyed_cat}");
}

// COMPOUND DATA TYPES => combines several different data types into 1.
// rust has 2 types: tuples and arrays

// TUPLES => container that holds distinct values of distinct types
// supports type annotation
// tuples are fixed in size, once created they cannot be increased
fn main() {
    let tup = (3, 5.03242, 1);
   //let tup: (u32, f32, u8) = (3, 5.4, 1);
    let (a, b, c) = tup;
    let three = tup.0; //using dot notation for indexing on tuples
    let one = tup.2;
    println!("The value of b is: {b}");
    println!("The values of three and one are {three}, {one}");
}

// ARRAYS => supports type annotation
// are fixed in size
// unlike in tuples, elements in a tuple must all be of the same type
// arrays are stored in the stack since they are fixed
// uses square brackets for defining, binding and accessing elements
// [indexing]
fn main() {
    let the_boys = ["fatau", "nash", "dan", "starlet"];
    //let the_boys: [u32; 4] = [1,2,3,4]; type annotated
    let repeat = ["A"; 3];
    let sharks = the_boys[0];
    let bitperse = the_boys[1];
    let top_ten = the_boys[2];
    let capital = the_boys[3];
    println!("repeat: {}", repeat[0]);
    println!("In the boys, sharks is {sharks}, bitperse is {bitperse}, top ten is {top_ten} and starlet capital is {capital}");
}

use std::io;

fn main() {
    let a = [1,2,3,4,5];

    println!("Please input an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse().
        expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}