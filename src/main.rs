fn main() {
    let number_list = vec![2, 27, 15, 5, 10, 12];
    let char_list = vec!['y', 'm', 'b', 'z', 'u'];

    //let result = largest(&number_list);
    //println!("Largest number is {}", result);

    //let result = largest(&char_list);
    //println!("Largest char is {}", result);

    let integer_point = Point{x: 3, y: 7};
    let float_point = Point{x: 4.0, y: 5.7};
    dbg!(&integer_point);
    dbg!(&float_point);

    dbg!(integer_point.x());

    // dbg!(integer_point.distance_from_origin());  // not defined for integers
    dbg!(float_point.distance_from_origin());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

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
}

fn largest<T>(list: &[T]) {//-> T {
    //let mut largest_v = list[0];

    /*for item in list {
        if item > largest_v {
            largest_v = item;
        }
    }*/

    //largest_v
    //list[0]
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {  // generic type T in the impl
    fn x(&self) -> &T {
        &self.x
    }
}

// Can define methods specifically for some concrete types
impl Point<f32> {  // impl is not generic, specific for f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// Traits
pub trait Summary {
    fn summarize(&self) -> String {
        // Default implementation
        format!("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}  // use default implementation

/*impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}*/

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Can implement trait for type anywhere within the crate (as long as things are in scope)
mod teste1 {
    mod teste2 {
use super::super::{Tweet, Summary};

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

    }
}

// the only restriction is called coherence: either the type or the trait (or both) must be from
// the local crate
