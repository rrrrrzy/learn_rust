// 11_error_handling.rs
// Rust 错误处理详解

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::num::ParseIntError;
use std::fmt;

// 自定义错误类型
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

// 为自定义错误实现Display trait
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "不能除以零"),
            MathError::NegativeSquareRoot => write!(f, "不能计算负数的平方根"),
            MathError::Overflow => write!(f, "计算溢出"),
        }
    }
}

// 实现Error trait
impl std::error::Error for MathError {}

// 自定义用户错误
#[derive(Debug)]
struct UserError {
    code: u32,
    message: String,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "错误代码 {}: {}", self.code, self.message)
    }
}

impl std::error::Error for UserError {}

fn main() {
    println!("=== Rust 错误处理详解 ===\n");
    
    // ========== panic! 宏 ==========
    println!("--- panic! 宏 ---");
    
    // 演示会导致panic的情况（注释掉以免程序崩溃）
    // panic!("这是一个故意的panic！");
    
    // 数组越界会导致panic
    let vec = vec![1, 2, 3];
    // let element = vec[99]; // 这会panic
    
    // 安全的数组访问
    match vec.get(2) {
        Some(value) => println!("获取到值: {}", value),
        None => println!("索引超出范围"),
    }
    
    // ========== Result 类型 ==========
    println!("\n--- Result 类型 ---");
    
    // 基本Result处理
    let result1: Result<i32, &str> = Ok(42);
    let result2: Result<i32, &str> = Err("出错了");
    
    match result1 {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    match result2 {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    // ========== 文件操作错误处理 ==========
    println!("\n--- 文件操作错误处理 ---");
    
    // 方法1: 使用match处理
    let file_result = File::open("nonexistent.txt");
    match file_result {
        Ok(_file) => println!("文件打开成功"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("文件不存在: {}", error),
            ErrorKind::PermissionDenied => println!("权限被拒绝: {}", error),
            other_error => println!("其他错误: {:?}", other_error),
        }
    }
    
    // 方法2: 使用unwrap_or_else
    let _file = File::open("nonexistent.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("文件不存在，创建新文件");
            File::create("nonexistent.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件时出现问题: {:?}", error);
        }
    });
    
    // ========== Result 的便捷方法 ==========
    println!("\n--- Result 的便捷方法 ---");
    
    let good_result: Result<i32, &str> = Ok(10);
    let bad_result: Result<i32, &str> = Err("错误");
    
    // unwrap - 获取Ok值或panic
    let value = good_result.unwrap();
    println!("unwrap值: {}", value);
    
    // expect - 带自定义消息的unwrap
    // let value = bad_result.expect("这是自定义错误消息"); // 会panic
    
    // unwrap_or - 提供默认值
    let value = bad_result.unwrap_or(0);
    println!("unwrap_or默认值: {}", value);
    
    // unwrap_or_else - 使用闭包提供默认值
    let value = bad_result.unwrap_or_else(|_| {
        println!("使用闭包处理错误");
        -1
    });
    println!("unwrap_or_else值: {}", value);
    
    // is_ok 和 is_err
    println!("good_result是否成功: {}", good_result.is_ok());
    println!("bad_result是否错误: {}", bad_result.is_err());
    
    // ========== ? 操作符 ==========
    println!("\n--- ? 操作符 ---");
    
    // 演示?操作符的使用
    match read_file_content() {
        Ok(content) => println!("文件内容长度: {}", content.len()),
        Err(error) => println!("读取文件错误: {}", error),
    }
    
    match parse_and_double("42") {
        Ok(result) => println!("解析并翻倍: {}", result),
        Err(error) => println!("解析错误: {}", error),
    }
    
    // ========== 自定义错误 ==========
    println!("\n--- 自定义错误 ---");
    
    // 使用自定义MathError
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 ÷ 2 = {}", result),
        Err(error) => println!("除法错误: {}", error),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 ÷ 0 = {}", result),
        Err(error) => println!("除法错误: {}", error),
    }
    
    match safe_sqrt(-4.0) {
        Ok(result) => println!("√(-4) = {}", result),
        Err(error) => println!("平方根错误: {}", error),
    }
    
    // 使用自定义UserError
    match validate_user("") {
        Ok(user) => println!("用户验证成功: {}", user),
        Err(error) => println!("用户验证失败: {}", error),
    }
    
    // ========== 错误传播 ==========
    println!("\n--- 错误传播 ---");
    
    match process_file("test.txt") {
        Ok(result) => println!("文件处理结果: {}", result),
        Err(error) => println!("文件处理错误: {}", error),
    }
    
    // ========== Option 类型 ==========
    println!("\n--- Option 类型 ---");
    
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    // 基本Option处理
    match some_value {
        Some(value) => println!("有值: {}", value),
        None => println!("无值"),
    }
    
    // Option的便捷方法
    println!("some_value是否有值: {}", some_value.is_some());
    println!("none_value是否为空: {}", none_value.is_none());
    
    // unwrap_or
    let value = none_value.unwrap_or(0);
    println!("Option默认值: {}", value);
    
    // map方法
    let doubled = some_value.map(|x| x * 2);
    println!("Option映射: {:?}", doubled);
    
    // and_then方法（扁平化映射）
    let result = some_value.and_then(|x| {
        if x > 0 {
            Some(x * 2)
        } else {
            None
        }
    });
    println!("Option扁平化映射: {:?}", result);
    
    // ========== 组合多个Result ==========
    println!("\n--- 组合多个Result ---");
    
    match multiply_strings("3", "4") {
        Ok(result) => println!("字符串相乘: {}", result),
        Err(error) => println!("字符串相乘错误: {}", error),
    }
    
    match multiply_strings("3", "abc") {
        Ok(result) => println!("字符串相乘: {}", result),
        Err(error) => println!("字符串相乘错误: {}", error),
    }
    
    // ========== 实际应用示例 ==========
    println!("\n--- 实际应用示例 ---");
    
    // 配置文件解析示例
    let config_data = "timeout=30\nretries=3\ndebug=true";
    match parse_config(config_data) {
        Ok(config) => println!("配置解析成功: {:?}", config),
        Err(error) => println!("配置解析失败: {}", error),
    }
    
    // 网络请求模拟
    match simulate_network_request("https://api.example.com") {
        Ok(response) => println!("网络请求成功: {}", response),
        Err(error) => println!("网络请求失败: {}", error),
    }
    
    // ========== 恢复机制 ==========
    println!("\n--- 错误恢复机制 ---");
    
    let numbers = vec!["1", "2", "abc", "4", "xyz", "6"];
    let parsed_numbers: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse().ok()) // 忽略解析错误
        .collect();
    
    println!("成功解析的数字: {:?}", parsed_numbers);
    
    // 收集所有错误
    let results: Vec<Result<i32, _>> = numbers
        .iter()
        .map(|s| s.parse())
        .collect();
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("索引{}: 成功解析 {}", i, value),
            Err(error) => println!("索引{}: 解析失败 {}", i, error),
        }
    }
    
    println!("\n=== 错误处理要点总结 ===");
    println!("1. panic!用于不可恢复的错误");
    println!("2. Result<T, E>用于可恢复的错误");
    println!("3. Option<T>用于可能为空的值");
    println!("4. ?操作符简化错误传播");
    println!("5. 自定义错误类型提供更好的错误信息");
    println!("6. 错误处理是显式的，不能被忽略");
}

// ========== 辅助函数 ==========

// 使用?操作符的函数
fn read_file_content() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 链式?操作符
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let number = s.parse::<i32>()?;
    Ok(number * 2)
}

// 自定义错误函数
fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn validate_user(name: &str) -> Result<String, UserError> {
    if name.is_empty() {
        Err(UserError {
            code: 1001,
            message: "用户名不能为空".to_string(),
        })
    } else if name.len() < 3 {
        Err(UserError {
            code: 1002,
            message: "用户名至少需要3个字符".to_string(),
        })
    } else {
        Ok(format!("用户: {}", name))
    }
}

// 错误传播示例
fn process_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    if contents.is_empty() {
        return Err(Box::new(UserError {
            code: 2001,
            message: "文件为空".to_string(),
        }));
    }
    
    Ok(format!("文件内容长度: {}", contents.len()))
}

// 组合多个Result
fn multiply_strings(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
    let num1 = s1.parse::<i32>()?;
    let num2 = s2.parse::<i32>()?;
    Ok(num1 * num2)
}

// 配置解析示例
#[derive(Debug)]
struct Config {
    timeout: u32,
    retries: u32,
    debug: bool,
}

fn parse_config(data: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut timeout = 0;
    let mut retries = 0;
    let mut debug = false;
    
    for line in data.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            return Err(Box::new(UserError {
                code: 3001,
                message: format!("无效的配置行: {}", line),
            }));
        }
        
        match parts[0] {
            "timeout" => timeout = parts[1].parse()?,
            "retries" => retries = parts[1].parse()?,
            "debug" => debug = parts[1].parse()?,
            _ => return Err(Box::new(UserError {
                code: 3002,
                message: format!("未知的配置项: {}", parts[0]),
            })),
        }
    }
    
    Ok(Config { timeout, retries, debug })
}

// 网络请求模拟
fn simulate_network_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 模拟各种网络错误
    if url.is_empty() {
        return Err(Box::new(UserError {
            code: 4001,
            message: "URL不能为空".to_string(),
        }));
    }
    
    if !url.starts_with("https://") {
        return Err(Box::new(UserError {
            code: 4002,
            message: "只支持HTTPS协议".to_string(),
        }));
    }
    
    // 模拟成功响应
    Ok(format!("来自 {} 的响应数据", url))
}

// 多种错误类型的处理
fn complex_operation(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // 先验证输入
    if input.is_empty() {
        return Err(Box::new(UserError {
            code: 5001,
            message: "输入不能为空".to_string(),
        }));
    }
    
    // 解析数字
    let number = input.parse::<i32>()?;
    
    // 数学运算
    let result = safe_sqrt(number as f64)?;
    
    Ok(result as i32)
}
