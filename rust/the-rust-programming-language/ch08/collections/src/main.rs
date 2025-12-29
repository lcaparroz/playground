// https://doc.rust-lang.org/stable/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector

// fn main() {
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{i}");
//     }
// }

// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//         println!("{i}");
//     }
// }

// https://doc.rust-lang.org/stable/book/ch08-02-strings.html#iterating-over-strings

// #[allow(unused)]
// fn main() {
//     for c in "Зд".chars() {
//         println!("{c}");
//     }
//
//     for b in "Зд".bytes() {
//         println!("{b}");
//     }
// }

// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map

// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
//     println!("{team_name} team's score is {score}");
// }

// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }

// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#managing-ownership-in-hash-maps

// fn main() {
//     use std::collections::HashMap;
//
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");
//
//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//
//     println!("{field_name}: {field_value}");
// }

// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#managing-ownership-in-hash-maps

// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);
//
//     println!("{scores:?}");
// }

// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#managing-ownership-in-hash-maps

// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//
//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
//
//     println!("{scores:?}");
// }

// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{map:?}");
}
