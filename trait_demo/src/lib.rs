use std::fmt::{format, Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String;

//  默认实现  默认实现的方法中可以调用trait中的其他方法，即使这些方法没有默认实现
//  无法从方法的重写实现里面调用默认的实现
    // fn summarize(&self) -> String {
    //     String::from("Read more")
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为类型 实现 Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline,self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为类型 实现 Summary
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait 作为接口
pub fn notify(item: impl Summary){
    print!("breading news: {}", item.summarize());
}

// 实现两个 trait
pub fn notify3(item: impl Summary + Display){
    print!("breading news: {}", item.summarize());
}

// trait 作为接口, trait bound 方式
pub fn notify2<T: Summary>(item: T){
    print!("breading news: {}", item.summarize());
}

// 实现两个 trait
pub fn notify4<T: Summary + Display>(item: T){
    print!("breading news: {}", item.summarize());
}

pub fn notify5<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("breading news: {}", a.summarize())
}

// 使用  where 简化
pub fn notify6<T, U>(a: T, b: U) -> String 
where 
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("breading news: {}", a.summarize())
}

// 使用 trait 作为返回值， 只能返回同一种具体类型
pub fn notify7() -> impl Summary {
    NewsArticle {
        headline: String::from("hah"),
        location: String::from("啊啊"),
        content: String::from("这里得得多多"),
        author: String::from("beijing"),
    }
}
