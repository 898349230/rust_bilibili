
unsafe fn dangerous(){

}
// 可变的静态变量
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32){
    // 访问和修改可变的静态变量是不安全的操作
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    
}

unsafe impl Foo for i32 {
    
} 

fn main() {

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }


    let mut num = 5;
    // 不可变原始指针
    let r1 = &num as *const i32;
    // 可变原始指针
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1: {}", *r1);
        println!("r1: {}", *r2);
        // unsafe 代码块内调用 unsafe 代码块， 只有 unsafe 内可以调用 unsafe
        dangerous();
    }
    

    println!("Hello, world!");
}
