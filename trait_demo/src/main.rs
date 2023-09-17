use trait_demo::Summary;
use trait_demo::Tweet;

fn main() {

    let tweet = Tweet{
        username: String::from("张三"),
        content: String::from("哈哈哈笑了起来"),
        reply: false,
        retweet: false,
    };

    print!("1 new tweet: {}", tweet.summarize());

}
