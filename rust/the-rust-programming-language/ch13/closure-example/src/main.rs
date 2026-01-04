// https://doc.rust-lang.org/stable/book/ch13-01-closures.html#inferring-and-annotating-closure-types

// use std::thread;
// use std::time::Duration;
//
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//
//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }
//
// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;
//
//     generate_workout(simulated_user_specified_value, simulated_random_number);
// }

// https://doc.rust-lang.org/stable/book/ch13-01-closures.html#capturing-references-or-moving-ownership

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining the closure: {list:?}");
//
//     let only_borrows = || println!("From closure: {list:?}");
//
//     println!("Before calling closure: {list:?}");
//     only_borrows();
//     println!("After calling closure: {list:?}");
// }

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");
//
//     let mut borrows_mutably = || list.push(7);
//
//     borrows_mutably();
//
//     println!("After calling closure: {list:?}");
// }

use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
