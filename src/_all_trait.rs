pub trait Summary {
  fn summarize(&self) -> String;
  fn summarize1(&self) -> String {
    println!("{}", "summarize1");
    format!("{}", "self impl")
  }
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
    fn summarize1(&self) -> String {
        println!("{}", "summarize1 rewrite");
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn test_trait() {
    let traittmp = NewsArticle {
        headline: String::from("a"),
        location: String::from("b"),
        author: String::from("c"),
        content: String::from("d"),
    };
    let result = traittmp.summarize1();
    print!("{}", result);
}