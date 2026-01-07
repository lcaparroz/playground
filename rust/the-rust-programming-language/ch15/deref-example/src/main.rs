// https://doc.rust-lang.org/stable/book/ch15-02-deref.html#defining-our-own-smart-pointer

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// https://doc.rust-lang.org/stable/book/ch15-02-deref.html#using-deref-coercion-in-functions-and-methods

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
