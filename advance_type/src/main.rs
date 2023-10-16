// 类型别名，主要作用是减少重复代码
type Kilometers = i32;
fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let answer = do_twice(add_one, 5);
    println!("answer = {}", answer);

}

fn add_one(x: i32) -> i32 {
    x + 1
}
// 函数作为参数
fn do_twice(f: fn(i32) -> i32, arg : i32) -> i32 {
    f(arg) + f(arg)
}

