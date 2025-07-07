// 01_variables_and_mutability.rs
// Rust 变量和可变性详解

fn main() {
    // 1. 不可变变量（默认）
    let x = 5;
    println!("不可变变量 x 的值: {}", x);
    
    // 以下代码会编译错误，因为 x 是不可变的
    // x = 6; // 错误！不能重新赋值给不可变变量
    
    // 2. 可变变量
    let mut y = 10;
    println!("可变变量 y 的初始值: {}", y);
    y = 20; // 可以修改可变变量的值
    println!("修改后的 y: {}", y);
    
    // 3. 常量（const）
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS: {}", MAX_POINTS);
    
    // 4. 变量遮蔽（Shadowing）
    let z = 5;
    let z = z + 1; // 创建了一个新的变量 z，遮蔽了之前的 z
    {
        let z = z * 2; // 在内部作用域中再次遮蔽 z
        println!("内部作用域中的 z: {}", z); // 输出 12
    }
    println!("外部作用域中的 z: {}", z); // 输出 6
    
    // 5. 遮蔽可以改变变量类型
    let spaces = "   ";
    let spaces = spaces.len(); // 从字符串类型变为数字类型
    println!("字符串中的空格数: {}", spaces);
    
    // 6. 变量声明但不初始化
    let uninitialized: i32;
    uninitialized = 42; // 必须在使用前初始化
    println!("延迟初始化的变量: {}", uninitialized);
    
    // 7. 解构赋值
    let (a, b, c) = (1, 2, "fa");
    println!("解构赋值: a = {}, b = {}, c = {}", a, b, c);
    
    // 8. 可变性与借用
    let mut name = String::from("Alice");
    println!("原始名称: {}", name);
    
    // 修改可变变量
    name.push_str(" Smith");
    println!("修改后的名称: {}", name);
    
    // 9. 静态变量
    static GLOBAL_VAR: i32 = 100;
    println!("静态变量: {}", GLOBAL_VAR);
    
    println!("\n=== 变量和可变性要点总结 ===");
    println!("1. Rust 变量默认是不可变的，需要 mut 关键字才能修改");
    println!("2. 常量使用 const 声明，必须指定类型，编译时确定值");
    println!("3. 变量遮蔽允许重新声明同名变量，甚至可以改变类型");
    println!("4. 静态变量在整个程序运行期间都存在");
}
