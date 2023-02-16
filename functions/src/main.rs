//FUNCTIONS
// rust use the "fn" keyword to define functions
// functions naming convention follows snake_case_just_like_this_text
// when passing in parameters, you must declare the type of the argument

const TIME_REVERSAL_LIMIT: i32 = 12*11+2; //imaginary constant that I created

fn main() {
    println!("Hello, world!");
    another_function();
    parametised_function(TIME_REVERSAL_LIMIT);
}

fn another_function() {
    println!("Another Function!");
}

//a function that takes in a parameter
fn parametised_function(x: i32) {
    println!("The argument passed as x is: {x}");
}

// STATEMENTS VRS EXPRESSIONS
// unlike in most other languages STATEMENTS and EXPRESSIONS
// are 2 distinct things in rust
// STATEMENTS do not return or evaluate to a value but EXPRESSIONS does
// STATEMENTS includes
// define a function, binding a variable,
// EXPRESSIONS includes
// calling a function, macro, creating an empty scope with brackets
// EXPRESSIONS can be part of STATEMENTS
// EXPRESSIONS do not end in ; I repeat EXPRESSIONS, DO NOT END in ';'
// if you end an EXPRESSIONS with a ';' it will become a STATEMENT
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

//FUNCTIONS THAT RETURNS A VALUE

fn five() -> i32 {
    let b = 11;
    b + 5 + TIME_REVERSAL_LIMIT // no ';'
    // using constants to play with expressions
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}

fn main() {
    let x = plust_one(5); // I have seen the t lol
    println!("The value of x is: {x}");
}

fn plust_one(x: i32) -> i32 { // here we are type annotating the function
    x + 1 // You realise there is no ';' right? Good
    // it we put ';' the expression becomes a STATEMENT
    // and the code wouldn't compile
}