// https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    let num = 11;
    println!("Hello again! {num} plus two is {}!", add_two::add_two(num));
}
