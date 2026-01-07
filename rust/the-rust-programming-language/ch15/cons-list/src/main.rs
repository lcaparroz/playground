// https://doc.rust-lang.org/stable/book/ch15-01-box.html#understanding-the-cons-list

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     println!("{list:?}");
// }

// https://doc.rust-lang.org/stable/book/ch15-04-rc.html#sharing-data

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

// #[allow(unused)]
// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
//
//     println!("{a:?}");
//     println!("{b:?}");
//     println!("{c:?}");
// }

// https://doc.rust-lang.org/stable/book/ch15-04-rc.html#cloning-to-increase-the-reference-count

// #[allow(unused)]
// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
//     drop(a);
//     println!("{b:?}");
// }

// https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html#allowing-multiple-owners-of-mutable-data

// #[allow(unused)]
// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;
//
// fn main() {
//     let value = Rc::new(RefCell::new(5));
//
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//
//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
//
//     *value.borrow_mut() += 10;
//
//     println!("a after = {a:?}");
//     println!("b after = {b:?}");
//     println!("c after = {c:?}");
// }

// https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html#creating-a-reference-cycle

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // The following statement results in a stack overflow
    // println!("a next item = {:?}", a.tail());
}
