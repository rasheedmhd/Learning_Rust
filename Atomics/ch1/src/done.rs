use std::cell::Cell;
use std::thread;
// Using Zero Sized Types
// Do not exist at Run time
use std::marker::PhantomData;
use std::sync::Mutex;

#[derive(Debug)]
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>,
}

#[derive(Debug)]
struct X2 {
    p: *mut i32,
}

// Opting in Send and Sync
unsafe impl Send for X2 {}
unsafe impl Sync for X2 {}


fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = b.get();
    if before != after {
        println!("Might never happen");
    }
}


fn main() {
    println!("Hello, world, Atomics and Locks");
    f(&Cell::new(6), &Cell::new(5));

    let n = Mutex::new(0);

    thread::scope( |s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }               
            });
        }
    });//.join().unwrap();

    // let xk = X {
    //     handle: 71,
    //     _not_sync: PhantomData::new(Cell::new(())),
    // };

    let xk = X2 {
        p: &mut 45,
    };

    // Moving none Send and Sync into threads
    thread::spawn( move || {
        println!("Struct: {:#?}", xk);
    }).join().unwrap();

}

