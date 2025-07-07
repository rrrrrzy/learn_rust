// 10_modules_and_packages.rs
// Rust 模块系统和包管理详解

// 注意：这个文件演示模块概念，但实际的包管理需要多文件结构

// ========== 模块定义 ==========

// 内联模块
mod math {
    // 公共函数
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 私有函数（默认）
    fn multiply_by_two(x: i32) -> i32 {
        x * 2
    }
    
    // 公共函数，使用私有函数
    pub fn double_add(a: i32, b: i32) -> i32 {
        multiply_by_two(add(a, b))
    }
    
    // 嵌套模块
    pub mod advanced {
        pub fn power(base: i32, exp: u32) -> i32 {
            base.pow(exp)
        }
        
        pub fn factorial(n: u32) -> u32 {
            if n <= 1 {
                1
            } else {
                n * factorial(n - 1)
            }
        }
        
        // 使用父模块的函数
        pub fn power_sum(a: i32, b: i32, exp: u32) -> i32 {
            super::add(
                power(a, exp),
                power(b, exp)
            )
        }
    }
}

// 结构体和枚举的模块
mod shapes {
    #[derive(Debug)]
    pub struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Rectangle {
        // 公共构造函数
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }
        
        // 公共方法
        pub fn area(&self) -> f64 {
            self.width * self.height
        }
        
        // 私有方法
        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
        
        // 公共方法调用私有方法
        pub fn info(&self) -> String {
            format!(
                "Rectangle: {}x{}, Area: {:.2}, Perimeter: {:.2}",
                self.width,
                self.height,
                self.area(),
                self.perimeter()
            )
        }
    }
    
    #[derive(Debug)]
    pub enum Shape {
        Rectangle(Rectangle),
        Circle { radius: f64 },
    }
    
    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Rectangle(rect) => rect.area(),
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            }
        }
    }
    
    // 公共常量
    pub const DEFAULT_SIZE: f64 = 10.0;
}

// 工具模块
mod utils {
    pub mod string_utils {
        pub fn reverse_string(s: &str) -> String {
            s.chars().rev().collect()
        }
        
        pub fn count_words(s: &str) -> usize {
            s.split_whitespace().count()
        }
        
        pub fn to_title_case(s: &str) -> String {
            s.split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
    
    pub mod number_utils {
        pub fn is_even(n: i32) -> bool {
            n % 2 == 0
        }
        
        pub fn is_prime(n: u32) -> bool {
            if n < 2 {
                return false;
            }
            for i in 2..=(n as f64).sqrt() as u32 {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
        
        pub fn fibonacci(n: u32) -> u32 {
            match n {
                0 => 0,
                1 => 1,
                _ => fibonacci(n - 1) + fibonacci(n - 2),
            }
        }
    }
}

// 使用use语句引入
use math::advanced;
use shapes::{Rectangle, Shape};
use utils::string_utils as str_utils;
use utils::number_utils;

// 也可以使用glob导入（谨慎使用）
// use utils::string_utils::*;

fn main() {
    println!("=== Rust 模块系统和包管理详解 ===\n");
    
    // ========== 基本模块使用 ==========
    println!("--- 基本模块使用 ---");
    
    // 直接调用模块函数
    let sum = math::add(5, 3);
    println!("5 + 3 = {}", sum);
    
    let double_sum = math::double_add(5, 3);
    println!("(5 + 3) * 2 = {}", double_sum);
    
    // 调用嵌套模块函数
    let power_result = math::advanced::power(2, 3);
    println!("2^3 = {}", power_result);
    
    let factorial_result = math::advanced::factorial(5);
    println!("5! = {}", factorial_result);
    
    // ========== 使用use简化调用 ==========
    println!("\n--- 使用use简化调用 ---");
    
    // 使用导入的模块
    let power_sum = advanced::power_sum(2, 3, 2);
    println!("2^2 + 3^2 = {}", power_sum);
    
    // ========== 结构体和枚举模块 ==========
    println!("\n--- 结构体和枚举模块 ---");
    
    // 使用导入的Rectangle
    let rect = Rectangle::new(5.0, 3.0);
    println!("矩形信息: {}", rect.info());
    
    // 使用Shape枚举
    let rectangle_shape = Shape::Rectangle(rect);
    let circle_shape = Shape::Circle { radius: 2.5 };
    
    println!("矩形面积: {:.2}", rectangle_shape.area());
    println!("圆形面积: {:.2}", circle_shape.area());
    
    // 使用模块常量
    let default_rect = Rectangle::new(shapes::DEFAULT_SIZE, shapes::DEFAULT_SIZE);
    println!("默认矩形: {}", default_rect.info());
    
    // ========== 工具模块使用 ==========
    println!("\n--- 工具模块使用 ---");
    
    let text = "hello rust world";
    
    // 使用字符串工具（通过别名）
    let reversed = str_utils::reverse_string(text);
    let word_count = str_utils::count_words(text);
    let title_case = str_utils::to_title_case(text);
    
    println!("原文: {}", text);
    println!("反转: {}", reversed);
    println!("单词数: {}", word_count);
    println!("标题格式: {}", title_case);
    
    // 使用数字工具
    let numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    
    println!("数字分析:");
    for &num in &numbers {
        println!(
            "  {}: 偶数={}, 质数={}, 斐波那契第{}项={}",
            num,
            number_utils::is_even(num),
            number_utils::is_prime(num as u32),
            num,
            if num <= 20 { number_utils::fibonacci(num as u32) } else { 0 }
        );
    }
    
    // ========== 模块可见性演示 ==========
    println!("\n--- 模块可见性演示 ---");
    
    demonstrate_visibility();
    
    // ========== 路径和作用域 ==========
    println!("\n--- 路径和作用域 ---");
    
    // 绝对路径（从crate根开始）
    let abs_result = crate::math::add(10, 20);
    println!("绝对路径调用: {}", abs_result);
    
    // 相对路径
    let rel_result = math::add(15, 25);
    println!("相对路径调用: {}", rel_result);
    
    // self 关键字（当前模块）
    // 在这里没有实际意义，因为我们在main函数中
    
    // super 关键字在嵌套模块中使用（见advanced模块中的例子）
    
    // ========== 重新导出 ==========
    println!("\n--- 重新导出演示 ---");
    
    // 演示pub use重新导出
    pub_use_example();
    
    // ========== 条件编译 ==========
    println!("\n--- 条件编译 ---");
    
    #[cfg(debug_assertions)]
    println!("这是调试版本");
    
    #[cfg(not(debug_assertions))]
    println!("这是发布版本");
    
    // 平台特定代码
    #[cfg(target_os = "windows")]
    println!("运行在Windows上");
    
    #[cfg(target_os = "linux")]
    println!("运行在Linux上");
    
    #[cfg(target_os = "macos")]
    println!("运行在macOS上");
    
    println!("\n=== 模块系统要点总结 ===");
    println!("1. 模块用mod关键字定义，默认私有");
    println!("2. pub关键字使项目变为公共");
    println!("3. use语句导入路径到作用域");
    println!("4. 路径可以是绝对的（从crate根）或相对的");
    println!("5. super和self关键字用于相对路径导航");
    println!("6. pub use可以重新导出项目");
    print_package_info();
}

// ========== 演示函数 ==========

fn demonstrate_visibility() {
    // 这些调用展示了什么是可访问的，什么是不可访问的
    
    // 可以访问公共函数
    let _ = math::add(1, 2);
    let _ = math::double_add(1, 2);
    
    // 不能访问私有函数
    // let _ = math::multiply_by_two(5); // 编译错误
    
    // 可以访问嵌套模块的公共项
    let _ = math::advanced::power(2, 3);
    
    println!("可见性演示完成");
}

// 重新导出演示
mod api {
    // 重新导出外部模块的项目
    pub use crate::math::add;
    pub use crate::shapes::{Rectangle, Shape};
    
    // 创建便利函数
    pub fn quick_rectangle(size: f64) -> Rectangle {
        Rectangle::new(size, size)
    }
}

fn pub_use_example() {
    // 通过重新导出的路径使用
    let result = api::add(5, 7);
    let rect = api::quick_rectangle(10.0);
    
    println!("通过重新导出: {} + 7 = {}", 5, result);
    println!("快速矩形: {}", rect.info());
}

// 打印包信息
fn print_package_info() {
    println!("\n--- 包管理系统信息 ---");
    println!("1. Cargo.toml - 包配置文件");
    println!("2. src/main.rs - 二进制包的入口");
    println!("3. src/lib.rs - 库包的入口");
    println!("4. 外部依赖通过Cargo.toml添加");
    println!("5. cargo build/run - 构建和运行");
    println!("6. cargo test - 运行测试");
    println!("7. cargo doc - 生成文档");
    
    // 在实际项目中，你可以这样使用外部crate：
    // use serde::{Serialize, Deserialize};
    // use tokio::runtime::Runtime;
    // use clap::{App, Arg};
}

// 如果这是一个库crate (lib.rs)，你可能会这样重新导出：
/*
pub use math::{add, double_add};
pub use math::advanced::{power, factorial};
pub use shapes::{Rectangle, Shape};
pub use utils::string_utils;
pub use utils::number_utils;
*/

// 测试模块（通常在单独的文件中）
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_math_add() {
        assert_eq!(math::add(2, 3), 5);
    }
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
    }
    
    #[test]
    fn test_string_utils() {
        assert_eq!(str_utils::reverse_string("hello"), "olleh");
        assert_eq!(str_utils::count_words("hello world"), 2);
    }
}
