use std::{thread, time::Duration};

struct Cacher<T> 
    where T: Fn(u32) -> u32 
    {
        //   calculation 是一个闭包啊
        calculation: T,
        value: Option<u32>,
    }

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }

    fn value (&mut self, arg: u32) -> u32{
        match self.value {
            // 有值 直接返回
            Some(v) => v,
            // 无值，执行闭包，并且设置 value为闭包执行的结果
            None => {
                let v = (self.calculation)(arg);
                v
            }
        }
    }
}

fn main() {

    let x = vec![1,2,3];
    // move 会将 x 的所有权转移到 闭包中，强制闭包取得他所使用的环境值的所有权
    let equal_to_x =  move |z: Vec<u32>| {z == x} ;
    // println!("can not use x here {:?}", x);

    generate_workout(12, 6);
}


fn generate_workout(intensity: u32, randon_number: u32){

    // 闭包
    let mut expensive_closure = Cacher::new(|num|{
            println!("caculating slow ...");  
            thread::sleep(Duration::from_secs(2));
            num
        }
    );
     
    if intensity < 25 {
        println!("Today do {} pushups", expensive_closure.value(intensity));
        println!("Next do {} situps", expensive_closure.value(intensity));
    }else {
        if randon_number == 3 {
            println!("take a break today");
        }else {
            println!("Today run for {} minutes", expensive_closure.value(intensity));
        }
    }

}

fn generate_workout2(intensity: u32, randon_number: u32){

    // 闭包
    let expensive_closure = |num|{
        println!("caculating slow ...");  
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today do {} pushups", expensive_closure(intensity));
        println!("Next do {} situps", expensive_closure(intensity));
    }else {
        if randon_number == 3 {
            println!("take a break today");
        }else {
            println!("Today run for {} minutes", expensive_closure(intensity));
        }
    }

}
