// https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic

// fn main() {
//     panic!("crash and burn");
// }

fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
