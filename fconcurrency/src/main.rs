use std::thread;
use std::time::Duration;


fn main() {
    // When the main thread finish running, all threads are closed
    // regardless to whether they have completed running or not.
    println!("Hello, world, Fearless Concurrency!");

    let e = 11; 
    let v = vec![1, 2, 3];

    let handle1 = thread::spawn( move || {
        println!("Here's a vector: {:?}", v);
    });

    handle1.join().unwrap();

    let handle = thread::spawn(||
        for i in 2..10 {
            println!("got {i}, by looping in spawned thread.");
            thread::sleep(Duration::from_millis(1))
        }
    );

    handle.join().unwrap();

    for i in 1..5 {
        println!("got {i}, by looping in main thread.");
        thread::sleep(Duration::from_millis(1))
    }

    // handle.join().unwrap();
}
