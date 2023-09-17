use std::fmt::Display;

struct ImportantExcerpt<'a> {
    // part 这个引用的生命周期要比实例要长
    part: &'a str,
}

// 使用生命周期
impl<'a> ImportantExcerpt<'a> {
    // self 省略生命周期
    fn level(&self) -> i32 {
        3
    }
}

// 生命周期和泛型一起使用
fn longest_with_an_announcement<'a, T> 
    (x: &'a str, y: &'a str, ann: T) -> &'a str 
    where 
        T: Display
{
    print!("Announcement ! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    print!("the longest string is {}", result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not found a '.'");

    let i = ImportantExcerpt {
        part: first_sentence
    };


}

// 这里会报错，因为不知道 返回值的生命周期是 x 还是 y
// fn longest (x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 

//  'a 标注生命周期  'a 生命周期是 x 和 y 中生命周期比较短的那个
fn longest <'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
