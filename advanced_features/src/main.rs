// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 19 - Advanced Features 

// === 19.1 - Unsafe Rust

use std::slice;

// UNSAFE RUST ALLOWS US TO ENABLE A SMALL SET OF ADDITIONAL FEATURES
// TO WRITE PROGRAMS THAT THE COMPILER MIGHT DEEM INVALID OTHERWISE
// The compiler is programmed to reject code that it isn't sure of,
// in this regard we can write code that is correct but the compiler 
// rejects it because it doesn't have enough info to be confident that it as valid code
// In situations like this rust allows us to use unsafe {} to bypass
// these optimizations

// Unsafe Rust allows us to interact directly to the OS, do
// low level programming, even create our own OSs

// UNSAFE RUST gives us the ff superpowers
//      1. Raw pointer deferencing
//      2. Calling unsafe functions or methods
//      3. Access or modification of mutable static variables
//      4. Implementation of unsafe traits
//      5. Access to fields of unions

// Unsafe Rust doesn't turn off the borrow checker or safety checking
// You still get some safely with unsafe {}

// DEREFERENCING RAW POINTERS
//   1. Raw pointers can be immutable or mutable just like references
//   2. A raw pointer been immutable means it cannot be directly assigned 
//   a value after it has been derefenced
//   3. Its syntax is *const T and *mut T 
//   read as a const is an immutable raw pointer to any type T and 
//   T is a type that is a mutable raw pointer
//   The asterisk(*) is not the deferencing operator yet, it is part 
//   the syntax of raw pointers 

// RAW POINTERS are;
//      1. Allowed to ignore the borrow checking rules by having both immutable and mutable pointers or
//      multiple mutable pointers to the same memory location.
//      2. Allowed to be null
//      3. Aren't guaranteed to always be pointing to a valid memory location.
//      4. Don't implement any automatic memory clean up

// having these features allows us to have greater performance or interface other languages or 
// hardware where rust safety guarantees do not apply

mod programming_rust;

fn main() {
    //programming_rust::crash();
    println!("Hello, world!");

    // mutable and immutable raw pointers
    // we can create raw pointers in safe code but we can't dereference them in safe code
    // unless in an unsafe {} block
    let mut fiver = 5;

    let r1 = &fiver as *const i32; // immutable raw pointer to fiver
    let r2 = &mut fiver as *mut i32; // mutable raw pointer to fiver

    let address = 0x012345usize;
    let r = address as *const i32;

    println!("r1: {:?} and r2: {:?} and r: {:?}", r1, r2, r);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // We use raw pointers dereferecinng when we are dealing with C
    // And when we are writing safe code abstractions that the compiler doesn't understand.
    unsafe {
        dangerous();
    }

    // dangerous(); this won't compile bc we need to encapsulate unsafe fns with unsage {}
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..]; // mutably all of vector v

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
}

// CALLING UNSAFE FUNCTIONS OR METHODS
unsafe fn dangerous() { println!("Hello, Monseiour Unsafe Function Avec Rust")}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();  // retrieves a pointer to the strings slice
     
    assert!(mid <= len); // panics if we try to use an index  greater than the len 

    unsafe {
        (

            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )

    }

   // (&mut values[..mid], &mut values[mid..])
}


