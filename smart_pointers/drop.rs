struct CustomSmartPointer {
    value: String,
}

// A trait that allows us to write custom code for when a value is about to go out of scope
// Code in the drop method of the trait impl doesn't needs explicit calling
impl Drop for CustomSmartPointer {
    //    type Target = String; 
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer, `{}`!", self.value);
    }
}


fn main() {
    let stuff_one = CustomSmartPointer {
        value: String::from("Doing plenty stuff here")
    };
    
    //    stuff_one.drop(); -> this fails
    //    Rust doesn't allow us to all the drop method in the trait explicitly
    //    The whole idea for the trait is for Rust to handle dropping data for us manaually
    //    Drop Trait is added into the prelude hence we need not include
    //    Rust gives gives us the std::mem::drop to do explicit data cleaning which is also included in the prelude
    
    drop(stuff_one);
    
    let other_stuff = CustomSmartPointer {
        value: String::from("Doing other stuff here")
    };
    println!("CustomSmartPointers created sir!");   
}