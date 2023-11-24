// DECLARATIVE MACROS
// Glorified match statements that produce code at comp time
// #[macro_export]
// macro_rules! vec {
//     ( $ ($x: expr), * ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )* 
//             temp_vec
//         }
//     }
// }

// Macro Expansion for
// vec![1,2,3]
// becomes
// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec
// }


// PROCEDURAL MACROS
// More like functions
// takes codes as input
// operate on the code
// produce and return some code as output

// 3 Kinds of Procedural Macros
// 1. Custom derive
// 2. Attribute-like
// 3. Function-like


// use proc_macro;

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
//     //
// }


// Writing Custom derive Macro
// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
// struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

// fn main() {
//     // Pancakes::hello_macro();
//     println!("Hello, world, Macro Rules!");
// }


use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}


// ATTRIBUTE LIKE MACROS
// derive
// only works on enums and structs
// Allows you to create new attributes