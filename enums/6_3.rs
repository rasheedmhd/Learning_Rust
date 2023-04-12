// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 6 - ENUMS AND PATTERN MATCHING

// === 6.3 - Concise Control Flow with if let

// the if let syntax lets you combine if and let to handle values
// that match a pattern while ignoring the rest
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // using the if let syntax
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let is more like syntactic sugar on top of match
    // if let has a down side of not proceeding exhausted checking like match
    // for that purpose I would personally favor using match than if let
    // put always remember that if let is available
    // you can add an option 'else' statement to use with your if let
    // else would handle the catch-all operator that we used in match
}
