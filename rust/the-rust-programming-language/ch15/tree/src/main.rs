// https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html#creating-a-tree-data-structure
// https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(unused)]
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("After creating `leaf`:");
    println!("`leaf`: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("`leaf`: parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        println!("After creating `parent`:");
        println!("`branch`: strong count = {}, weak count = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("After assigning `branch` to `leaf` as parent:");
        println!("`branch`: strong count = {}, weak count = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("`leaf`: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("`leaf`: parent = {:?}", leaf.parent.borrow().upgrade());
    }

    println!("After exiting `branch` scope");
    println!("`leaf`: strong count = {}, weak count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("`leaf`: parent = {:?}", leaf.parent.borrow().upgrade());
}
