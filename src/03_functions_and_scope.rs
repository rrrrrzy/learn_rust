// 03_functions_and_scope.rs
// Rust 函数与作用域详解

fn main() {
    println!("=== Rust 函数与作用域详解 ===\n");
    
    // ========== 函数基础 ==========
    println!("--- 函数基础 ---");
    
    // 调用无参数函数
    greet();
    
    // 调用有参数函数
    greet_person("Alice");
    
    // 调用有返回值的函数
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // 调用多返回值函数
    let (quotient, remainder) = divide(10, 3);
    println!("10 ÷ 3 = {} 余 {}", quotient, remainder);
    
    // ========== 函数表达式与语句 ==========
    println!("\n--- 表达式与语句 ---");
    
    // 表达式作为返回值
    let result = {
        let x = 3;
        x + 1 // 没有分号，这是一个表达式，作为块的返回值
    };
    println!("块表达式的结果: {}", result);
    
    // 函数调用是表达式
    let doubled = double(result);
    println!("双倍值: {}", doubled);
    
    // ========== 作用域 ==========
    println!("\n--- 作用域 ---");
    
    let outer_var = "外部变量";
    println!("外部作用域: {}", outer_var);
    
    {
        let inner_var = "内部变量";
        println!("内部作用域: {}", inner_var);
        println!("内部可以访问外部: {}", outer_var);
        
        // 变量遮蔽
        let outer_var = "被遮蔽的外部变量";
        println!("遮蔽后的外部变量: {}", outer_var);
    }
    
    // inner_var在这里不可访问
    // println!("{}", inner_var); // 编译错误
    println!("回到外部作用域: {}", outer_var); // 恢复原来的值
    
    // ========== 函数作为参数 ==========
    println!("\n--- 高阶函数 ---");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 传递函数作为参数
    let squared_sum = apply_operation(&numbers, square);
    println!("平方和: {}", squared_sum);
    
    let cubed_sum = apply_operation(&numbers, cube);
    println!("立方和: {}", cubed_sum);
    
    // ========== 闭包 ==========
    println!("\n--- 闭包 ---");
    
    let multiplier = 3;
    let multiply_by_3 = |x| x * multiplier; // 捕获外部变量的闭包
    
    println!("5 × 3 = {}", multiply_by_3(5));
    
    // 闭包作为函数参数
    let closure_sum = apply_closure(&numbers, |x| x * 2);
    println!("所有数字乘以2的和: {}", closure_sum);
    
    // ========== 递归函数 ==========
    println!("\n--- 递归函数 ---");
    
    let factorial_5 = factorial(5);
    println!("5的阶乘: {}", factorial_5);
    
    let fib_10 = fibonacci(10);
    println!("斐波那契数列第10项: {}", fib_10);
    
    // ========== 函数指针 ==========
    println!("\n--- 函数指针 ---");
    
    // 函数指针类型
    let operation: fn(i32, i32) -> i32 = add;
    println!("通过函数指针调用add: {}", operation(7, 3));
    
    // 函数指针数组
    let operations: [fn(i32, i32) -> i32; 3] = [add, subtract, multiply];
    for (i, op) in operations.iter().enumerate() {
        println!("操作{}: 10, 5 = {}", i, op(10, 5));
    }
}

// ========== 函数定义 ==========

// 无参数无返回值函数
fn greet() {
    println!("Hello, Rust!");
}

// 有参数无返回值函数
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// 有参数有返回值函数
fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式，没有分号
}

// 多返回值函数（使用元组）
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// 表达式函数
fn double(x: i32) -> i32 {
    x * 2
}

// 早返回
fn check_positive(x: i32) -> String {
    if x < 0 {
        return String::from("负数"); // 早返回
    }
    
    if x == 0 {
        return String::from("零");
    }
    
    String::from("正数")
}

// 高阶函数：接受函数作为参数
fn apply_operation(numbers: &[i32], operation: fn(i32) -> i32) -> i32 {
    numbers.iter().map(|&x| operation(x)).sum()
}

// 用于传递给高阶函数的函数
fn square(x: i32) -> i32 {
    x * x
}

fn cube(x: i32) -> i32 {
    x * x * x
}

// 接受闭包作为参数的函数
fn apply_closure<F>(numbers: &[i32], closure: F) -> i32 
where
    F: Fn(i32) -> i32,
{
    numbers.iter().map(|&x| closure(x)).sum()
}

// 递归函数
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 基本算术运算函数
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 泛型函数示例
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 带生命周期参数的函数
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
