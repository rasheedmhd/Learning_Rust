// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 5 - STRUCTS

// === 5.1 - Defining and Instantiating Structs

fn main() {
    // holds multiple related values
    // values can be of different datatypes
    // you define the names of the data and each type
    // to use a struct we define it like below then we create an instance of it
    struct user { // struct defined
        username: String,
        email: String,
        age: i32,
        sign_in_count: u64,
        active: bool,
    };

    let user1 = user { // an instance of a struct
        email: String::from("user1@user1.com"),
        username: String::from("user_one_here"),
        active: true,
        sign_in_count: 1,
        age: 34,
    };

    // when creating structs instances you can decide to make them mutable
    // so that you can mutate data fields later on
    // the entire instance must be mutable, Rust doesn't allow us to mutate fields individually

    let mut user1 = user { // a mutable instance of a struct
        email: String::from("user1@user1.com"),
        username: String::from("user_one_here"),
        active: true,
        sign_in_count: 1,
        age: 34,
    };
    // you can mutate a fields data now
    // we use . notation to access structs instances values
    user1.username = String::from("user_one_mutated");

    //  A STRUCT INSTANCE CAN BE RETURNED AS AN EXPRESSION
    fn build_user(email: String, username: String) -> user {
        user {
            email,
            username,

            // this is a little bit confusing right? Why?
            // u might have been expecting
            // email: email, username: username, active: true(we set it ourself)

            // when the parameter name is the thesame as the struct field name, rust allows
            // us to use only the parameter name hence the code below
            // THIS IS CALLED 'THE FIELD INIT SHORTHAND'

            // if there are some fields in the struct instance that our function don't explicitly
            // define that we need to initialize with data through parameters, rust results on the defaults
            // defined in the struct instance.
            // We should still set active: , sign_in_count:  and age:

            active: true,
            sign_in_count: 1,
            age: 34,
        }
    }

    // STRUCT UPDATE SYNTAX
    // creating a instance from a struct instance
    let user2 = user {
        active: user1.active,
        username: user1.username,
        email: String::from("user2@user2.com"),
        sign_in_count: user1.sign_in_count,
    };

    // it is time consuming to have to manually type to get all the data from the
    // first instance when you only need to change one field
    // rust allows us to change what we want to change and copy the rest
    // using the struct update syntax, the above code becomes

    // it should be noticed that we are essentially moving user1 into user2
    let user2 = user {
        email: String::from("user2@user2.com"),
        ..user1 // this is where all the magic is done sir, you feel me?
    };

    // TUPLE STRUCTS
    // allows have to have named tuples
    // tuple structs are used when we need a distinct typed tuple but names of the fields aren't important
    // defining tuple structs starts with the struct keyword
    struct Color(i32, i32, i32);
    struct Point(i32,i32, i32);

}

fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    // tuple structs are distinctly type
    // a function that takes Color as a parameter cannot be passed Point
    // because they are not the same type
}

// UNIT-LIKE STRUCTS
// structs without any fields
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}














