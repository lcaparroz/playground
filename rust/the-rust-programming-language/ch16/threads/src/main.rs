// https://doc.rust-lang.org/stable/book/ch16-01-threads.html#creating-a-new-thread-with-spawn

// use std::thread;
// use std::time::Duration;
//
// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// https://doc.rust-lang.org/stable/book/ch16-01-threads.html#waiting-for-all-threads-to-finish

// use std::thread;
// use std::time::Duration;
//
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     handle.join().unwrap();
//
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
//
//     // handle.join().unwrap();
// }

// https://doc.rust-lang.org/stable/book/ch16-01-threads.html#using-move-closures-with-threads

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
