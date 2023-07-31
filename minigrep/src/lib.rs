use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "ast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, contents)
        );
    }
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matched_lines = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matched_lines.push(line);
        }
    }
    
    matched_lines

}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.text_file_path)?;

    for line in search(&config.search_term, &contents) {
        println!("{line}");
    }

    //println!("With text:\n\n{contents}");

    Ok(())
}

pub struct Config {
    pub search_term: String,
    pub text_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let search_term = args[1].clone();
        let text_file_path = args[2].clone();
        Ok(Config { search_term, text_file_path })
        //(search_term, text_file_path)
    }
}

//fn parse_config(args: &[String]) -> Config {
//    let search_term = args[1].clone();
//    let text_file_path = args[2].clone();
//
//
//    Config { search_term, text_file_path }
//
//    //(search_term, text_file_path)
//}
