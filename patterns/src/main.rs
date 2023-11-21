fn main() {
    let favColor: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<i32, _> = "25".parse();

    if let Some(color) = favColor {
        println!("Using your favorite color, {color}, as the background")
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(4);
    stack.push(11);
    stack.push(22);

    println!("Removing from this stack: {:?}", stack);
    while let Some(value) = stack.pop() {
        println!("Removed {value} from {:?}", stack);
    }
}
