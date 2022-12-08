// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 5 - STRUCTS

// === 5.3 - Method Syntax


#[derive(Debug)] //formatting trait, needed to display structs
// rust has debug display inbuilt but we have to opt in first
// before we can use, hence the above

struct Rectangle {
    width: u32,
    height: u32,
}

// creating a method for a struct
// method starts with impl (implementation) with the name of the struct
// a function is defined inside the impl which takes only one parameter
// which is a reference to the Struct, a &self parameter of the type of Self
// methods can takes owner of the struct, borrow both mutably and immutably
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

//methods can have names of instance fields
// rect1.width => rust knows we are talking about the field
// rect1.width() => rust knows we are talking about the method
// methods with same name as the fields are called getters,
// getters aren't implemented by default for us in rust be we can implement
// them ourselves to use them to return a field, while get the field private

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
     }
}

// Methods with More Parameters
impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}
// associated functions
// they are called with the :: notation
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { // self == Rectangle
            width: size,
            height: size,
        } 
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!( "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
    println!("Does the rectangle has a width greater than zero? {}", rect1.width())
}