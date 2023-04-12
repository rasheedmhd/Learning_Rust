const CANT_SHADOW: &str = "Hello";

fn main()
{
    let CANT_SHADOW = "world"; // throws an error
    let shadow_able = "Heyya!";
    println!("shadow_able: {}", shadow_able);
    let shadow_able = "Salut!";
    println!("shadowed: {}", shadow_able);
}
