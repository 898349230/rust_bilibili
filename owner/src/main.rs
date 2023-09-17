fn main() {

    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    let s2 = s1;
    // 这里 s1 会报错，s1 已经被移动
    // println!("{}", s1);

    println!("s2 = {}", s2);

// 深拷贝
    let s3 = s2.clone();
    println!("s3 = {}", s3);

    let some_string = String::from("hhhhh");

// 执行完后 some_string 会被销毁， 存放在heap上，所有权转移到 函数中
    take_ownership(some_string);

    let some_number = 100;
    // some_number 不会被销毁
    makes_copy(some_number);
    println!("some_number {}", some_number);

    let s1 = give_ownership();
    let s2 = take_and_gives_back(s1);
    println!("take_and_gives_back s2 = {}", s2);

//  传递引用（借用）， 这个引用是不可变的
    let len = calculate_length(&s2);
    println!("len = {}", len);

// 可变的引用, 特定作用域内，对某一款数据 只能有一个可变引用
// 不可以同时拥有一个可变引用和一个不变的引用， 多个可变的引用是可以的
    let mut s11 = String::from("可变的引用");
    let len = calculate_length2(&mut s11);

    println!("可变引用 {}", len);

    // 切片
    let mut word = String::from("hello wwwww");
    let word_index = first_word(&word);
    println!("word_index = {}", word_index);

    let first_word = first_word2(&word);
    println!("first_word = {}", first_word);

    let str = "hello2 world";
    let first_word2 = first_word3(str);
    println!("first_word2 = {}", first_word2);

    let hello_w = String::from("hello world");
    let hello2 = &hello_w[0..5];
    let hello = &hello_w[..5];
    let world = &hello_w[6..11];
    let world2 = &hello_w[6..];

    let whole = &hello_w[..];

    println!("切片 = {} {}, {} {}, {}", hello, world, hello2, world2, whole);


}

fn take_ownership(some_string: String){
    println!("some_string: {}", some_string);
}

fn makes_copy(some_number: i32){
    println!("some_number: {}", some_number)
}
// 赋予所有权
fn give_ownership() -> String {
    let some_string = String::from("aaaaaaaa");
    some_string
}
// 获取所有权然后返回
fn take_and_gives_back(a_string: String) -> String {
    a_string
}

// 参数为引用, 引用允许引用某些值，但是不获取所有权， 函数执行完不会清理引用指向的数据
fn calculate_length(s: &String) -> usize {
    s.len()
}
// 可变的引用
fn calculate_length2(s: &mut String) -> usize {
    s.push_str("hahaha");
    s.len()
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// 返回 切片（&str）
fn first_word2(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 切片作为参数
fn first_word3(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}