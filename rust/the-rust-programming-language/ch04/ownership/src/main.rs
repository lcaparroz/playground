// https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//
//     println!("{s1}, world");
// }

// https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#ownership-and-functions

// fn main() {
//     let s = String::from("hello");
//
//     takes_ownership(s);
//
//     let x = 5;
//
//     makes_copy(x);
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }

// https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#references-and-borrowing

// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The lenght of '{s1}' is {len}");
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#mutable-references

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
}
