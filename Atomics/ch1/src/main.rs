use std::thread;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::time::Duration;

fn main() {
    let q = Mutex::new(VecDeque::new());

    thread::scope( |s| {
        // consuming thread
        let t = s.spawn(|| loop {
            let item = q.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        // consuming thread
        for i in 0.. {
            q.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(3));
        }
    });

}