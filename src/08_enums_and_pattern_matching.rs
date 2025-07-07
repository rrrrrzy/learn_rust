// 08_enums_and_pattern_matching.rs
// Rust 枚举与模式匹配详解

use std::fmt;

// 基本枚举定义
#[derive(Debug, PartialEq)]
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 复杂枚举：不同变体可以有不同类型和数量的关联数据
#[derive(Debug)]
enum Message {
    Quit,                       // 没有关联数据
    Move { x: i32, y: i32 },    // 命名字段
    Write(String),              // 单个String
    ChangeColor(i32, i32, i32), // 三个i32
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
    
    fn message_type(&self) -> &str {
        match self {
            Message::Quit => "Quit",
            Message::Move { .. } => "Move",
            Message::Write(_) => "Write",
            Message::ChangeColor(_, _, _) => "ChangeColor",
        }
    }
}

// 自定义Option类型示例
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

// 自定义Result类型示例
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// 状态机枚举
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    
    fn duration_seconds(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

// 复杂枚举：表示表达式
#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(left, right) => left.evaluate() + right.evaluate(),
            Expr::Subtract(left, right) => left.evaluate() - right.evaluate(),
            Expr::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Expr::Divide(left, right) => left.evaluate() / right.evaluate(),
        }
    }
}

// 自定义Display trait
impl fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrafficLight::Red => write!(f, "🔴 红灯"),
            TrafficLight::Yellow => write!(f, "🟡 黄灯"),
            TrafficLight::Green => write!(f, "🟢 绿灯"),
        }
    }
}

fn main() {
    println!("=== Rust 枚举与模式匹配详解 ===\n");
    
    // ========== 基本枚举使用 ==========
    println!("--- 基本枚举使用 ---");
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IP类型: {:?}, {:?}", four, six);
    
    // 枚举作为函数参数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    // ========== 带数据的枚举 ==========
    println!("\n--- 带数据的枚举 ---");
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("IPv4地址: {:?}", home);
    println!("IPv6地址: {:?}", loopback);
    
    // ========== 复杂枚举 ==========
    println!("\n--- 复杂枚举 ---");
    
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    let messages = vec![msg1, msg2, msg3, msg4];
    
    for msg in &messages {
        println!("消息类型: {}", msg.message_type());
        msg.call();
    }
    
    // ========== Option 枚举 ==========
    println!("\n--- Option 枚举 ---");
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("有值的数字: {:?}", some_number);
    println!("有值的字符串: {:?}", some_string);
    println!("空值: {:?}", absent_number);
    
    // Option 的常用方法
    if let Some(value) = some_number {
        println!("提取的值: {}", value);
    }
    
    let doubled = some_number.map(|x| x * 2);
    println!("映射后: {:?}", doubled);
    
    let default_value = absent_number.unwrap_or(0);
    println!("默认值: {}", default_value);
    
    // ========== match 表达式 ==========
    println!("\n--- match 表达式 ---");
    
    // 基本匹配
    let coin = Coin::Quarter(UsState::Alaska);
    println!("硬币价值: {} 美分", value_in_cents(&coin));
    
    // 匹配Option
    let x = Some(5);
    let y = None;
    
    println!("x + 1 = {:?}", plus_one(x));
    println!("y + 1 = {:?}", plus_one(y));
    
    // 匹配数字
    let number = 13;
    match number {
        1 => println!("一"),
        3 | 5 | 7 | 9 | 11 | 13 => println!("奇数且小于15"),
        2 | 4 | 6 | 8 | 10 | 12 | 14 => println!("偶数且小于15"),
        15..=19 => println!("15到19之间"),
        _ => println!("其他数字"),
    }
    
    // ========== if let 控制流 ==========
    println!("\n--- if let 控制流 ---");
    
    let some_u8_value = Some(3);
    
    // 使用match的方式
    match some_u8_value {
        Some(3) => println!("match: 找到3"),
        _ => (),
    }
    
    // 使用if let的简洁方式
    if let Some(3) = some_u8_value {
        println!("if let: 找到3");
    }
    
    // if let 与 else
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("州区硬币来自 {:?}!", state);
    } else {
        count += 1;
        println!("非州区硬币，计数: {}", count);
    }
    
    // ========== while let 循环 ==========
    println!("\n--- while let 循环 ---");
    
    let mut stack = vec![1, 2, 3];
    
    while let Some(top) = stack.pop() {
        println!("从栈中弹出: {}", top);
    }
    
    // ========== 模式匹配的高级用法 ==========
    println!("\n--- 高级模式匹配 ---");
    
    // 解构结构体
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    match person {
        Person { name, age: 30 } => println!("30岁的人: {}", name),
        Person { name, age } => println!("其他年龄的人: {}, 年龄: {}", name, age),
    }
    
    // 解构枚举
    let msg = Message::Move { x: 100, y: 200 };
    match msg {
        Message::Move { x, y } => println!("移动消息: x={}, y={}", x, y),
        _ => println!("其他消息"),
    }
    
    // 匹配守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于5的数: {}", x),
        Some(x) => println!("大于等于5的数: {}", x),
        None => println!("没有数字"),
    }
    
    // ========== 状态机示例 ==========
    println!("\n--- 状态机示例 ---");
    
    let mut light = TrafficLight::Red;
    println!("当前状态: {}, 持续时间: {}秒", light, light.duration_seconds());
    
    for _ in 0..5 {
        light = light.next();
        println!("下一状态: {}, 持续时间: {}秒", light, light.duration_seconds());
    }
    
    // ========== 表达式计算器 ==========
    println!("\n--- 表达式计算器 ---");
    
    // 构建表达式: (10 + 5) * 2
    let expr = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(10.0)),
            Box::new(Expr::Number(5.0)),
        )),
        Box::new(Expr::Number(2.0)),
    );
    
    println!("表达式: {:?}", expr);
    println!("计算结果: {}", expr.evaluate());
    
    // ========== Result 类型示例 ==========
    println!("\n--- Result 类型示例 ---");
    
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出错了");
    
    match success {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    match failure {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    // ========== 实用示例：配置解析 ==========
    println!("\n--- 配置解析示例 ---");
    
    let configs = vec![
        Config::Database { 
            host: String::from("localhost"), 
            port: 5432 
        },
        Config::Cache { 
            size_mb: 256 
        },
        Config::Logging { 
            level: LogLevel::Info 
        },
    ];
    
    for config in configs {
        process_config(config);
    }
    
    println!("\n=== 枚举与模式匹配要点总结 ===");
    println!("1. 枚举定义一组可能的值，每个变体可以有不同的数据");
    println!("2. match表达式必须穷尽所有可能的模式");
    println!("3. if let和while let提供了简洁的模式匹配语法");
    println!("4. 模式匹配支持解构、守卫、范围等高级特性");
    println!("5. Option和Result是Rust中最重要的枚举类型");
}

// ========== 辅助定义和函数 ==========

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    // ... 更多州
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
enum Config {
    Database { host: String, port: u32 },
    Cache { size_mb: u32 },
    Logging { level: LogLevel },
}

// 路由函数
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("处理IPv4地址"),
        IpAddrKind::V6 => println!("处理IPv6地址"),
    }
}

// 硬币价值函数
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士！");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("州区硬币来自 {:?}!", state);
            25
        },
    }
}

// Option处理函数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 配置处理函数
fn process_config(config: Config) {
    match config {
        Config::Database { host, port } => {
            println!("配置数据库: {}:{}", host, port);
        },
        Config::Cache { size_mb } => {
            println!("配置缓存: {}MB", size_mb);
        },
        Config::Logging { level } => {
            println!("配置日志级别: {:?}", level);
        },
    }
}

// 分析消息的函数
fn analyze_message(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("应用程序退出"),
        Message::Move { x, y } => format!("移动到位置 ({}, {})", x, y),
        Message::Write(text) => format!("显示文本: {}", text),
        Message::ChangeColor(r, g, b) => format!("更改颜色为 RGB({}, {}, {})", r, g, b),
    }
}

// 处理可选值的函数
fn handle_optional(opt: Option<i32>) -> i32 {
    match opt {
        Some(value) if value > 10 => value * 2,
        Some(value) => value,
        None => 0,
    }
}
