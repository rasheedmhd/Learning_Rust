use crate::List::{ Cons, Nil };
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil
}

//enum List<T> {
//    Cons(T, List<T>),
//    Nil
//}

// Defining out own Smart Pointers
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
//    let b = Box::new(5);
    let b = MyBox::new(5);
    println!("b holds: {:#?}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //15.2. Treating Smart Pointers Like Regular References with the Deref Trait
    let x = 5;
    // let y = &x;
    // Using Box<T> Like a Reference
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);


}