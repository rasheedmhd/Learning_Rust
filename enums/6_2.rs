// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 6 - ENUMS AND PATTERN MATCHING

// === 6.2 - The match Control Flow Construct

// Rust has an extremely powerful control flow construct, that allows us to compare
// values to patterns and execute code when the pattern is matched

// match is very similar to if except a differetn syntax and that if must always
// return a boolean but match can return any value/expression.
#[derive(Debug)] // ?? state inspection

enum UsState {
    Alabama,
    Alaska,
}

enum Coin { // creating an enum to group variants of a coin
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // modified to bind another enum to its value
}

fn value_in_cents(coin: Coin) -> u8 { // passing in a variable of data type Coin
    match coin {
        Coin::Penny => 1, // a pattern arm and the code to execute
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// arms are seperated by ',' except when the code is multiline that we use
// {} then ',' becomes optional

// Patterns that Bind to Values


fn main() {
    // Matching with Option<T>
    // a function that works on Option<T>
    // it takes in Option<T> type as a parameter and returns an Option<T> type,
    // when Option<T> returns a value, the Some variant, it adds one to it else
    // if Option<T> returns None, it does Nothing

    fn plus_un(optn: Option<i32>) -> Option<i32> {
        match optn {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5); // binds the value of five to Some(5) and Option<T> variant
    let six = plus_un(five); // Pass Some(5) to the function to be incremented by 1
    let none = plus_un(None); // Pass None to the function so nothing happens here

    // Matches Are Exhaustive
    fn plus_un(optn: Option<i32>) -> Option<i32> {
        match optn {
            Some(i) => Some(i+1)
            // this won't compile because we didn't handle None
            // hence this code can be said to be not exhaustive
        };
    }

    // Catch-all Patterns and the _ Placeholder
    // a catch all pattern does what it says, it catches all
    // it is a special pattern that matches any value without binding to the value
    // it is denoted by '_'
    // () = unit tuple, used when we don't want to execute any code.


}







