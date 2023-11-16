// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;

use std::sync::Mutex;

fn main() {
    // When the main thread finish running, all threads are closed
    // regardless to whether they have completed running or not.
    // println!("Hello, world, Fearless Concurrency!");

    // let e = 11; 
    // let v = vec![1, 2, 3];

    // let handle1 = thread::spawn( move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle1.join().unwrap();

    // let handle = thread::spawn(||
    //     for i in 2..10 {
    //         println!("got {i}, by looping in spawned thread.");
    //         thread::sleep(Duration::from_millis(1))
    //     }
    // );

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("got {i}, by looping in main thread.");
    //     thread::sleep(Duration::from_millis(1))
    // }

    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();

    // let tx1 = tx.clone();

    // thread::spawn( move || {
    //     let msg = String::from("Mendeleev Crypto Exchange");
    //     let chats = vec![
    //             String::from("hi"),
    //             String::from("I"),
    //             String::from("am"),
    //             String::from("buying"),
    //             "---------------------".to_string(),
    //         ];
    //     // tx sends a Result<T, E>
    //     // returns Error if rx has been dropped already
    //     tx.send(msg).unwrap();

    //     // println!("msg is {}", msg);

    //     for chat in chats {
    //         tx.send(chat).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });


    // obviously my small crypto startup doesn't work, I give up.
    // The messages are getting sent in a random order, not good for business,
    // I am dumb, I can't fix it. I give up. 
    // thread::spawn( move || {
    //     let crypto = vec![
    //         String::from("Bitcoin"),
    //         String::from("Tether"),
    //         String::from("Ethereum"),
    //         String::from("Cardano"),
    //     ];

    //     for coin in crypto {
    //         tx1.send(coin).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // rx blocks main thread and returns a Result<T, E>
    // returns Error if tx has been closed and stopped sending

    // try_recv doesn't block main thread
    // let received_message = rx.recv().unwrap();
    // let received_chats = rx.try_recv().unwrap();

    // for chat in rx {
    //     println!(">> this user: {:?}", chat );
    // }

    // print!("Message received from spawned thread: {:?}", received_message);

    let n = Mutex::new(5);

    let some_fucker = n.lock().unwrap();

    {
        let mut num = n.lock()
        .unwrap();
        *num = 6;
        println!(">> n: {:?}", n);
    }

    println!(">> n: {:?}", n);
}































// USING MESSAGE PASSING TO TRANSTER DATA BETWEEN THREADS