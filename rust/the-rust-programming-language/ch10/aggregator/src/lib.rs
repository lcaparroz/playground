// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#using-default-implementations

// pub trait Summary {
//     fn summarize_author(&self) -> String;
//
//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }
//
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }
//
// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }
// }
//
// pub struct SocialPost {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub repost: bool,
// }
//
// impl Summary for SocialPost {
//     // fn summarize(&self) -> String {
//     //     format!("{}: {}", self.username, self.content)
//     // }
//
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// https://doc.rust-lang.org/stable/book/ch10-02-traits.html#using-traits-as-parameters

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
