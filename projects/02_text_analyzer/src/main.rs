// 文本分析工具 - Rust 函数和控制流练习项目
// 学习目标：函数设计、控制流、字符串处理、基础所有权

use std::collections::HashMap;
use std::fs;
use std::io;

// 字符类别枚举（练习枚举和模式匹配）
#[derive(Debug, PartialEq)]
enum CharCategory {
    Uppercase,
    Lowercase,
    Digit,
    Punctuation,
    Whitespace,
    Other,
}

// 文本统计结构体（练习结构体定义）
#[derive(Debug, Default)]
struct TextStats {
    total_chars: usize,
    total_words: usize,
    total_lines: usize,
    total_sentences: usize,
    char_categories: HashMap<CharCategory, usize>,
    word_frequencies: HashMap<String, usize>,
    longest_word: String,
    shortest_word: String,
    average_word_length: f64,
}

impl TextStats {
    // 创建新的统计实例
    fn new() -> Self {
        Self {
            char_categories: HashMap::new(),
            word_frequencies: HashMap::new(),
            ..Default::default()
        }
    }
    
    // 显示统计结果
    fn display(&self) {
        println!("\n=== 文本分析结果 ===");
        println!("字符总数: {}", self.total_chars);
        println!("单词总数: {}", self.total_words);
        println!("行数总数: {}", self.total_lines);
        println!("句子总数: {}", self.total_sentences);
        println!("平均单词长度: {:.2}", self.average_word_length);
        
        if !self.longest_word.is_empty() {
            println!("最长单词: {} ({} 字符)", self.longest_word, self.longest_word.len());
        }
        
        if !self.shortest_word.is_empty() {
            println!("最短单词: {} ({} 字符)", self.shortest_word, self.shortest_word.len());
        }
        
        println!("\n=== 字符分类统计 ===");
        for (category, count) in &self.char_categories {
            println!("{:?}: {}", category, count);
        }
        
        println!("\n=== 高频词汇 (前10个) ===");
        let mut word_vec: Vec<_> = self.word_frequencies.iter().collect();
        word_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        for (word, frequency) in word_vec.iter().take(10) {
            println!("{}: {} 次", word, frequency);
        }
    }
}

fn main() {
    println!("=== Rust 文本分析工具 ===");
    println!("这是一个练习函数、控制流和字符串处理的项目\n");
    
    // 显示菜单
    show_menu();
    
    // 主循环
    loop {
        print!("\n请选择操作 > ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("读取输入失败");
        
        let choice = input.trim();
        
        match choice {
            "1" => analyze_user_input(),
            "2" => analyze_file(),
            "3" => demonstrate_functions(),
            "4" => demonstrate_control_flow(),
            "5" => {
                println!("感谢使用文本分析工具！");
                break;
            },
            "help" | "h" => show_menu(),
            _ => println!("无效选择，请输入 1-5 或 help"),
        }
    }
}

// 显示菜单（练习函数定义）
fn show_menu() {
    println!("=== 功能菜单 ===");
    println!("1. 分析用户输入的文本");
    println!("2. 分析文件内容");
    println!("3. 演示函数特性");
    println!("4. 演示控制流");
    println!("5. 退出程序");
    println!("help/h. 显示菜单");
}

// 分析用户输入的文本
fn analyze_user_input() {
    println!("\n请输入要分析的文本（输入 'END' 结束）:");
    
    let mut text = String::new();
    
    // 使用 loop 读取多行输入
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("读取输入失败");
        
        if line.trim() == "END" {
            break;
        }
        
        text.push_str(&line);
    }
    
    if text.trim().is_empty() {
        println!("没有输入任何文本");
        return;
    }
    
    // 分析文本并显示结果
    let stats = analyze_text(&text);
    stats.display();
}

// 分析文件内容
fn analyze_file() {
    print!("请输入文件路径: ");
    
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)
        .expect("读取输入失败");
    
    let file_path = file_path.trim();
    
    // 尝试读取文件（练习错误处理）
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("文件读取成功，开始分析...");
            let stats = analyze_text(&content);
            stats.display();
        },
        Err(error) => {
            println!("文件读取失败: {}", error);
        }
    }
}

// 主要的文本分析函数（练习借用和所有权）
fn analyze_text(text: &str) -> TextStats {
    let mut stats = TextStats::new();
    
    // 统计总字符数
    stats.total_chars = text.chars().count();
    
    // 统计行数
    stats.total_lines = count_lines(text);
    
    // 统计句子数
    stats.total_sentences = count_sentences(text);
    
    // 分析字符类别
    analyze_characters(text, &mut stats);
    
    // 分析单词
    analyze_words(text, &mut stats);
    
    stats
}

// 统计行数（练习简单函数）
fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

// 统计句子数（练习字符处理）
fn count_sentences(text: &str) -> usize {
    let mut count = 0;
    
    for ch in text.chars() {
        // 使用 match 进行模式匹配
        match ch {
            '.' | '!' | '?' => count += 1,
            _ => continue,
        }
    }
    
    count
}

// 分析字符类别（练习字符分类和HashMap）
fn analyze_characters(text: &str, stats: &mut TextStats) {
    for ch in text.chars() {
        let category = categorize_character(ch);
        
        // 更新字符类别计数
        let count = stats.char_categories.entry(category).or_insert(0);
        *count += 1;
    }
}

// 字符分类函数（练习模式匹配）
fn categorize_character(ch: char) -> CharCategory {
    match ch {
        'a'..='z' => CharCategory::Lowercase,
        'A'..='Z' => CharCategory::Uppercase,
        '0'..='9' => CharCategory::Digit,
        ' ' | '\t' | '\n' | '\r' => CharCategory::Whitespace,
        '.' | ',' | '!' | '?' | ';' | ':' | '"' | '\'' => CharCategory::Punctuation,
        _ => CharCategory::Other,
    }
}

// 分析单词（练习字符串处理和迭代）
fn analyze_words(text: &str, stats: &mut TextStats) {
    let words: Vec<String> = extract_words(text);
    
    if words.is_empty() {
        return;
    }
    
    stats.total_words = words.len();
    
    // 计算平均单词长度
    let total_length: usize = words.iter().map(|w| w.len()).sum();
    stats.average_word_length = total_length as f64 / words.len() as f64;
    
    // 找最长和最短单词
    find_extremes(&words, stats);
    
    // 计算词频
    calculate_word_frequencies(&words, stats);
}

// 提取单词（练习字符串分割和过滤）
fn extract_words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| clean_word(word))
        .filter(|word| !word.is_empty())
        .collect()
}

// 清洗单词（移除标点符号）
fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|ch| ch.is_alphabetic())
        .collect::<String>()
        .to_lowercase()
}

// 找到最长和最短单词（练习迭代器和比较）
fn find_extremes(words: &[String], stats: &mut TextStats) {
    if let Some(first_word) = words.first() {
        stats.longest_word = first_word.clone();
        stats.shortest_word = first_word.clone();
        
        for word in words {
            if word.len() > stats.longest_word.len() {
                stats.longest_word = word.clone();
            }
            
            if word.len() < stats.shortest_word.len() {
                stats.shortest_word = word.clone();
            }
        }
    }
}

// 计算单词频率（练习HashMap操作）
fn calculate_word_frequencies(words: &[String], stats: &mut TextStats) {
    for word in words {
        let count = stats.word_frequencies.entry(word.clone()).or_insert(0);
        *count += 1;
    }
}

// 演示函数特性
fn demonstrate_functions() {
    println!("\n=== 函数特性演示 ===");
    
    // 无参数函数
    print_separator();
    
    // 有参数函数
    let test_text = "Hello, Rust! This is a test.";
    println!("测试文本: \"{}\"", test_text);
    
    // 返回值函数
    let word_count = count_words(test_text);
    println!("单词数量: {}", word_count);
    
    // 借用参数
    let char_count = count_characters(test_text);
    println!("字符数量: {}", char_count);
    
    // 所有权转移
    let owned_text = String::from(test_text);
    let processed = process_text_owned(owned_text);
    println!("处理后的文本: \"{}\"", processed);
    
    // 可变借用
    let mut mutable_text = String::from("original");
    modify_text(&mut mutable_text);
    println!("修改后的文本: \"{}\"", mutable_text);
    
    print_separator();
}

// 演示控制流
fn demonstrate_control_flow() {
    println!("\n=== 控制流演示 ===");
    
    // if/else 演示
    let number = 42;
    if number > 50 {
        println!("数字大于50");
    } else if number > 30 {
        println!("数字在30到50之间");
    } else {
        println!("数字小于等于30");
    }
    
    // match 演示
    let category = categorize_number(number);
    match category {
        "small" => println!("这是一个小数字"),
        "medium" => println!("这是一个中等数字"),
        "large" => println!("这是一个大数字"),
        _ => println!("未知类别"),
    }
    
    // loop 演示
    println!("使用 loop 计数到 5:");
    let mut counter = 0;
    loop {
        counter += 1;
        print!("{} ", counter);
        
        if counter >= 5 {
            break;
        }
    }
    println!();
    
    // while 演示
    println!("使用 while 倒数从 5:");
    let mut countdown = 5;
    while countdown > 0 {
        print!("{} ", countdown);
        countdown -= 1;
    }
    println!();
    
    // for 演示
    println!("使用 for 遍历数组:");
    let numbers = [1, 2, 3, 4, 5];
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();
    
    // for range 演示
    println!("使用 for range 范围:");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    print_separator();
}

// 辅助函数：无参数函数示例
fn print_separator() {
    println!("----------------------------------------");
}

// 辅助函数：简单计数函数
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// 辅助函数：字符计数
fn count_characters(text: &str) -> usize {
    text.chars().count()
}

// 辅助函数：获取所有权的函数
fn process_text_owned(mut text: String) -> String {
    text.push_str(" [已处理]");
    text
}

// 辅助函数：可变借用
fn modify_text(text: &mut String) {
    text.push_str(" [已修改]");
}

// 辅助函数：数字分类
fn categorize_number(num: i32) -> &'static str {
    if num < 10 {
        "small"
    } else if num < 100 {
        "medium"
    } else {
        "large"
    }
}
