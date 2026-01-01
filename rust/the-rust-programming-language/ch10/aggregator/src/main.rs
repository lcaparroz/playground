// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

// use aggregator::{SocialPost, Summary};
//
// fn main() {
//     let post = SocialPost {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         repost: false,
//     };
//
//     println!("1 new post: {}", post.summarize());
// }

// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#using-default-implementations

use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    aggregator::notify(&article);

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
    aggregator::notify(&post);
}
