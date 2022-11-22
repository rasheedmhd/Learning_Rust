// a library crate that allows us to generate random
// numbers when inscope
//extern crate rand;
//standard library input/ouput functions
use std::io;
//the trait that defines how random numbers are generated
use rand::Rng;
//Ordering enum used with match
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess!");

    // generating a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);

    loop {
        //new empty string
        let mut guess = String::new();

        //handling user input
        io::stdin() //method
        .read_line(&mut guess) //function: returns the Result Enum
        .expect("Error reading line"); //Handling the Result Enum for errors

        //modifying one type (String in this case) to another
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // error handling
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}