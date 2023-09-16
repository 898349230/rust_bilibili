enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

//   声明 vector 里的值
    // let v: Vec<i32> = Vec::new();

    // 初始化值
    // let v = Vec![1,2,3];

// 赋值 改变 vector 元素， 使用 mut 可变
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

// 索引获取值
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
// 使用 get() 方法获取值
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("there is no third element"),
    }

    println!("遍历 vector。。。。。");
    for i in &v {
        println!("i = {}", i);

    }

    println!("修改 vector。。。。。");
    let mut v = vec![100,50,66,5];
    for i in &mut v {
        // * 解引用
        *i += 100;
    }

    for i in &v {
        println!("i = {}", i);
    }


    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(10.4),
        SpreadsheetCell::Text(String::from("blue")),
    ];

}
