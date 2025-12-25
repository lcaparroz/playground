// https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    print_user(&user2);
    // print_user(&user1);
}

fn print_user(user: &User) {
    println!("user.email: {}", user.email);
    println!("user.username: {}", user.username);
    println!("user.active: {}", user.active);
    println!("user.sign_in_count: {}", user.sign_in_count);
}
