
struct User {
    user_name: String,
    email: String,
    age: i32,
    user_actice: bool,
}

// 派生于Debug 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// 定义结构体方法
impl Rectangle {
    // 方法，第一个参数是自己
    fn area(&self) -> u32{
        self.length * self.width
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

//  关联函数
    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {

    let user1 = User {
        user_name: String::from("jenny"),
        age: 16,
        email: String::from("aaa163.com"),
        user_actice: true,
    };

    let user_name = user1.user_name;
    println!("user_name = {}", user_name);   
    
    // 如果声明的结构题是 mut ，那么所有结构体内的所有元素都是 mut 的

     let mut user3 = User {
        user_name: String::from("jenny"),
        age: 16,
        email: String::from("aaa163.com"),
          user_actice: true,
    };

    user3.user_name = String::from("danny");
    let user3_name = user3.user_name;
    println!("user3_name = {}", user3_name);


//  赋值
    let user2 = User {
        user_name: String::from("lily"),
        age: 18,
        // 复用 user1的 email 和 user_actice 属性
        ..user1
    };
    println!("user2 user_name = {}", user2.user_name);

    let w = 30;
    let h = 50;
    println!("area = {}", area(w, h));
    
    let rec = (30,40);
    println!("area2 = {}", area2(rec));

    let rectangle = Rectangle { 
        width: 50, 
        length: 80,
    };
    println!("area3 = {}", area3(&rectangle));


    println!("{:#?}", rectangle);

    println!("rectangle.area() = {}", rectangle.area());

    let rectangle2 = Rectangle { 
        width: 30, 
        length: 70,
    };

    println!("can_hold : {}", rectangle.can_hold(&rectangle2));

// 关联函数
    let square1 = Rectangle::square(4);
    println!("square1 {:?}", square1);

}

fn area(width: i32, height: i32) -> i32{
    width * height
}

fn area2(dim: (i32, i32)) -> i32{
    dim.0 * dim.1
}

fn area3(rec: &Rectangle) -> u32{
    rec.length * rec.width
}

