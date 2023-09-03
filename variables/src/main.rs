fn main() {

// 整数类型

// 浮点类型  f32  f64 (默认类型)
let x = 2.0;
let y:f32 = 3.4;
println!("x={},y={}",x,y);
// 布尔

// 字符 使用单引号 四个字节

// 元组 tuple 
let tup: (i32, f64, u8) = (33, 5.4,1);
let (x,y,z) = tup;
println!("{},{},{}", x,y,z);

println!("{},{},{}", tup.0,tup.1,tup.2);

// 数组
 let arr  = [1,2,3];
println!("{}", arr[1]);
another_function(88);
}

fn another_function(i: i32){

    println!("another_function {}", i)

}

