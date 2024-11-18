
//- 泛型函数
pub fn print_t<T>(arg: T) {
    println!(T)
}

//- 泛型结构体: 默认T类型为i32
pub struct Point<T = i32> {
    x: T,
    y: T,
}

// 实现泛型结构体
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
}

//- 内置泛型枚举: Option<T>, Result<T, E>

//- trait泛型约束
use std::cmp::Ord;

// 参数a, b必须实现Ord trait
pub fn max<'a, T: Ord>(a: &'a T, b: &'a T) {
    if a>b {
        a
    } else {
        b
    }
}
