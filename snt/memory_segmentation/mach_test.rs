// an unitialized constant
// RUST DOESN'T ALLOW const THIS_IS_A_RUST_CONSTANT: u64;
const THIS_IS_A_RUST_CONSTANT: u64 = 64;
// an unitialized constant
const CONSTANT_3: u64 = 23;


fn main() {

    let unitialized_variable: u32;
    let unitialized_string: String;

    let integer_variable: u32 = 32;
    let boolean_variable: bool = true;
    let string_variable: String = String::from(
                            "This is a string variable"
                         );
    struct Cart {
        items_count: u32,
        name: String,
        cart_owner: String,
        status: bool,
    }

    println!(
            "
            This prints out text and some variables,
            {integer_variable},
            {boolean_variable},
            {string_variable}"
    );

    println!(
            "
            This prints out a constant {THIS_IS_A_RUST_CONSTANT}
            and this {CONSTANT_3}
            "
    );
}