// PROGRAMMING RUST BOOK
// CHAPTER 1 - Fundamental Types

// Typing out every type regardless of type inference
//fn build_vector() -> Vec<i16> {
//    let mut v: Vec<i16> = Vec::<i16>::new();
//    v.push(10i16);
//    v.push(20i16);
//    v
//}

// making use of rust type inference
//fn build_vector() -> Vec<i16> {
//    let mut v = Vec::new();
//    v.push(10);
//    v.push(20);
//    v
//}

// we can convert from one type to another using 'as' called Type Casting
// if you trying converting from a larger sized type to a smaller sized Type
// we would cause integer overflow - rust results into truncating

// the std lib provides some methods to work on integers
// .pow()
// .abs()
// .count_ones()

// QUIRKS
fn main()  {
    // println!("{}", (4).abs());
    // fixed
    // println!("{}", (4_u32).abs());
    // println!("{}", u32::abs(4));

    // methods takes precedence over unary operations

    // checked, wrapping, saturating and overflowing arithmetic
    // let mut i = 1;
    //    let mut i: i32 = 1;
    //    loop {
    //        // i = i * 10; // panic: Attempt to multiply with overflow in debug build only
    //        // i *= 10;
    //        // panic: multiplication overflow in any build
    //        i = i.checked_mul(10).expect("multiplication overflowed");
    //    }

    let mut i: i32 = 1;
    loop { i *= 10 }

    // integer arithmetic categories
    // 1. checked - return Option of the result
    //    rust arithmetic behavior in debug builds
    // 2. wrapped - return the value to the mathematically correct result modulo
    //    the range of the value - rust arithmetic behavior in release builds
    // 3. saturating - returns the representable value that is closest to the
    //    mathematically correct result
    //    32760_i16.saturating_add(10) returns -32,767 because that is the minimum
    //    number that an i16 can hold.
    //    no saturating for division, remainder or bitwise shift methods
    // 4. overflowing - returns a tuple (result, overflowed)
    //    result = the result of the operation which has been wrapped or not
    //    overflowed = a boolean that tells if the result was wrapped or not.


    // floating Points
    // Rust provides IEEE single- and double floating point Types
    // f32[FP32] - single- precision
    // f64[FP64] - double- precision

    // rust doesn't do implicit conversions - not even i16 for i32

    // BOOLEAN - [true and false]
    // no implicit conversions
    // represented by a byte so that you can create pointers to it
    // can be type casted to integers(0 and 1) but not vice versa

    // CHARACTERS
    // Single unicode characters as 32 bit values
    // represented with '' [single quotes]
    // if a char is drawn from ASCII it can be written in hexadecimal
    // the std lib has methods on characters

    // TUPLES
    // tuples only allow constants, you can't write t.i or t[i] to get the ith element
    let text = "I see the eigenvalue in thine eye";
    println!("text when not shadowed: {text}");
    let text = text.split_at(21); // shadowing
    let head = text.0;
    let tail = text.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    println!("text when shadowed: {head} {tail}");

    fn f<T: Into<MyType>>(t: T) -> MyType
    {
        t.into()
    }

    fn f<T: Into<String>>(t: T) -> String
    {
        t.into()
    }

    let x = f(b"bytes");
    let y = f("string");
}

