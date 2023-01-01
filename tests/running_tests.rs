// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 11 - WRITING AUTOMATED TESTS

// === 11.2 - CONTROLLING HOW TESTS WORKS

const JUPITER_RADIUS: i32 = 69_911;

// cargo test compiles and runs the test code as one binary
// eact test in run on parallely on its own thread, and rust captures the output
// you can pass command line arg to control the default behaviour

// RUNNING TESTS IN PARALLEL || CONSECUTIVELY
// - TEST are run in parallel by default as such it is important to not let
// - them depend on each other or any shared state
//
// tells the compiler to run the tests in 1 thread
// cargo test -- --test-threads=1
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
// this prints out the output of the functions when the test passes
// cargo test -- --show-output


// RUNNING A SUBSET OF TEST BY NAME
// cargo test <test_name>
// cargo test deduct_two

// RUNNING TEST BY FILTERING
// cargo test [..add]
// cargo test add

// You can run all the test using only the module NAME
// cargo test tests

pub fn deduct_two(a: i32) -> i32 {
    a - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ignoring Some Tests Unless Specifically Requested
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

#[test]
#[ignore]
fn radius_of_Kepler44() -> float {
        // code that takes an hour to run
        // --snip--
        //let radius = 1.09 * JUPITER_RADIUS;
        //radius
    }

// RUN ALL TEST REGARDLESS
// cargo test -- --include-ignored



#[test]
fn deduct_two_() {
        assert_eq!(4, deduct_two(6));
    }

#[test]
fn deduct_three() {
        assert_eq!(5, deduct_two(8));
    }

#[test]
fn one_hundred() {
        assert_eq!(deduct_two(104), 102)
    }

#[test]
fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

#[test]
fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value)
    }
}