fn main() {
    let tweet = Tweet {
        username: String::from("roye"),
        content: String::from("Learning Rust"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let post = Post {
        author: String::from("royge"),
        title: String::from("Learning Trait"),
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        author: String::from("royge"),
        headline: String::from("Rust is the next Big Thing"),
        content: String::from("Learn Rust while you can."),
        location: String::from("Cebu, Philippines"),
    };
    println!("News of the day: {}", article.summarize());

    let tweet2 = &tweet;

    notify(article);
    notify_more(&tweet, &post);
    notify_most(&tweet, tweet2);

    println!("Summarizable: {}", summarizable().summarize());
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify_more(item1: &impl Summary, item2: &impl Summary) {
    println!("Trending: {}", item1.summarize());
    println!("Popular: {}", item2.summarize());
}

pub fn notify_most<T: Summary>(item1: &T, item2: &T) {
    println!("Most Trending: {}", item1.summarize());
    println!("Most Popular: {}", item2.summarize());
}

pub fn summarizable() -> impl Summary {
    Tweet {
        username: String::from("roye"),
        content: String::from("Learning Rust"),
        reply: false,
        retweet: false,
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Post {
    pub author: String,
    pub title: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
