// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 9 - ERROR HANDLING

// === 9.1 - Unrecoverable Erros with panic!
// Rust has 2 types of errors
// 1. Unrecoverable - array out of bounds access
// 2. Recoverable - file reading errors
// Rust doesn't have exceptions, it use the Result<T, E> to handle errors

// Rust unwinds when a panic occurs - cleaning the stack, leaving the memory as it was
// you can change this behaviour to get a smaller binary by setting panic to abort in cargo.toml
// [profile.release]
// panic = 'abort'

fn main() {
    let v = vec![1,2,3];
    v[99]; // this line tries to access index 99 of our vector
    // which only has 3 items, this will throw an out out of bounds error
    println!("Hello");
}