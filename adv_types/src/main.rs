type kilometers = i32;

// Use CASES

type thunk = Box<dyn Fn() + Send + Sync + 'static>;

fn takes_thunk(t: thunk ) {}
// Instead of 
fn takes_not_thunk(t: Box<dyn Fn() + Send + Sync + 'static>) {}

// The Never type
// A type that has no value,
// never returns a value
// When used as a return value, it never does hence the name Never type
// Read as function return_never returns never or never returns
// Useful is building OS main functions
fn return_never() -> ! {}

// DYNAMICALLY SIZED TYPES
// golden rule -> always put them behind a sort of pointer
// Rust must know the size of every type at compile time
// so every value of the type uses the same size

// Rust allows for types of dynamic sizes
// types who sizes are only known at runtime
// trait bounds are dynamically sized
// that is why we put them behind ref types like box
// Box<dyn trait>, Box<&dyn trait>

// Rust implicitly size generic functions/types
// therefore their sizes are known at compile time
fn gt<T>(t: T)
// is actually 
fn gt<T: Sized>(t: T)

// To relax the implicit sizing Rust provides us with ?Sized 
fn gdt<T: ?Sized>(t: T) // meaning T can have a size or not.

fn main() {
    let x: i32 = 100;
    let y: kilometers = 200;
    println!("{y}");
    let f: Box<dyn Fn() + Send + Sync + 'static> = Box::new(|| println!("hi"));
}
