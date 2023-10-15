use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {

    // Rc<T> 需要再heap上分配数据，这些数据被程序的多个部分读取（只读），但在编译时无法确定
    // 哪个部分最后使用完这些数据， Rc<T> 只能用于单线程场景
    // 使单个值有多个使用者， 示例  a list 被 b list 和 c list 共同使用
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("强引用计数 {}", Rc::strong_count(&a));
    // clone() 方法增加引用计数，不会执行数据的深度拷贝
    let b = Cons(3, Rc::clone(&a));
    println!("强引用计数 {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("强引用计数 {}", Rc::strong_count(&a));
    } // c 作用域结束， 引用计数 -1 
    println!("强引用计数 {}", Rc::strong_count(&a));


}

    