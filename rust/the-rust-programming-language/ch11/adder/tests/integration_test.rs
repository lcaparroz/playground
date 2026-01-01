// https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#the-tests-directory
// https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#submodules-in-integration-tests

use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4)
}
