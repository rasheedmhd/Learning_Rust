fn add_one(i: i32) -> i32 {
    i + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {

    let ans = do_twice(add_one, 5);

    let nums = vec![1,2,3,4,5];
    // let num_as_strings: Vec<String> =
    // nums.iter().map(|i| i.to_string()).collect();

    let num_as_strings: Vec<String> =
    nums.iter().map(ToString::to_string).collect();

    println!("Hello, world! Let's do Advanced Functions");
    println!("Answer: {ans}");
    println!("Answer: {:?}", num_as_strings);
}
