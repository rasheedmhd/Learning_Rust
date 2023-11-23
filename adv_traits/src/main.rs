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


fn main() {
    println!("Hello, world!");

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
