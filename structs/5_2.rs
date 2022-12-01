// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 5 - STRUCTS

// === 5.2 - An Example Program Using Structs
#[derive(Debug)] //formatting trait, needed to display structs
// rust has debug display inbuilt but we have to opt in first
// before we can use, hence the above

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is {:#?}", rect1);
    // :? tells println! that we want to use the Debug output format
    // :#? helps us to get more cleaner display formats of structs
    // really important when structs are large
    println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
            );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}