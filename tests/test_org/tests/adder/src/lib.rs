// TEST ORGANIZATION
// 1. UNIT TEST
// -> test unit code in isolation, to pinpoint where code is and isn't working as expected
// 2. INTEGRATION TEST
// -> are entirely external to your library
// -> test whether many parts of your code work together correctly

// Tells the compiler to only compile the code when we run cargo test, not in cargo build
//#[cfg(test)]

// TESTING PRIVATE FUNCTIONS
// rust allows us to test private FUNCTIONS, some PL's don't

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
