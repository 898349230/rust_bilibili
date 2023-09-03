// module 模块
mod front_of_house {
    // 子模块
   pub mod hosting {
    // 设置为 pub 让父级模块或者其他可以调用
        pub fn add_to_waitList () {}
        // 默认是私有的
        fn seat_at_table () {}
    }

    mod serving {
        fn take_order () {}
    }

    fn fun () {
        // 使用 super 调用上级模块
        super::eat_at_restaurant();
        // 绝对路径调用
        crate::eat_at_restaurant();
    }

// 公有的
    pub struct Breakfast {
        // 公有的
        pub toast: String,
        // 默认私有的
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches") 
            }
        }
    }

    // 枚举是pub的，默认里元素都是pub的
    pub enum Appletizer {
        Soup,
        Salad,
    }


//  使用 use 关键字引用
    pub mod use_test {
        pub fn test_method1(){}
        fn test_method2(){}
    }    
    pub mod use_test2 {
        pub fn test_method1(){}
        fn test_method2(){}
    }

}

pub fn eat_at_restaurant () {
    // 绝对路径调用， 同级私有模块可以调用
    crate::front_of_house::hosting::add_to_waitList();
    //  相对路径调用  同级私有模块可以调用
    front_of_house::hosting::add_to_waitList();

    let mut meal = front_of_house::Breakfast::summer("Rye");
    // 修改,  toast 是 pub 的，这里可以访问
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

//   seasonal_fruit 默认是私有的 访问不到
    // meal.seasonal_fruit = String::from("wheat");

}

// use 导出 front_of_house 模块， 引入的默认是私有的
use crate::front_of_house::use_test;
// 使用 pub 导出，可以被外部使用
// pub use crate::front_of_house::use_test;
// 使用 as 别名
use crate::front_of_house::use_test2 as user_test2;


pub fn user_test(){
    // 直接使用 use 导出的 pub mod
    use_test::test_method1();
    // 私有的不可以使用
    // use_test::test_method2();
}
