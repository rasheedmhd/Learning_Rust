// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 11 - WRITING AUTOMATED TESTS

// === 11.1 - HOW TO WRITE TESTS

// TESTs are rust functions that verify that the non-test code is functioning as expected

/// WHAT TESTS DO
// 1. Set up the needed data or state
// 2. Run the code to be tested
// 3. Assert the results are what you expected

// tests files can have functions that aren't test functions


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// checking panics with should_panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        //        if value < 1 || value > 100 {
        //            panic!("Guess value must be between 1 and 100, got {}", value);
        //        }
        if value < 1 {
            panic!(
                    "Guess value must be less than or equal to 100, got {}.",
            value
            );
        } else if value > 100 {
            panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
            value
            );
        }

        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    //format!("Hello {}!", name)
    String::from("Hello!")
}


// assert! macro
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // using Result<T, E> in Tests
    // you can't use #[should_panic] on test that use Result<T, E>
    // using Result<T, E> allows us to bubble up errors using the '?' operator
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 22 == 4 {
            Ok(())
        } else {
            Err(String::from("Un plu deux n'est pas egal a quatre"))
        }
    }

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
        Guess::new(20066789);
    }

#[test]
// ADDING CUSTOM ERROR MESSAGES
fn greeting_contains_name() {
        let result = greeting("zaeskegen");
        assert!(
                result.contains("Carol"),
        "GREETING DID NOT CONTAIN  NAME, VALUE WAS `{}`",
        result
        );
    }

#[test]
fn it_adds_two() {
        // assert_eq!(7, add_two(7)); -> fails bc 7 is not equal to 7+2
    assert_eq!(7, add_two(5));
        // assert_ne!(7, add_two(5));
    }

#[test]
fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

#[test]
fn smaller_cannot_hold_larger() {
        //use super::*;

    let smaller = Rectangle {
            width: 23,
        height: 17,
    };

        let larger = Rectangle {
            width: 27,
            height: 24,
        };

        assert!(!smaller.can_hold(&larger));
    }

#[test]
fn exploration() {
        //let result = add(2, 2);
    assert_eq!(2 + 2, 4);
    }

// Each test runs on a new thread,
// when the new thread sees that a thread has died, it marks it as failed
#[test]
fn another() {
        panic!("Make this test fail");
    }

}








