// https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#repeating-code-with-loop

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#disambiguating-with-loop-labels

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//          count += 1;
//     }
//     println!("End count = {count}");
// }

// https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#streamlining-conditional-loops-with-while

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}!");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

// https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#looping-through-a-collection-with-for

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
