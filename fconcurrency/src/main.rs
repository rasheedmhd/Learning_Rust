use std::thread;
use std::time::Duration;


fn main() {
    // When the main thread finish running, all threads are closed
    // regardless to whether they have completed running or not.
    println!("Hello, world, Fearless Concurrency!");

    let handle = thread::spawn(||
        for i in 2..10 {
            println!("got {i}, by looping in spawned thread.");
            thread::sleep(Duration::from_millis(1))
        }
    );

    // handle.join().unwrap();

    for i in 1..5 {
        println!("got {i}, by looping in main thread.");
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();
}
