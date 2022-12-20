use std::cmp::Ordering;

enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}


// A Message enum whose variants each store different amounts and types of values
enum Message {
    Quit,
    Start,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum ApplicationState {
    Started,
    Running,
    Stopped,
}

let state = ApplicationState::Started;
if state == ApplicationState::Running;

// let state = 1
// if state == 2






// AN ENUM CAN HOLDS VALUES OF DIFFERENT DATA TYPES

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
    }

    // let m = Message::Write(String::from("hello"));
    // m.call();

// OPTION
enum Option<T> {
    None,
    Some(T),
}

// TYPE INFERENCE
// let some_number = Some(5); - Option<String>
// let some_char = Some('e'); - Option<char>

// let absent_number: Option<i32> = None; - explicitly stated for None

// OPTION ERROR -WON'T RUN
//let x: i8 = 5;
//let y: Option<i8> = Some(5);

//let sum = x + y;



