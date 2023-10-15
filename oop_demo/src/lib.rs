pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> 任意的实现了 Draw trait 
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for compinent in self.components.iter() {
            compinent.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮
    }
}
