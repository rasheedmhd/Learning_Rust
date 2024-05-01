// !LESSONS! 

// it is a good practice to restructure your codes when it grows
// this helps in reducing the complexity of the program
// Group things in structs
// and others in a completely new file like lib, from main

// Don't rely on generic error messages, try to return specific errors messages
// Helps in understanding your errors which helps in understanding the program
// in prod and how to fix it.

// returning () is the idiomatic way to calling functions for their side effects only

// !endLesson!

use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
	eprintln!("Application error: {e}");
	process::exit(1);
    }

    //println!("Searching for {}", config.search_term);
    //println!("In file {}", config.text_file_path);
}


//dbg!(&args);

//let (search_term, text_file_path) = parse_config(&args);
//let config = parse_config(&args);



//let search_term = &args[1];
//let text_file_path = &args[2];

//println!("Hello, world!, args contains {} words", &args.len());










