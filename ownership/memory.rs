#[derive(Debug)]

pub struct Card {
    pub number: i64,
    pub suit: i64,
}

fn main() {

    let ace_of_spades = Card {
        number: 1,
        suit: 4,
    };

    let reface: *const i64 = ace_of_spades;

    println!("as_of_spades: {:#?}", ace_of_spades);

    println!("&ace_of_spades: {}", *reface);
//    println!("&ace_of_spades.suit: {:#?}", *ace_of_spades.suit);
//    println!("&ace_of_spades.number: {:#?}", *ace_of_spades.number);

    //let quintillionx18: i128 = 18_000_000_000_000_000_000;
    //let memory: Vec<u8> = vec![0; 18_000_000_000_000_000_000];

    //println!("abstract memory representation: {:#?}", memory);

    //    every program usually has its own memory
    //    sometimes not so, because OS design
    //    this is provided by virtualization
    //    virtual memory is usually larger that physical memory
    //    the OS manages mappings between virtual memory and physical memory
    //
    //    A program can request a memory from the OS, but it can specifically
    //    choose the address it wants to be allocated to it, doing that will
    //    probably end in pointing to unallocated memory,
}