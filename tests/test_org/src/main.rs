// TEST ORGANIZATION
// 1. UNIT TEST
// -> test unit code in isolation, to pinpoint where code is and isn't working as expected
// 2. INTEGRATION TEST
// -> are entirely external to your library
// -> test whether many parts of your code work together correctly
// if a unit tests fails, doc tests and integration tests won't run
// if any tests in any section (ie unit, integration, doc) tests fails
// the other sections won't run

// running specific integration test
// cargo test --test integration_test

// INTEGRATION TESTS FOR BINARY CRATES
// we can't create tests folder and call in functions defined in
// (src/main.rs) into scope wiht 'use' to use for integration TESTING
// only libray crates expose functions that other crates can use
// if our crate is a binary crate not library, binary crates are meant to be executed on their own

// Tells the compiler to only compile the code when we run cargo test, not in cargo build
//#[cfg(test)]

// TESTING PRIVATE FUNCTIONS
// rust allows us to test private FUNCTIONS, some PL's don't

//

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2,2));
    }
}


fn main() {
    println!("Hello, world!");
}
