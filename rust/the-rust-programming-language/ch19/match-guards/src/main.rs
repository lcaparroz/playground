// https://doc.rust-lang.org/stable/book/ch19-03-pattern-syntax.html#adding-conditionals-with-match-guards

// fn main() {
//     let num = Some(5);
//
//     match num {
//         Some(x) if x % 2 == 0 => println!("The number {x} is even"),
//         Some(x) => println!("The number {x} is odd"),
//         None => (),
//     }
// }

// fn main() {
//     let x = Some(5);
//     let y = 10;
//
//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("Matched, n = {n}"),
//         _ => println!("Default case, x = {x:?}"),
//     }
//
//     println!("at the end: x = {x:?}, y = {y}");
// }

fn main() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
