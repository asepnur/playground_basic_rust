pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub trait Summary {
    fn summarize(&self) -> String;
    // actually this is bad design please dont do this in production
    // example to apply default method
    fn print_summarize(&self, s:String){
        println!("default summary: {}", s);
    }
}
// returning type of trat
pub fn new_summary_tweet(username: String, content: String, reply: bool, retweet: bool) -> impl Summary {
    Tweet{username, content, reply, retweet}
}
pub trait Display {
    fn display(&self);
}
impl NewsArticle {
    pub fn new(headline:String, location:String, author: String, content:String) -> NewsArticle {
        NewsArticle{headline, location, author, content}
    }
}
impl Summary for NewsArticle {
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

impl Tweet {
    pub fn new(username:String, content: String, reply:bool, retweet:bool) -> Tweet{
        Tweet{
            username, content, reply, retweet
        }
    }
}
impl Summary for Tweet{
     fn summarize(&self) -> String{
         format!("{}: {}", self.username, self.content)
     }
}

impl Display for Tweet{
    fn display(&self){
        println!("{} : {}", self.username, self.content);
    }
}