use std::ops::Add;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Iterator in generics
pub trait Interator<T> {
    fn next(&mut self) -> Option<T>;
}
// but this leads to multiple trait implementations
// impl Iterator<String> for Counter
// impl Iterator<i32> for Counter
// impl Iterator<f64> for Counter
// etc 
struct Counter;

// impl Iterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         // todo()!
//     }
// }

// Default Generic Type Parameters and Operator Overloading
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point ) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


trait Animal {
    fn baby_name() -> String;
    fn movement() -> String {
        String::from("f*&$ker just moves")
    }
}

struct Dog;
struct Fish;

impl Fish {
    fn baby_name() -> String {
        String::from("Mawu")
    }
    fn movement() -> String {
        String::from("I move by swimming in the water")
    }
}

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
    fn movement() -> String {
        String::from("I move by walking and running on land")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
    fn movement() -> String {
        String::from("I move by walking and running")
    }
}

impl Animal for Fish {
    fn baby_name() -> String {
        String::from("Asma")
    }
    fn movement() -> String {
        String::from("I move by swimming")
    }
}


fn main() {
    println!("Hello, world!");

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // Rust defaults to the method impl on the Human struct 
    person.fly();

    // Using Fully Qualified Syntax 
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::movement());
    println!("A baby dog is called a {}", <Dog as Animal>::movement());
    println!("A baby fish is called a {}", Fish::movement());
    println!("A baby fish is called a {}", <Fish as Animal>::movement());
    // println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // println!("A baby fish is called a {}", Fish::baby_name());
    // println!("A baby fish is called a {}", <Fish as Animal>::baby_name());
}
