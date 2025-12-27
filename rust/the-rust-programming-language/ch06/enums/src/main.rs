// https://doc.rust-lang.org/stable/book/ch06-02-match.html#patterns-that-bind-to-values

// #[allow(dead_code)]
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
//
// #[allow(dead_code)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
//
// fn main() {
//     let value = value_in_cents(Coin::Quarter(UsState::Alaska));
//     println!("The value is {value}.");
// }

// https://doc.rust-lang.org/stable/book/ch06-03-if-let.html#staying-on-the-happy-path-with-letelse

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        println!("{desc}");
    }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
}
