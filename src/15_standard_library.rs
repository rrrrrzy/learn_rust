// 15_standard_library.rs
// Rust 常用标准库函数与实用宏详解

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::path::Path;
use std::env;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== Rust 标准库函数与实用宏详解 ===\n");
    
    // ========== 基础宏 ==========
    println!("--- 基础宏 ---");
    
    // println! 系列宏
    println!("基本输出");
    println!("格式化输出: {}", 42);
    println!("多个参数: {} 和 {}", "hello", "world");
    println!("命名参数: {name} 今年 {age} 岁", name="Alice", age=30);
    println!("调试输出: {:?}", vec![1, 2, 3]);
    println!("美化调试: {:#?}", vec![1, 2, 3]);
    
    // print! 和 eprint! 系列
    print!("不换行输出 ");
    print!("继续输出\n");
    
    eprintln!("错误输出到stderr");
    
    // format! 宏
    let formatted = format!("格式化字符串: {}", 42);
    println!("{}", formatted);
    
    // panic! 宏
    // panic!("这会导致程序崩溃"); // 注释掉以免程序终止
    
    // ========== 断言宏 ==========
    println!("\n--- 断言宏 ---");
    
    // assert! 宏
    assert!(2 + 2 == 4);
    assert!(true, "这个断言应该通过");
    
    // assert_eq! 和 assert_ne! 宏
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
    
    // debug_assert! 只在调试模式下启用
    debug_assert!(2 + 2 == 4);
    
    println!("所有断言都通过了");
    
    // ========== 向量和集合宏 ==========
    println!("\n--- 向量和集合宏 ---");
    
    // vec! 宏
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![0; 5]; // 创建包含5个0的向量
    
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    
    // 自定义宏示例（创建HashMap）
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("HashMap: {:?}", map);
    
    // ========== 字符串处理 ==========
    println!("\n--- 字符串处理 ---");
    
    let text = "Hello, World! This is Rust programming.";
    
    // 字符串方法
    println!("原文: {}", text);
    println!("长度: {}", text.len());
    println!("字符数: {}", text.chars().count());
    println!("是否包含'Rust': {}", text.contains("Rust"));
    println!("以'Hello'开头: {}", text.starts_with("Hello"));
    println!("以'.'结尾: {}", text.ends_with("."));
    
    // 字符串分割
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("单词: {:?}", words);
    
    let parts: Vec<&str> = text.split(',').collect();
    println!("按逗号分割: {:?}", parts);
    
    // 字符串替换
    let replaced = text.replace("World", "Rust");
    println!("替换后: {}", replaced);
    
    // 字符串转换
    let uppercase = text.to_uppercase();
    let lowercase = text.to_lowercase();
    println!("大写: {}", uppercase);
    println!("小写: {}", lowercase);
    
    // 字符串修剪
    let padded = "  hello world  ";
    println!("修剪前: '{}'", padded);
    println!("修剪后: '{}'", padded.trim());
    
    // ========== 数字处理 ==========
    println!("\n--- 数字处理 ---");
    
    let num = 42;
    println!("数字: {}", num);
    println!("绝对值: {}", num.abs());
    println!("最大值: {}", num.max(50));
    println!("最小值: {}", num.min(30));
    
    let float_num = 3.14159;
    println!("浮点数: {}", float_num);
    println!("向上取整: {}", float_num.ceil());
    println!("向下取整: {}", float_num.floor());
    println!("四舍五入: {}", float_num.round());
    println!("保留2位小数: {:.2}", float_num);
    
    // 数字转换
    let string_num = "123";
    match string_num.parse::<i32>() {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
    
    // ========== 迭代器 ==========
    println!("\n--- 迭代器 ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 基本迭代
    println!("原数组: {:?}", numbers);
    
    // map 操作
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("翻倍: {:?}", doubled);
    
    // filter 操作
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // fold 操作
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("累加和: {}", sum);
    
    // reduce 操作
    let product = numbers.iter().reduce(|acc, x| acc * x);
    println!("累乘积: {:?}", product);
    
    // 链式操作
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > 3)
        .map(|x| x * x)
        .collect();
    println!("大于3的数的平方: {:?}", result);
    
    // find 操作
    let found = numbers.iter().find(|&&x| x > 5);
    println!("第一个大于5的数: {:?}", found);
    
    // any 和 all
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("包含偶数: {}", has_even);
    println!("全部为正数: {}", all_positive);
    
    // ========== 文件操作 ==========
    println!("\n--- 文件操作 ---");
    
    // 写入文件
    let filename = "test_file.txt";
    let content = "这是测试内容\n第二行\n第三行";
    
    match std::fs::write(filename, content) {
        Ok(_) => println!("文件写入成功"),
        Err(e) => println!("文件写入失败: {}", e),
    }
    
    // 读取文件
    match std::fs::read_to_string(filename) {
        Ok(content) => {
            println!("文件内容:");
            println!("{}", content);
        }
        Err(e) => println!("文件读取失败: {}", e),
    }
    
    // 检查文件是否存在
    if Path::new(filename).exists() {
        println!("文件存在");
        
        // 获取文件元数据
        match std::fs::metadata(filename) {
            Ok(metadata) => {
                println!("文件大小: {} 字节", metadata.len());
                println!("是否为目录: {}", metadata.is_dir());
                println!("是否为文件: {}", metadata.is_file());
            }
            Err(e) => println!("获取元数据失败: {}", e),
        }
    }
    
    // 删除文件
    match std::fs::remove_file(filename) {
        Ok(_) => println!("文件删除成功"),
        Err(e) => println!("文件删除失败: {}", e),
    }
    
    // ========== 环境变量 ==========
    println!("\n--- 环境变量 ---");
    
    // 获取环境变量
    match env::var("PATH") {
        Ok(path) => println!("PATH环境变量长度: {}", path.len()),
        Err(e) => println!("获取PATH失败: {}", e),
    }
    
    // 设置环境变量
    env::set_var("MY_VAR", "my_value");
    println!("MY_VAR: {:?}", env::var("MY_VAR"));
    
    // 获取所有环境变量
    let env_count = env::vars().count();
    println!("环境变量总数: {}", env_count);
    
    // 获取当前目录
    match env::current_dir() {
        Ok(path) => println!("当前目录: {}", path.display()),
        Err(e) => println!("获取当前目录失败: {}", e),
    }
    
    // 获取程序参数
    let args: Vec<String> = env::args().collect();
    println!("程序参数: {:?}", args);
    
    // ========== 时间处理 ==========
    println!("\n--- 时间处理 ---");
    
    // 计时器
    let start = Instant::now();
    
    // 模拟一些工作
    let sum: i32 = (1..1000000).sum();
    
    let duration = start.elapsed();
    println!("计算耗时: {:?}", duration);
    println!("计算结果: {}", sum);
    
    // 睡眠
    println!("睡眠1秒...");
    thread::sleep(Duration::from_secs(1));
    println!("睡眠结束");
    
    // ========== 错误处理辅助 ==========
    println!("\n--- 错误处理辅助 ---");
    
    // unwrap_or 系列
    let maybe_number: Option<i32> = Some(42);
    let maybe_none: Option<i32> = None;
    
    println!("unwrap_or: {}", maybe_none.unwrap_or(0));
    println!("unwrap_or_else: {}", maybe_none.unwrap_or_else(|| 99));
    
    // map_or 系列
    println!("map_or: {}", maybe_number.map_or(0, |x| x * 2));
    println!("map_or_else: {}", maybe_none.map_or_else(|| 0, |x| x * 2));
    
    // and_then 链式操作
    let result = maybe_number
        .and_then(|x| if x > 0 { Some(x * 2) } else { None })
        .and_then(|x| if x < 100 { Some(x + 10) } else { None });
    println!("链式操作结果: {:?}", result);
    
    // ========== 线程和并发 ==========
    println!("\n--- 线程和并发 ---");
    
    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("线程输出: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 等待线程完成
    handle.join().unwrap();
    
    // 共享状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数: {}", *counter.lock().unwrap());
    
    // ========== 实用工具函数 ==========
    println!("\n--- 实用工具函数 ---");
    
    demonstrate_utility_functions();
    
    // ========== 自定义宏示例 ==========
    println!("\n--- 自定义宏示例 ---");
    
    // 使用自定义宏
    let my_vec = create_vec![1, 2, 3, 4, 5];
    println!("自定义向量宏: {:?}", my_vec);
    
    // 条件打印宏
    let debug_mode = true;
    debug_print!(debug_mode, "这是调试信息");
    
    // 重复宏
    let repeated = repeat_expr!(5 + 3; 3);
    println!("重复表达式: {:?}", repeated);
    
    println!("\n=== 标准库要点总结 ===");
    println!("1. 宏提供了代码生成和元编程能力");
    println!("2. 迭代器提供了函数式编程风格");
    println!("3. 标准库涵盖了文件、网络、线程等常用功能");
    println!("4. Option和Result类型提供了安全的错误处理");
    println!("5. 集合类型提供了高效的数据结构");
    println!("6. 时间和环境API提供了系统交互能力");
}

// ========== 实用函数演示 ==========

fn demonstrate_utility_functions() {
    println!("实用函数演示:");
    
    // 数组操作
    let mut arr = [3, 1, 4, 1, 5, 9, 2, 6];
    println!("  原数组: {:?}", arr);
    
    arr.sort();
    println!("  排序后: {:?}", arr);
    
    arr.reverse();
    println!("  反转后: {:?}", arr);
    
    // 字符串操作
    let text = "hello,world,rust";
    let parts: Vec<&str> = text.split(',').collect();
    println!("  分割结果: {:?}", parts);
    
    let joined = parts.join(" | ");
    println!("  重新连接: {}", joined);
    
    // 数学运算
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    
    println!("  数字: {:?}", numbers);
    println!("  和: {}", sum);
    println!("  积: {}", product);
    println!("  最大值: {:?}", max);
    println!("  最小值: {:?}", min);
    
    // 类型转换
    let int_vec = vec![1, 2, 3, 4, 5];
    let float_vec: Vec<f64> = int_vec.iter().map(|&x| x as f64).collect();
    println!("  整数转浮点: {:?}", float_vec);
    
    // 去重
    let duplicates = vec![1, 2, 2, 3, 3, 3, 4, 5];
    let mut unique = duplicates.clone();
    unique.sort();
    unique.dedup();
    println!("  去重前: {:?}", duplicates);
    println!("  去重后: {:?}", unique);
}

// ========== 自定义宏定义 ==========

// 创建向量的宏
macro_rules! create_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 条件调试打印宏
macro_rules! debug_print {
    ($condition:expr, $($arg:tt)*) => {
        if $condition {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

// 重复表达式宏
macro_rules! repeat_expr {
    ($expr:expr; $n:expr) => {
        {
            let mut vec = Vec::new();
            for _ in 0..$n {
                vec.push($expr);
            }
            vec
        }
    };
}

// 创建HashMap的宏
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// ========== 高级标准库使用 ==========

fn advanced_stdlib_usage() {
    // 使用BTreeMap进行有序存储
    let mut btree = BTreeMap::new();
    btree.insert("c", 3);
    btree.insert("a", 1);
    btree.insert("b", 2);
    
    println!("BTreeMap (有序): {:?}", btree);
    
    // 使用VecDeque作为双端队列
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_front(0);
    deque.push_back(2);
    
    println!("VecDeque: {:?}", deque);
    
    // 使用HashSet进行集合操作
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    let union: HashSet<_> = set1.union(&set2).collect();
    
    println!("交集: {:?}", intersection);
    println!("并集: {:?}", union);
}

// ========== 字符串处理工具 ==========

fn string_processing_tools() {
    let text = "  Hello, Beautiful World!  ";
    
    // 链式字符串处理
    let processed = text
        .trim()
        .to_lowercase()
        .replace(",", "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-");
    
    println!("处理后的字符串: {}", processed);
    
    // 字符串格式化
    let name = "Alice";
    let age = 30;
    let formatted = format!("姓名: {}, 年龄: {}", name, age);
    println!("格式化: {}", formatted);
    
    // 字符串解析
    let numbers_str = "1,2,3,4,5";
    let numbers: Result<Vec<i32>, _> = numbers_str
        .split(',')
        .map(|s| s.parse::<i32>())
        .collect();
    
    match numbers {
        Ok(nums) => println!("解析的数字: {:?}", nums),
        Err(e) => println!("解析错误: {}", e),
    }
}

// ========== 错误处理模式 ==========

fn error_handling_patterns() {
    // 使用 ? 操作符
    fn read_file_content(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(filename)?;
        Ok(content.trim().to_string())
    }
    
    // 错误映射
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse().map_err(|_| format!("无法解析 '{}' 为数字", s))
    }
    
    // 使用示例
    match parse_number("42") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
    
    match parse_number("abc") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
}
