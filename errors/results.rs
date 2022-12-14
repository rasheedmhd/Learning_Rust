// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 9 - ERROR HANDLING

// === 9.1 - Recovering Erros with Result

// The Result Enum
use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    enum Result<T, E> {
        Ok(T), // what is returned when the code is successful
        Err(E), // what is returned when the code throws an error
    }

    let greeting_file_result = File::open("Hello.txt");
    // open() returns a Result<T, E> enum
    // The Result<T, E> has only 2 variant Ok, and Err
    // it returns Ok, when the operation is successful & returns Err
    // when the operation fails
    // Ok holds the successful returned value and its type
    // Err holds the the failed operation and throws an error kind info
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    // Matching on Different Errors
    // Shortcuts for Panic on Error: unwrap and expect
    // unwrap is a Shortcut just like match
    // when called on Result<T,E> it returns the okay variant
    // if successful or panics if the operation fails

    // let open_unwrap = File::open("ello.txt").unwrap();

    let open_unwrap = File::open("llo.txt")
    .expect("hello.txt should be included in this project");

    // expect()
    // we can use expect to return a custom error message when the operation fails
    // expect() works similarly to unwrap()

    // Propagating Errors
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("Hello.txt"); // Open file to get a File Handler

        // check if the opening succeeded bc open() returns Result
        let mut username_file = match username_file_result {
            Ok(file) => file, // return file handler if open succeeded
            Err(e) => return Err(e), // return error in it failed
        };

        let mut username = String::new();

        // read the contents of username_file into username variable
        // this could fail so we need match to handle the returned Result
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username), // returns username if we successful copied contents to username variable
            Err(e) => Err(e), // rturn an error to the calling code if operation failed
        }
    }


    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("Hello.txt")?;
        //? returns the Ok value if successful and Err value from the whole
        // function to the calling code, just like using return

        let mut username = String::new();

        username_file.read_to_string(&mut username)?;
        Ok(username)
        // ? is Different from match in the way they handle the errors received
        // ? passes errors to the from function defined in the Form trait, which is
        // used to convert one type ot another
        // when ? passes the error, the error type is converted to the error type
        // defined in the return type of the current function
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("Hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // we can even make the above function shorter
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("Hello.txt");
    }
    // you can us ? on a function that returns a Result
    // and you can also use it on a function that returns an Option
    // but you can't mix and match. ? won't auto convert btn the 2
    // however you can use ok on Result or the ok_or on Option to do the conversion explicitly

}













