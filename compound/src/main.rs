fn main() {
    another_function(6);
    println!("five {}", five_plus(6));

    let num = 5;
    if num < 5 {
        println!(" 小于5 ");

    } else if num == 5 {
        println!(" 等于5 ");

    } else {
        println!(" 大于5 ");

    }

    let condition = true;
    let number = if condition {5} else {6};
    println!(" number = {} ", number);


    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result = {}", result);

    let mut number = 10;
    while number != 0 {
        println!("the number = {}!", number);
        number -= 1;
    }

    let a = [0,1,2,3,4,5];

    for el in a.iter() {
        println!("el = {}", el);
    }

    for number in (1..4).rev() {
        println!("number = {}", number);
    }

}

// 函数
fn another_function(i: i32){

    println!("another_function {}", i)

}
// 又返回值的函数
fn five_plus(x: i32) -> i32{
    // 没有分号，是一个表达式，最后一个表达式是返回值
    5 + x
}