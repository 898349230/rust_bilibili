use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,20];
    let sores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    // 所有权
    let filed_name = String::from("Favorite color");
    let filed_value = String::from("Blue");

    let mut map = HashMap::new();
    // filed_name，filed_value 所有权给了 map
    // map.insert(filed_name, filed_value);
    // 会报错
    // println!("{}:{}", filed_name, filed_value);

    // 使用引用，所有权不会转移
    map.insert(&filed_name, &filed_value);
    // 会报错
    println!("{}:{}", filed_name, filed_value);


    // 获取值
    let key = String::from("Blue");
    let value = map.get(&key);
    match value {
        Some(s) => println!("value is {}", s),
        None => println!(" value is null "),
    }

    // 遍历 map 
    for (k, v) in &scores {
        println!("{} : {}", k, v);
    }

    let e = scores.entry(String::from("Yellow2"));
    println!("{:?}",e);
    // 如果 不存在 就插入, 返回元素的value
    let v = e.or_insert(50);
    println!("{:?}",v);

}
