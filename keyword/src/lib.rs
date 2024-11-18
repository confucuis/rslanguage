/// 关键字示例
use mlib;

// 模块mod
mod m_module {
    // 对外不可见
    fn example_a() {
        println!("hello world");
    }

    // 对外可以见
    pub fn example_b() {
        println!("hello world");
    }

    // crate之内可见,crate之外不可见
    pub(crate) fn example_c() {
        println!("hello world");
    }

    // 只能在当前模块的父模块可见
    pub(super) fn example_d() {
        println!("hello world");
    }
}

