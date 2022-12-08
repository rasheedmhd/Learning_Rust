// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 6 - ENUMS AND PATTERN MATCHING

// === 6.1 - Defining an Enum


// enums allows us to create custom data types
enum IPAddressKind {
    v4,
    v6,
} // IPAddressKind is a custom data type. We can use it anywhere in our code in scope.
fn main () {
    // ENUM VALUES
    let four = IPAddressKind::v4;
    let six = IPAddressKind::v6;

    // we can create a function that takes in an IPAddressKind as a datatype
    fn route(ip_address: IPAddressKind) {
        // do epic sh*t
    }
    // calling the function and passing in the 2 IP Address variants
    route(IPAddressKind::v4);
    route(IPAddressKind::v6);
    // IPAddressKind::v4 and IPAddressKind::v6 should be real ip addresses.

    // now we have defined a custom datatype for ip addresses, we need to define a [struct]ure,
    // to store ip addresses, because they aren't just types right? the types just make way
    // for the ip addresss values. Let's used 'struct' to built on our knowledge of chp 5

    struct IPAddress {
        kind: IPAddressKind, // using our custom data type as if it were a primitive
        address: String,
    }
    // our custom data type, IPAddressKind at its core, is a String(in this case),
    // that means, it is been stored in the heap segment of our memory,
    // and it is been moved not copied in rebinding.

    let home = IPAddress {
        kind: IPAddressKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPAddress {
        kind: IPAddressKind::v6,
        address: String::from("::1")
    };

    // in the preceeding code we used enums to define the data type of our ip addresses
    // tha this cool and enums have an even cooler future.
    // that is we can specify that an enums variant will always be a particular datatype
    // so that when we are creating a variable, we can just pass the particular data/value
    // into the enum to initialize the variable.
    enum IPAddressKind {
        v4(String),
        v6(String),
    }
    // home and loopback hence, becomes
    // this is cleaner, concise and easier that using enums + structs
    let home = IPAddressKind::v4(String::from("127.0.0.1"));
    let loopback = IPAddressKind::v6(String::from("::1"));

    // enums can take any kind of datatypes including other enums
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // An enums with different types of data types
    // The message enum above is very similar to a struct exvept that
    // it doesn't use the struct keyword and everything is grouped
    // under message.
    // Let's deconstruct the enum above into variables
    struct Quit(); // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // it is very evident that struts and enums have a lot in common
    // another similarity is that just like you can define methods
    // on structs with 'impl' you can define methods on enums same way
    impl Message {
        fn call(&self) {
            println!(".") // function body here
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call()
}
