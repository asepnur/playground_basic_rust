extern crate playground_basic_rust;

use playground_basic_rust::tweet;
// trait must be in your scope
// so please import the trait if you want to use the method of specific trait
// remove this line will affect error
use crate::playground_basic_rust::tweet::Summary;

fn main(){
    let article_1 = tweet::NewsArticle::new(
        String::from("healine of article"), 
        String::from("actual location"), 
        String::from("me as author"), 
        String::from("the content is length ...")
    );

    let summarize = article_1.summarize();
    article_1.print_summarize(summarize);

    let tweet_1 = tweet::Tweet::new(
        String::from("itsmedeveloper"),
        String::from("my frist tweet ever in this world"),
        false,
        false,
    );
    let summarize_tweet = tweet_1.summarize();
    tweet_1.print_summarize(summarize_tweet);
}