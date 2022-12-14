// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 9 - ERROR HANDLING

// === 9.3 - TO panic! or Not to panic!

// You can decide to use panic to handle both errors, recoverable and non recoverable
// doing that is situation where the error can be recovered, you are making a decision for the calling code
// that the error can't be recovered which is not something we might want in production
// it would be better if you return an error to calling code and allowed it to decide

// Examples, Prototype Code, and Tests

// unwrap() and expect() are good when prototyping since you wouldn't want robust error handling code
// to obscure the codes meaning. When you are ready to relese then you handle errors appropriately

// Creating Custom Types for Validation

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 { self.value } //getter
    // gets value from its (private)fields and returns it
}













