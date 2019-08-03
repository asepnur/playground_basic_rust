pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub trait Summary {
    fn summarize(&self) -> String;
    // actually this is bad design please dont do this in production
    fn print_summarize(&self, s:String);
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
    fn print_summarize(&self, s:String){
        println!("summarize of your articel: {}",s);
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
     fn print_summarize(&self, s:String){
        println!("summarize of your articel: {}",s);
    }
}