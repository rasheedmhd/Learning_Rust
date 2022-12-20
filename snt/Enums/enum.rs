
fn main() {
    // enums can take any kind of datatypes including other enums
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!(".") // function body here
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call()
