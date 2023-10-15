    use crate::List::{Cons, Nil};
use std::ops::Deref;

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T ;
//  返回引用
    fn deref(&self) -> &T {
        &self.0
    }
}


struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {
    // 释放资源, 离开作用域时调用
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    // 5 存在 heap 上， 
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3,
                Box::new(Nil))))));
                

    let x = 5;
    let y = &x;
    // Box 像引用一样使用
    let z = Box::new(x);
    let d = MyBox::new(x);
    assert_eq!(5,x);
    // * 解引用操作
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *d);
    // *(d.deref())

    let a = CustomSmartPoint{data: String::from("my stuff")};
    // 使用 drop() 函数提前调用 a.drop()
    drop(a);
    let b = CustomSmartPoint{data: String::from("other stuff")};

    println!("CustomSmartPointer creat.");

}

enum List {
    // Box<T> 是一个指针 提供了“间接”存储和heap内存分配的功能，没有其他额外功能，没有性能开销
    // 适用于需要“间接”存储的场景，如 Cons， 实现了 Deref trait 和 Drop trait
    Cons(i32, Box<List>),
    Nil,
}
