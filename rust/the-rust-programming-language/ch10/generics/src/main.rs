// https://doc.rust-lang.org/stable/book/ch10-00-generics.html#removing-duplication-by-extracting-a-function

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let mut largest = &number_list[0];
//
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     println!("The largest number is {largest}");
//     assert_eq!(*largest, 100);
// }

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
//     assert_eq!(*result, 100);
//
//     let number_list = vec![102, 34, 6000, 89, 52, 2, 43, 8];
//
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
//     assert_eq!(*result, 6000);
// }

// https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-function-definitions

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }

// https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-method-definitions

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn main() {
//     let p = Point { x: 5, y: 10 };
//
//     println!("{p:?}");
//     println!("p.x = {}", p.x());
//
//     let p = Point { x: 3.0, y: 4.0 };
//
//     println!("{p:?}");
//     println!("distance from origin: {}", p.distance_from_origin());
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
