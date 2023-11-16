use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    // let n = Mutex::new(5);

    // // let some_fucker = n.lock().unwrap();

    // {
    //     let mut num = n.lock().unwrap();
    //     *num = 6;
    //     println!(">> n: On L99 {:?}", n);
    // }

    // println!(">> n: {:?}", n);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}































// USING MESSAGE PASSING TO TRANSTER DATA BETWEEN THREADS