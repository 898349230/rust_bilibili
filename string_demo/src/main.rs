fn main() {

    let s1 = String::from("tic");
    let s2 = "tac".to_string();
    let s3 = String::from("toc");

    // let s3 = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s3);

// format 不会获得参数的所有权
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    let s4 = String::from("你a好哈哈abc");
    // 字符串切片
    let s5 = &s4[0..4];
    println!("{}", s5);

    for a in s4.chars() {
        println!("a = {}", a);
    }

    for b in s4.bytes() {
        println!("b = {}", b);
    }

}
