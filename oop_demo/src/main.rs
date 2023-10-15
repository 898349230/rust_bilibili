use oop_demo::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制选择框
    }
}

fn main() {

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 55,
                height: 43,
                options: vec![
                    String::from("yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ],
    };
    screen.run();
    println!("Hello, world!");
}
