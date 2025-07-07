// 14_lifetimes.rs
// Rust 生命周期详解

use std::fmt::Display;

fn main() {
    println!("=== Rust 生命周期详解 ===\n");
    
    // ========== 基本生命周期概念 ==========
    println!("--- 基本生命周期概念 ---");
    
    // 简单的引用生命周期
    {
        let x = 5;                    // ----------+-- 'a
        let r = &x;                   // --+-- 'b  |
        println!("r 引用的值: {}", r); //   |       |
    }                                 // --+       |
    // x 在这里超出作用域                      // ----------+
    
    // 生命周期省略规则演示
    let string1 = String::from("hello");
    let string2 = "world";
    
    let result = longest_elided(&string1, string2);
    println!("最长的字符串: {}", result);
    
    // ========== 显式生命周期注解 ==========
    println!("\n--- 显式生命周期注解 ---");
    
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("最长的字符串: {}", result);
    }
    
    // 生命周期确保安全性
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // 注意：这里result的生命周期由最短的输入决定
        println!("在作用域内使用结果: {}", result);
    }
    // 如果在这里使用result会编译错误，因为string2已经超出作用域
    // println!("作用域外使用结果: {}", result); // 编译错误
    
    // ========== 结构体中的生命周期 ==========
    println!("\n--- 结构体中的生命周期 ---");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {}", i.part);
    println!("重要程度: {}", i.level());
    
    // 使用带生命周期的结构体
    let announcement = "今天是个好日子";
    let excerpt_result = i.announce_and_return_part(announcement);
    println!("宣布并返回: {}", excerpt_result);
    
    // ========== 方法定义中的生命周期 ==========
    println!("\n--- 方法定义中的生命周期 ---");
    
    let text = "这是一个很长的文本，我们将从中提取第一个单词";
    let wrapper = StringWrapper::new(text);
    
    if let Some(first_word) = wrapper.first_word() {
        println!("第一个单词: {}", first_word);
    }
    
    let prefix = wrapper.get_prefix(5);
    println!("前5个字符: {}", prefix);
    
    // ========== 静态生命周期 ==========
    println!("\n--- 静态生命周期 ---");
    
    let s: &'static str = "I have a static lifetime.";
    println!("静态字符串: {}", s);
    
    // 静态生命周期的函数
    let static_str = get_static_str();
    println!("静态函数返回: {}", static_str);
    
    // ========== 生命周期子类型 ==========
    println!("\n--- 生命周期子类型 ---");
    
    let string1 = String::from("long string is long");
    let string2 = String::from("short");
    
    let result = choose_first(&string1, &string2);
    println!("选择第一个: {}", result);
    
    // ========== 生命周期与泛型 ==========
    println!("\n--- 生命周期与泛型 ---");
    
    let str_ref = "hello world";
    let holder = Holder::new(str_ref);
    println!("持有者内容: {}", holder.value);
    
    // 泛型与生命周期结合
    let number = 42;
    let generic_holder = GenericHolder::new(&number);
    println!("泛型持有者: {}", generic_holder.value);
    
    // ========== 高级生命周期模式 ==========
    println!("\n--- 高级生命周期模式 ---");
    
    // 多个生命周期参数
    let x = 10;
    let y = 20;
    let z = 30;
    
    let result = complex_function(&x, &y, &z);
    println!("复杂函数结果: {}", result);
    
    // 生命周期约束
    let data = vec![1, 2, 3, 4, 5];
    let processor = DataProcessor::new(&data);
    let sum = processor.sum();
    println!("数据处理器求和: {}", sum);
    
    // ========== 实际应用示例 ==========
    println!("\n--- 实际应用示例 ---");
    
    // 文本分析器
    let text = "Rust programming language is amazing and powerful";
    let analyzer = TextAnalyzer::new(text);
    
    println!("文本分析:");
    println!("  单词数量: {}", analyzer.word_count());
    println!("  字符数量: {}", analyzer.char_count());
    
    if let Some(longest_word) = analyzer.longest_word() {
        println!("  最长单词: {}", longest_word);
    }
    
    let words = analyzer.words();
    println!("  前5个单词: {:?}", &words[..5.min(words.len())]);
    
    // 配置管理器
    let config_text = "database_url=localhost:5432\napi_key=secret123\ndebug=true";
    let config = Config::parse(config_text);
    
    println!("配置信息:");
    if let Some(db_url) = config.get("database_url") {
        println!("  数据库URL: {}", db_url);
    }
    if let Some(api_key) = config.get("api_key") {
        println!("  API密钥: {}", api_key);
    }
    
    // ========== 生命周期边界和约束 ==========
    println!("\n--- 生命周期边界和约束 ---");
    
    let data = "important data";
    let container = Container::new(data);
    let reference = container.get_reference();
    println!("容器引用: {}", reference);
    
    // 使用where子句的生命周期约束
    let result = function_with_lifetime_bounds(&data, &"other data");
    println!("生命周期约束函数: {}", result);
    
    println!("\n=== 生命周期要点总结 ===");
    println!("1. 生命周期确保引用的有效性");
    println!("2. 大多数情况下生命周期是隐式的（省略规则）");
    println!("3. 复杂情况需要显式生命周期注解");
    println!("4. 结构体持有引用需要生命周期参数");
    println!("5. 静态生命周期存在于整个程序运行期间");
    println!("6. 生命周期参数不改变引用的实际生命周期");
    println!("7. 生命周期是 Rust 内存安全的重要保证");
}

// ========== 生命周期函数示例 ==========

// 基本生命周期函数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期省略规则示例
fn longest_elided(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 不同生命周期参数
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 返回固定引用（总是返回第一个参数）
fn choose_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// 静态生命周期函数
fn get_static_str() -> &'static str {
    "这是一个静态字符串"
}

// 复杂生命周期函数
fn complex_function<'a, 'b>(x: &'a i32, y: &'a i32, z: &'b i32) -> &'a i32
where
    'b: 'a, // 'b 必须比 'a 活得更久
{
    if x > y && x > z {
        x
    } else {
        y
    }
}

// 带where子句的生命周期约束
fn function_with_lifetime_bounds<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ========== 带生命周期的结构体 ==========

// 基本生命周期结构体
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    // 方法中的生命周期省略
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }
    
    // 显式生命周期注解
    fn get_part(&self) -> &'a str {
        self.part
    }
}

// 字符串包装器
struct StringWrapper<'a> {
    content: &'a str,
}

impl<'a> StringWrapper<'a> {
    fn new(content: &'a str) -> Self {
        StringWrapper { content }
    }
    
    fn first_word(&self) -> Option<&'a str> {
        self.content.split_whitespace().next()
    }
    
    fn get_prefix(&self, len: usize) -> &'a str {
        if len >= self.content.len() {
            self.content
        } else {
            &self.content[..len]
        }
    }
    
    fn len(&self) -> usize {
        self.content.len()
    }
}

// 持有泛型引用的结构体
struct Holder<'a> {
    value: &'a str,
}

impl<'a> Holder<'a> {
    fn new(value: &'a str) -> Self {
        Holder { value }
    }
}

// 泛型与生命周期结合
struct GenericHolder<'a, T> {
    value: &'a T,
}

impl<'a, T> GenericHolder<'a, T> {
    fn new(value: &'a T) -> Self {
        GenericHolder { value }
    }
    
    fn get(&self) -> &'a T {
        self.value
    }
}

// 数据处理器
struct DataProcessor<'a> {
    data: &'a [i32],
}

impl<'a> DataProcessor<'a> {
    fn new(data: &'a [i32]) -> Self {
        DataProcessor { data }
    }
    
    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }
    
    fn max(&self) -> Option<&'a i32> {
        self.data.iter().max()
    }
    
    fn min(&self) -> Option<&'a i32> {
        self.data.iter().min()
    }
}

// ========== 实际应用结构体 ==========

// 文本分析器
struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }
    
    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
    
    fn char_count(&self) -> usize {
        self.text.chars().count()
    }
    
    fn longest_word(&self) -> Option<&'a str> {
        self.text
            .split_whitespace()
            .max_by_key(|word| word.len())
    }
    
    fn words(&self) -> Vec<&'a str> {
        self.text.split_whitespace().collect()
    }
    
    fn find_word(&self, target: &str) -> Option<&'a str> {
        self.text
            .split_whitespace()
            .find(|&word| word == target)
    }
}

// 配置管理器
struct Config<'a> {
    content: &'a str,
}

impl<'a> Config<'a> {
    fn parse(content: &'a str) -> Self {
        Config { content }
    }
    
    fn get(&self, key: &str) -> Option<&'a str> {
        for line in self.content.lines() {
            if let Some(eq_pos) = line.find('=') {
                let (k, v) = line.split_at(eq_pos);
                if k == key {
                    return Some(&v[1..]); // 跳过 '=' 字符
                }
            }
        }
        None
    }
    
    fn keys(&self) -> Vec<&'a str> {
        self.content
            .lines()
            .filter_map(|line| {
                line.find('=').map(|pos| &line[..pos])
            })
            .collect()
    }
}

// 容器示例
struct Container<'a, T> {
    value: &'a T,
}

impl<'a, T> Container<'a, T> {
    fn new(value: &'a T) -> Self {
        Container { value }
    }
    
    fn get_reference(&self) -> &'a T {
        self.value
    }
}

impl<'a, T: Display> Container<'a, T> {
    fn display(&self) {
        println!("容器值: {}", self.value);
    }
}

// ========== 高级生命周期示例 ==========

// 生命周期子类型
struct Context<'a> {
    data: &'a str,
}

impl<'a> Context<'a> {
    fn new(data: &'a str) -> Self {
        Context { data }
    }
    
    fn parse<'b>(&'b self) -> Parser<'a, 'b> 
    where
        'a: 'b, // 'a 必须比 'b 活得更久
    {
        Parser {
            context: self,
            position: 0,
        }
    }
}

struct Parser<'a, 'b> {
    context: &'b Context<'a>,
    position: usize,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn next_token(&mut self) -> Option<&'a str> {
        if self.position < self.context.data.len() {
            let start = self.position;
            self.position += 1;
            Some(&self.context.data[start..self.position])
        } else {
            None
        }
    }
}

// HRTB (Higher-Ranked Trait Bounds) 示例
fn apply_to_all<F>(items: &[&str], f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> String,
{
    items.iter().map(|&item| f(item)).collect()
}

// 生命周期与闭包
fn create_closure<'a>(text: &'a str) -> impl Fn() -> &'a str {
    move || text
}
