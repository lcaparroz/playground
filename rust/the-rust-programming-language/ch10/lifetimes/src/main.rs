// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#generic-lifetimes-in-functions
// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#in-function-signatures

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");
// }

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is '{result}'");
// }
//
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#in-struct-definitions

// #[allow(unused)]
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// #[allow(unused)]
// fn main() {
//     let novel = String::from("Call me Ishmael. Some year ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//
//     println!("{}", i.part);
// }

// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#in-method-definitions

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// #[allow(unused)]
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }
//
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {announcement}");
//         self.part
//     }
// }
//
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//
//     println!("{}", i.announce_and_return_part("Thank you for the attention!"));
// }

// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#generic-type-parameters-trait-bounds-and-lifetimes

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is '{result}'");
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
