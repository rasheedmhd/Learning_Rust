fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // WE CAN CREATE METHODS FOR ENUMS
impl Message {
        fn Add_World(&self) -> &self  {
            // method body would be defined here
            println!("{}, World", )
        }
    }

    let world = Message::Add_World(String::from("Whatever"));
}