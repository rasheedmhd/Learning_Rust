use std::sync::Mutex;

fn main() {

    let n = Mutex::new(5);

    // let some_fucker = n.lock().unwrap();

    {
        let mut num = n.lock().unwrap();
        *num = 6;
        println!(">> n: On L99 {:?}", n);
    }

    println!(">> n: {:?}", n);
}































// USING MESSAGE PASSING TO TRANSTER DATA BETWEEN THREADS