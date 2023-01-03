use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.search_term);
    println!("In file {}", config.text_file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


//dbg!(&args);

//let (search_term, text_file_path) = parse_config(&args);
//let config = parse_config(&args);



//let search_term = &args[1];
//let text_file_path = &args[2];

//println!("Hello, world!, args contains {} words", &args.len());










