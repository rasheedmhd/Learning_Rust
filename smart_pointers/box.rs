use crate::List::{ Cons, Nil };

enum List {
    Cons(i32, Box<List>),
    Nil
}

//enum List<T> {
//    Cons(T, List<T>),
//    Nil
//}

fn main() {
    let b = Box::new(5);
    println!("b holds: {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}