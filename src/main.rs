fn main() {
   
}

// 声明接口
pub trait Behavior {
    fn run(&self);
    fn swim(&self);
}

// 实现接口
pub struct Dog<'a> {
    pub name: &'a str, // 使用 &str 类型，要求指定生命周期
}

impl<'a> Dog<'a> {
    // 创建一个新的 Dog 实例
    fn new(name: &'a str) -> Self {
        Dog { name }
    }
}

impl<'a> Behavior for Dog<'a> {
    fn run(&self) {
        println!("{} running ...", self.name);
    }

    fn swim(&self) {
        println!("{} swimming ...", self.name); // 注意这里加了分号
    }
}