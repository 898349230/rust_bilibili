use std::io;
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1, 100);


    println!("输入的数字为 {}", secret_number);

    loop {
        println!("猜测一个数");

        //  可变变量
            let mut guess = String::new();
        
            io::stdin().read_line(&mut guess).expect("无法继续读行");
        
            println!("你猜测的数是：{}", guess);
        
        //  shadow
            // let guess: u32 = guess.trim().parse().expect("please type a number!");

        // 处理错误
            let guess: u32 = match guess.trim().parse() {
                // 没有异常 返回生成的数
                Ok(num) => num,
                // 发生一次  下一次
                Err(_) => continue,
            };
        
        //  match 匹配
            match guess.cmp(&secret_number){
                Ordering::Less => println!("too small"),
                Ordering::Greater => println!("too big"),
                Ordering::Equal => {println!("you win"); break;},
            }
    }


}
