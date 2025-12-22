// https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#parameters

// fn main() {
//     another_function(5);
// }
//
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
