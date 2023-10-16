pub trait Iterator {
    // 关联类型  无需标注类型，无法为单个类型多次实现某个trait
    type Item;

    fn next(&mut self) -> Option<Self::Item> ;
}

struct Counter {

}

// 只能为 Counter 实现一次 Iterator
impl Iterator for Counter {
    // 指明关联类型
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}


// 同名方法调用
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot fly...");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard fly...");
    }
}
impl Human {
    fn fly (&self){
        println!("Human fly...");
    }
}

fn main() {
    let person = Human;
    // 同名方法调用
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);
}
