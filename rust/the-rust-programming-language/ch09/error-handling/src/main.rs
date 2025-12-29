// https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result

// use std::fs::File;
//
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {error:?}"),
//     };
// }

// https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors

use std::fs::File;
use std::io::ErrorKind;

#[allow(unused)]
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
