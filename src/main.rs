use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // let mut a = 0;
    // for i in 0..10 {
    for _ in 0..10 {
        // a += 1;
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // thread::sleep_ms(50);
            let mut num = counter.lock().unwrap();
            *num += 1;
            // println!("num: {}", num);
            // println!("i: {}", i);
        });
        // println!("handle: {:?}", handle);
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // thread::sleep_ms(50);

    println!("Result: {}", *counter.lock().unwrap());
}
