fn main() {

}

pub fn crash() {
    let mut a: usize = 0;
    let ptr = &mut a as *mut usize;
    unsafe {
        *ptr.offset(13786) = 0x7ffff72f484c;
    }
}