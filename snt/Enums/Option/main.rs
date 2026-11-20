use std::ptr;

fn main() {
    let pointer: *const i32 = ptr::null();
    assert!(pointer.is_null());
    if pointer.is_null() { println!("{:?} is a pointer", pointer) } else { println!("{:?} is not a pointer", pointer) };
    println!("Hello, world! pointy pointers");
}
