// A basic trait with two functions
trait Summary {
    fn summarize(&self) -> String;
    fn preview(&self) -> String {
        format!("Read more: {}", self.summarize())
    }
}

// I built a type
struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} with {}", self.headline, self.author, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2.0"),
        author: String::from("Jane Doe"),
        content: String::from("The rust team has announced"),
    };

    println!("Article summary: {}", article.summarize());
}
