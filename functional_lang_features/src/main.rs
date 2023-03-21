//// THE RUST PROGRAMMING LANGUAGE BOOK
//// CHAPTER 13 - FUNCTIONAL LANGUAGE FEATURES: Iterators and Closures
//// ===
///  13.1
//   Closures: Anonymous functions that capture their executing environment

// WHAT THE HECK IS FUNCTIONAL PROGRAMMING?
// Programming in functional style means doing several operations with functions
// Such operations include
//      1. Passing functions as arguments to other functions
//      2. Returning functions from othr functions
//      3. Assigning/binding functions to variables
//         and other operations

// ENTER CLOSURES
// They are anonymous functions
// 1. That capture their environment (values in the scope where they are defined)
// 2. Can be saved in variables
// 3. Can be passed as arguments to functions
// 4. Can be called outside of the context they were defined in to be evaluated

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
}
