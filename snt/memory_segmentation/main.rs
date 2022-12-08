#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]

pub extern "C" fn main() -> isize {
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}







//fn main() {
//    //println!("Hello World!");
//    std::process::exit(0)
//}