// https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#grouping-related-code-in-modules

// #[allow(unused)]
// mod front_of_house{
//     mod hosting {
//         fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }

// https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword

// #[allow(unused)]
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public

// #[allow(unused)]
// pub mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast.
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like.
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//     // meal.seasonal_fruit = String::from("blueberries");
// }

// https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
