// 智能计算器 - Rust 基础语法练习项目
// 学习目标：变量、数据类型、控制流、错误处理

use std::io;
use std::collections::HashMap;

// 计算器的主要状态
struct Calculator {
    current_value: f64,           // 当前值（可变）
    memory: HashMap<String, f64>, // 变量存储（练习HashMap）
    history: Vec<String>,         // 计算历史（练习Vec）
}

impl Calculator {
    // 创建新的计算器实例
    fn new() -> Self {
        Calculator {
            current_value: 0.0,
            memory: HashMap::new(),
            history: Vec::new(),
        }
    }
    
    // 执行基本运算
    fn calculate(&mut self, operator: char, operand: f64) -> Result<f64, String> {
        let result = match operator {
            '+' => self.current_value + operand,
            '-' => self.current_value - operand,
            '*' => self.current_value * operand,
            '/' => {
                if operand == 0.0 {
                    return Err("错误：除零操作！".to_string());
                }
                self.current_value / operand
            },
            '^' => self.current_value.powf(operand),
            _ => return Err(format!("不支持的操作符: {}", operator)),
        };
        
        // 记录历史
        let operation = format!("{} {} {} = {}", 
            self.current_value, operator, operand, result);
        self.history.push(operation);
        
        self.current_value = result;
        Ok(result)
    }
    
    // 设置当前值
    fn set_value(&mut self, value: f64) {
        self.current_value = value;
    }
    
    // 获取当前值
    fn get_value(&self) -> f64 {
        self.current_value
    }
    
    // 存储变量
    fn store_variable(&mut self, name: String, value: f64) {
        self.memory.insert(name, value);
    }
    
    // 获取变量值
    fn get_variable(&self, name: &str) -> Option<f64> {
        self.memory.get(name).copied()
    }
    
    // 显示历史记录
    fn show_history(&self) {
        if self.history.is_empty() {
            println!("没有计算历史");
            return;
        }
        
        println!("\n=== 计算历史 ===");
        for (index, operation) in self.history.iter().enumerate() {
            println!("{}: {}", index + 1, operation);
        }
        println!();
    }
    
    // 清除历史
    fn clear_history(&mut self) {
        self.history.clear();
        println!("历史记录已清除");
    }
    
    // 显示所有变量
    fn show_variables(&self) {
        if self.memory.is_empty() {
            println!("没有存储的变量");
            return;
        }
        
        println!("\n=== 存储的变量 ===");
        for (name, value) in &self.memory {
            println!("{} = {}", name, value);
        }
        println!();
    }
}

fn main() {
    println!("=== Rust 智能计算器 ===");
    println!("这是一个练习 Rust 基础语法的项目");
    println!("支持基本运算、变量存储、历史记录等功能\n");
    
    // 创建计算器实例（练习结构体）
    let mut calculator = Calculator::new();
    
    // 显示帮助信息
    show_help();
    
    // 主循环
    loop {
        print!("\n当前值: {} > ", calculator.get_value());
        
        // 读取用户输入
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("读取输入失败");
        
        let input = input.trim();
        
        // 处理特殊命令
        match input {
            "quit" | "q" | "exit" => {
                println!("感谢使用智能计算器！");
                break;
            },
            "help" | "h" => {
                show_help();
                continue;
            },
            "history" => {
                calculator.show_history();
                continue;
            },
            "clear_history" => {
                calculator.clear_history();
                continue;
            },
            "variables" | "vars" => {
                calculator.show_variables();
                continue;
            },
            "clear" | "c" => {
                calculator.set_value(0.0);
                println!("计算器已重置");
                continue;
            },
            "" => continue,
            _ => {
                // 处理计算表达式
                if let Err(error) = process_input(&mut calculator, input) {
                    println!("错误: {}", error);
                }
            }
        }
    }
}

// 处理用户输入的计算表达式
fn process_input(calculator: &mut Calculator, input: &str) -> Result<(), String> {
    // 检查是否是变量赋值 (例如: x = 42)
    if input.contains('=') && !input.contains("==") {
        return handle_variable_assignment(calculator, input);
    }
    
    // 检查是否是单独的数字或变量名
    if let Ok(number) = input.parse::<f64>() {
        calculator.set_value(number);
        println!("当前值设置为: {}", number);
        return Ok(());
    }
    
    // 检查是否是变量名
    if let Some(value) = calculator.get_variable(input) {
        calculator.set_value(value);
        println!("从变量 {} 加载值: {}", input, value);
        return Ok(());
    }
    
    // 解析运算表达式 (例如: + 5, * 2.5, ^ 3)
    parse_operation(calculator, input)
}

// 处理变量赋值
fn handle_variable_assignment(calculator: &mut Calculator, input: &str) -> Result<(), String> {
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() != 2 {
        return Err("变量赋值格式错误，应该是: 变量名 = 值".to_string());
    }
    
    let var_name = parts[0].trim().to_string();
    let value_str = parts[1].trim();
    
    // 尝试解析值
    let value = if let Ok(num) = value_str.parse::<f64>() {
        num
    } else if let Some(var_value) = calculator.get_variable(value_str) {
        var_value
    } else {
        return Err(format!("无法解析值: {}", value_str));
    };
    
    calculator.store_variable(var_name.clone(), value);
    println!("变量 {} 设置为: {}", var_name, value);
    Ok(())
}

// 解析运算操作
fn parse_operation(calculator: &mut Calculator, input: &str) -> Result<(), String> {
    let input = input.trim();
    
    // 检查输入格式
    if input.len() < 2 {
        return Err("输入格式错误，应该是: 操作符 数值 (例如: + 5)".to_string());
    }
    
    // 提取操作符
    let operator = input.chars().next().unwrap();
    let operand_str = &input[1..].trim();
    
    // 解析操作数
    let operand = if let Ok(num) = operand_str.parse::<f64>() {
        num
    } else if let Some(var_value) = calculator.get_variable(operand_str) {
        var_value
    } else {
        return Err(format!("无法解析操作数: {}", operand_str));
    };
    
    // 执行计算
    match calculator.calculate(operator, operand) {
        Ok(result) => {
            println!("结果: {}", result);
            Ok(())
        },
        Err(error) => Err(error),
    }
}

// 显示帮助信息
fn show_help() {
    println!("=== 帮助信息 ===");
    println!("基本运算:");
    println!("  + 数值    - 加法");
    println!("  - 数值    - 减法");
    println!("  * 数值    - 乘法");
    println!("  / 数值    - 除法");
    println!("  ^ 数值    - 幂运算");
    println!();
    println!("变量操作:");
    println!("  数值            - 设置当前值");
    println!("  变量名 = 数值   - 存储变量");
    println!("  变量名          - 加载变量值");
    println!();
    println!("命令:");
    println!("  help/h          - 显示帮助");
    println!("  history         - 显示计算历史");
    println!("  clear_history   - 清除历史");
    println!("  variables/vars  - 显示所有变量");
    println!("  clear/c         - 重置计算器");
    println!("  quit/q/exit     - 退出程序");
    println!();
}

// 演示函数：展示不同的数据类型使用
#[allow(dead_code)]
fn demonstrate_data_types() {
    println!("=== 数据类型演示 ===");
    
    // 整数类型演示
    let small_int: i8 = 127;
    let regular_int: i32 = 2_147_483_647;
    let big_int: i64 = 9_223_372_036_854_775_807;
    
    println!("整数类型: i8={}, i32={}, i64={}", small_int, regular_int, big_int);
    
    // 浮点类型演示
    let float_32: f32 = 3.14159;
    let float_64: f64 = 2.718281828459045;
    
    println!("浮点类型: f32={}, f64={}", float_32, float_64);
    
    // 类型转换演示
    let int_val = 42i32;
    let float_val = int_val as f64;
    println!("类型转换: {} (i32) -> {} (f64)", int_val, float_val);
    
    // 变量可变性演示
    let immutable_var = 10;
    println!("不可变变量: {}", immutable_var);
    
    let mut mutable_var = 20;
    println!("可变变量初始值: {}", mutable_var);
    mutable_var = 30;
    println!("可变变量修改后: {}", mutable_var);
    
    // 变量遮蔽演示
    let shadowed_var = 5;
    println!("原始变量: {}", shadowed_var);
    let shadowed_var = shadowed_var * 2;
    println!("遮蔽后的变量: {}", shadowed_var);
    let shadowed_var = "现在是字符串";
    println!("再次遮蔽（类型改变）: {}", shadowed_var);
}
