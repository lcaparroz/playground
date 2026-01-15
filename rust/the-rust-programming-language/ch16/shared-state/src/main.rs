// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#the-api-of-mutext

// use std::sync::Mutex;
//
// fn main() {
//     let m = Mutex::new(5);
//
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//
//     println!("m = {m:?}");
// }

// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#shared-access-to-mutext
// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#the-api-of-mutext

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            print!("thread #{i}: {num} => ");
            *num += 1;
            println!("{num:?}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
