enum IpAddrKind {
    V4,
    V6,
}
// 枚举内嵌入其他类型
enum IpAddrKind2 {
    V4(u8,u8,u8,u8),
    V6(String),
}
// 枚举定义方法
impl IpAddrKind {
    fn me1(&self){

    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // 嵌套枚举
    Quarter(UsState),
}

fn main() {
    let four = IpAddrKind::V4;
    route(four);

    let four2 = IpAddrKind2::V4(192,168,0,1);
    let six2 = IpAddrKind2::V6(String::from("::1"));

    // println!("four2:{}, six2:{}", four2, six2);

//  模式匹配
    let m = value_in_cents(Coin::Nickel);
    println!("value_in_cents = {}", m);

    let m2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    
    println!("value_in_cents = {}", m2);

// Option 枚举
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);




    let v = Some(0u8);
    match v {
        Some(1) => println!("one") ,
        Some(2) => println!("two") ,
        Some(3) => println!("three") ,
        _ => (),
    }

//  if let  只匹配一种模式，放弃穷举可能
    if let Some(2) = v {
        println!("two");
    }    
    
    if let Some(2) = v {
        println!("two");
    } else {
        println!("other");
    }

}

fn route(ip_kind: IpAddrKind) {

}

fn value_in_cents(coin: Coin) -> u8 {
    // 模式匹配, 必须穷举所有匹配的， 或者使用通配符  _
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Nickel ...");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(usState) => {
            println!(" State is {:#?}", usState);
            5
        },
        // 通配符
        _ => {100},
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}