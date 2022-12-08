// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 7 - Managing Growing Projects with Packages, Crates, and Modules

// === 7.1 - Packages and Crates
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}