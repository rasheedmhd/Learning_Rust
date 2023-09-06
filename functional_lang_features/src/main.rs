//// THE RUST PROGRAMMING LANGUAGE BOOK
//// CHAPTER 13 - FUNCTIONAL LANGUAGE FEATURES: Iterators and Closures
//// ===
///  13.1
//   Closures: Anonymous functions that capture their executing environment

// WHAT THE HECK IS FUNCTIONAL PROGRAMMING?
// Programming in functional style means doing several operations with functions
// Such operations include
//      1. Passing functions as arguments to other functions
//      2. Returning functions from other functions
//      3. Assigning/binding functions to variables and other operations

// ENTER CLOSURES
// They are anonymous functions
// 1. That capture their environment (values in the scope where they are defined)
// 2. Can be saved in variables
// 3. Can be passed as arguments to functions
// 4. Can be called outside of the context they were defined in to be evaluated
use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let first_winner_pref = Some(ShirtColor::Red);
    let first_giveaway = store.giveaway(first_winner_pref);

    println!(
        "The user with preference {:?} gets {:?}",
        first_winner_pref, first_giveaway
    );

    let snd_winner_pref = None;
    let snd_giveaway = store.giveaway(snd_winner_pref);

    println!(
        "The user with preference {:?} gets {:?}",
        snd_winner_pref, snd_giveaway
    );

    // CLOSURE TYPE INFERENCE AND ANNOTATION
    // 1. Do not require type annotation for parameters and return values
    // 2. There are rate cases where the compiler need closure type annotations
    let expensive_closure = |num: u32| -> u32 {
        print!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v4 = |x| x + 1;
    // Doesn't compile the compiler demands explicit type annotations
    // Made a commit in an Issue in the book hoping to get a reply to rectify the issue
    let egp = |x| x;
    let s = egp(String::from("Hello"));
    println!("{s}");
    // let n = egp(5);
    // The first time we call a closure, the types of the parameter(s)
    // and return values are inferred and locked into the closure
    // therefore calling the closure later with a different type raise an error

    // CAPTURING REFERENCES OR MOVING OWNERSHIP
    // 3 ways closures capture their environment
    // 1. borrow immutably
    // 2. borrow mutably
    // 3. take ownership
    //
    // the closure determine which way to use based on what the body of the function
    // does with the captured values

    // let mut list = vec![1, 3, 4];
    let list = vec![1, 3, 4];
    println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);
    // let mut mutable_borrow = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // mutable_borrow();

    // println!("After calling closure: {:?}", list);

    // Moving ownership
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("After calling closure: {:?}", list);\
    //
    //
    //
    // MOVING CAPTURED VALUES OUT OF CLOSURES AND THE Fn TRAITS
    // Things closure bodies do during later closure evaluation
    // 1. Capture values in the environment
    // 2. Move captured values out of the closure
    // 3. Mutate captured values
    // 4. Not mutating or moving a value
    // 5. Not capturing in the first place
    //
    //  this determines the traits the closure implements
    //  traits are how functions and structs specifies the kind of closures they can use
    //
    // FnOnce -> moves captured values outside of the closure therefore called only once
    //           implemented exclusively
    // FnMut -> doesn't move values out, might mutate values, called more than once
    // Fn -> (pure functional closures) doesn't mutate values, doesn't move values out
    //          might not capture values from environment, can be called multiple times concurrently
    //
    //
    // Looking at unwrap_or_else
    // implemented on a Option which is generic over any type
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T,
    //     {
    //         match self {
    //             // returns the value of the Some variant of Option
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    // LOOKING AT std lib METHOD sort_by_key
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // list.sort_by_key(|r| r.width);
    // let mut sort_ops = vec![];
    // let value = String::from("by key called");
    let mut num_sort_ops = 0;

    // list.sort_by_key(|r| {
    //     // sort_ops.push(value.clone());
    //     sort_ops.push(value);
    //     r.width
    // });
    //
    //
    list.sort_by_key(|r| {
        num_sort_ops += 1;
        r.width
    });
    println!("{:#?} sorted in {num_sort_ops} operations ", list);
}
