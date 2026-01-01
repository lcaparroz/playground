// https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#structuring-test-functions

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

// https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#testing-equality-with-assert_eq-and-assert_ne

// pub fn add_two(a: u64) -> u64 {
//     a + 3
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_adds_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }
// }

// https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#using-resultt-e-in-tests

// pub fn add(left: u64, right: u64) -> u64 {
//     // left + right
//     left + right + 1
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() -> Result<(), String> {
//         let result = add(2, 2);
//
//         if result == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html#running-a-subset-of-tests-by-name

// pub fn add_two(a: u64) -> u64 {
//     a + 2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn add_two_and_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }
//
//     #[test]
//     fn add_three_and_two() {
//         let result = add_two(3);
//         assert_eq!(result, 5);
//     }
//
//     #[test]
//     #[ignore]
//     fn one_hundred() {
//         let result = add_two(100);
//         assert_eq!(result, 102);
//     }
// }

// https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#private-function-tests
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
