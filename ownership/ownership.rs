#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#[derive(Debug)]


pub struct Card {
    pub number: i64,
    pub suit: i64,
}

fn main() {

    let s1: String = String::from("Hello World");
    let s1 = String::from("Hello World");
    let s2 = 17;

    println!("s1: {}", s1);
    println!("s2: {}", 17);

    // Taking and returning ownership
    // this is an anti-pattern if we don't want the ownership, then we shouldn't take printlnfn main() {
    let mut ace_of_spades = Card {
        number: 1,
        suit: 1,
    };

    ace_of_spades = print_card(ace_of_spades);

    fn print_card(card: Card) -> Card {
        println!("Number: {} Suit: {}", card.number, card.suit);
        return card;
    }

// FAILS TO PASS THE BORROW CHECKER
    let mut v = vec![1, 2];
    let one = &mut v[0];
    let two = &mut v[1];
    *two += *one;

    let mut v = vec![1, 2];
    let mut iter = v.iter_mut();
    let one = iter.next().unwrap();
    let two = iter.next().unwrap();
    *two += *one;

    println!("one: {}, two: {}", one, two);

    //create_next_card(&ace_of_spades);

}

// we can have data that we are sharing with others in rust
// because we are sharing with others, (&) we cannot change
// that data(this prevents data races) but we can make a copy
// of the data and change parts of it or all of it.

//fn create_next_card(card: &Card) -> Card {
//    let mut next_card = card.clone();
//    next_card.number += 11;
//    next_card
//}

// COPY
//rust copy is a byte by byte copy of memory
//moving ownership from one to another sometmes
//copy only works on data types that implements the Copy trait
//copying from shared references is okay but copying from pointers
//that point to mutable data can cause double frees and rust prevents this