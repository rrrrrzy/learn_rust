// 13_traits_and_trait_bounds.rs
// Rust Trait 与 Trait Bound 详解

use std::fmt::{Display, Debug};
use std::ops::Add;

// ========== 基本 Trait 定义 ==========

// 基本 trait 定义
trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn author(&self) -> String {
        String::from("Unknown")
    }
    
    // 调用其他方法的默认实现
    fn full_summary(&self) -> String {
        format!("(由 {} 撰写) {}", self.author(), self.summarize())
    }
}

// 新闻文章结构体
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} ({})", self.headline, self.author, self.location)
    }
    
    fn author(&self) -> String {
        self.author.clone()
    }
}

// 推文结构体
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 只使用默认实现的结构体
struct BlogPost {
    pub title: String,
    pub content: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("博客文章: {}", self.title)
    }
}

// ========== 高级 Trait 示例 ==========

// 可绘制的 trait
trait Drawable {
    fn draw(&self);
    
    // 默认边界框实现
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, 100.0, 100.0)
    }
}

// 可计算面积的 trait
trait Area {
    fn area(&self) -> f64;
}

// 图形结构体
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制圆形，半径: {}", self.radius);
    }
    
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        let d = self.radius * 2.0;
        (-self.radius, -self.radius, d, d)
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
    }
    
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, self.width, self.height)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Triangle {
    fn draw(&self) {
        println!("绘制三角形，底: {}, 高: {}", self.base, self.height);
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// ========== 操作符重载 trait ==========

// 自定义点结构体
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

// 为 Point 实现 Add trait
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 为 Point 实现 Display trait
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 为 Point 实现 PartialEq trait
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON
    }
}

// ========== 泛型 trait ==========

// 泛型 trait 定义
trait Container<T> {
    fn get(&self, index: usize) -> Option<&T>;
    fn add(&mut self, item: T);
    fn len(&self) -> usize;
    
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// 实现泛型 trait
struct MyVec<T> {
    items: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { items: Vec::new() }
    }
}

impl<T> Container<T> for MyVec<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// ========== 关联类型 trait ==========

trait Iterator {
    type Item; // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // 默认实现，使用关联类型
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_iter(self)
    }
}

trait FromIterator<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}

// 自定义迭代器
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// ========== 约束和边界 ==========

// 多个 trait 约束
fn print_and_compare<T: Display + PartialOrd>(a: &T, b: &T) {
    println!("比较 {} 和 {}", a, b);
    if a > b {
        println!("第一个更大");
    } else if a < b {
        println!("第二个更大");
    } else {
        println!("两个相等");
    }
}

// where 子句
fn complex_function<T, U>(a: &T, b: &U) -> String
where
    T: Display + Clone,
    U: Display + Debug,
{
    format!("a: {}, b: {} (调试: {:?})", a, b, b)
}

// 返回 trait 对象
fn create_summary(content_type: &str) -> Box<dyn Summary> {
    match content_type {
        "news" => Box::new(NewsArticle {
            headline: "重要新闻".to_string(),
            location: "北京".to_string(),
            author: "记者".to_string(),
            content: "新闻内容".to_string(),
        }),
        "tweet" => Box::new(Tweet {
            username: "用户".to_string(),
            content: "这是一条推文".to_string(),
            reply: false,
            retweet: false,
        }),
        _ => Box::new(BlogPost {
            title: "默认博客".to_string(),
            content: "博客内容".to_string(),
        }),
    }
}

// ========== 条件实现 ==========

// 为满足特定条件的类型实现 trait
struct Wrapper<T>(T);

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wrapper({})", self.0)
    }
}

// 只有当 T 实现了 Display 和 PartialOrd 时才实现这个方法
impl<T: Display + PartialOrd> Wrapper<T> {
    fn compare_and_print(&self, other: &Wrapper<T>) {
        if self.0 > other.0 {
            println!("{} > {}", self.0, other.0);
        } else {
            println!("{} <= {}", self.0, other.0);
        }
    }
}

// ========== 超 trait ==========

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("在飞机中飞行");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("使用魔法飞行");
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "人类")
    }
}

impl OutlinePrint for Human {}

fn main() {
    println!("=== Rust Trait 与 Trait Bound 详解 ===\n");
    
    // ========== 基本 trait 使用 ==========
    println!("--- 基本 trait 使用 ---");
    
    let news = NewsArticle {
        headline: "Rust 1.70 发布".to_string(),
        location: "全球".to_string(),
        author: "Rust团队".to_string(),
        content: "Rust 1.70 带来了许多新特性...".to_string(),
    };
    
    let tweet = Tweet {
        username: "rust_lang".to_string(),
        content: "Rust 是最好的编程语言！".to_string(),
        reply: false,
        retweet: false,
    };
    
    let blog = BlogPost {
        title: "学习 Rust Trait".to_string(),
        content: "Trait 是 Rust 中定义共享行为的方式...".to_string(),
    };
    
    println!("新闻摘要: {}", news.summarize());
    println!("推文摘要: {}", tweet.summarize());
    println!("博客摘要: {}", blog.summarize());
    
    println!("新闻完整摘要: {}", news.full_summary());
    println!("推文完整摘要: {}", tweet.full_summary());
    println!("博客完整摘要: {}", blog.full_summary());
    
    // ========== trait 作为参数 ==========
    println!("\n--- trait 作为参数 ---");
    
    notify(&news);
    notify(&tweet);
    notify(&blog);
    
    // 使用 trait bound 语法
    notify_verbose(&news);
    
    // ========== 图形绘制示例 ==========
    println!("\n--- 图形绘制示例 ---");
    
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 8.0 };
    let triangle = Triangle { base: 6.0, height: 4.0 };
    
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle, &triangle];
    
    for shape in shapes {
        shape.draw();
        let (x, y, w, h) = shape.bounding_box();
        println!("  边界框: ({}, {}, {}, {})", x, y, w, h);
    }
    
    // 计算面积
    println!("\n面积计算:");
    println!("圆形面积: {:.2}", circle.area());
    println!("矩形面积: {:.2}", rectangle.area());
    println!("三角形面积: {:.2}", triangle.area());
    
    // ========== 操作符重载 ==========
    println!("\n--- 操作符重载 ---");
    
    let point1 = Point::new(1.0, 2.0);
    let point2 = Point::new(3.0, 4.0);
    let point3 = point1 + point2;
    
    println!("点1: {}", point1);
    println!("点2: {}", point2);
    println!("点1 + 点2 = {}", point3);
    println!("点1 == 点2: {}", point1 == point2);
    
    // ========== 泛型 trait 使用 ==========
    println!("\n--- 泛型 trait 使用 ---");
    
    let mut container = MyVec::new();
    container.add(1);
    container.add(2);
    container.add(3);
    
    println!("容器长度: {}", container.len());
    println!("容器是否为空: {}", container.is_empty());
    
    for i in 0..container.len() {
        if let Some(item) = container.get(i) {
            println!("索引 {}: {}", i, item);
        }
    }
    
    // ========== 迭代器使用 ==========
    println!("\n--- 迭代器使用 ---");
    
    let mut counter = Counter::new(5);
    println!("计数器:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }
    
    // ========== trait 约束 ==========
    println!("\n--- trait 约束 ---");
    
    print_and_compare(&10, &20);
    print_and_compare(&"hello", &"world");
    
    let result = complex_function(&42, &"test");
    println!("复杂函数结果: {}", result);
    
    // ========== trait 对象 ==========
    println!("\n--- trait 对象 ---");
    
    let summaries: Vec<Box<dyn Summary>> = vec![
        create_summary("news"),
        create_summary("tweet"),
        create_summary("blog"),
    ];
    
    for summary in summaries {
        println!("动态摘要: {}", summary.summarize());
    }
    
    // ========== 条件实现 ==========
    println!("\n--- 条件实现 ---");
    
    let wrapper1 = Wrapper(42);
    let wrapper2 = Wrapper(10);
    
    println!("包装器1: {}", wrapper1);
    wrapper1.compare_and_print(&wrapper2);
    
    // ========== 超 trait 和方法歧义 ==========
    println!("\n--- 超 trait 和方法歧义 ---");
    
    let person = Human;
    
    // 调用不同 trait 的同名方法
    person.fly(); // 默认调用
    Pilot::fly(&person); // 显式调用
    Wizard::fly(&person); // 显式调用
    
    // 使用超 trait
    person.outline_print();
    
    // ========== 高级 trait 示例 ==========
    println!("\n--- 高级 trait 示例 ---");
    
    demonstrate_advanced_traits();
    
    // ========== 实际应用示例 ==========
    println!("\n--- 实际应用示例 ---");
    
    let mut storage = FileStorage::new();
    storage.save("config.txt", "配置内容");
    
    if let Some(content) = storage.load("config.txt") {
        println!("加载的内容: {}", content);
    }
    
    // 序列化示例
    let data = UserData {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let json = data.to_json();
    println!("序列化结果: {}", json);
    
    println!("\n=== Trait 要点总结 ===");
    println!("1. Trait 定义共享行为，类似于其他语言的接口");
    println!("2. 可以为任何类型实现 trait，包括外部类型");
    println!("3. Trait 可以有默认实现");
    println!("4. Trait bound 限制泛型类型必须实现特定 trait");
    println!("5. Trait 对象允许动态分发");
    println!("6. 可以使用 trait 进行操作符重载");
    println!("7. 超 trait 要求实现者同时实现多个 trait");
}

// ========== 辅助函数 ==========

// trait 作为参数的不同语法
fn notify(item: &impl Summary) {
    println!("通知: {}", item.summarize());
}

fn notify_verbose<T: Summary>(item: &T) {
    println!("详细通知: {}", item.full_summary());
}

// 多个 trait bound
fn notify_and_display<T: Summary + Display>(item: &T) {
    println!("显示并通知: {}", item);
    println!("摘要: {}", item.summarize());
}

// 返回实现了 trait 的类型
fn return_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: "条件新闻".to_string(),
            location: "某地".to_string(),
            author: "某人".to_string(),
            content: "内容".to_string(),
        }
    } else {
        NewsArticle {
            headline: "其他新闻".to_string(),
            location: "其他地方".to_string(),
            author: "其他人".to_string(),
            content: "其他内容".to_string(),
        }
    }
}

// 高级 trait 演示
fn demonstrate_advanced_traits() {
    // 使用 trait bound 的泛型函数
    fn compare_and_print<T>(a: &T, b: &T)
    where
        T: PartialOrd + Display,
    {
        if a > b {
            println!("{} > {}", a, b);
        } else {
            println!("{} <= {}", a, b);
        }
    }
    
    compare_and_print(&10, &5);
    compare_and_print(&"hello", &"world");
}

// 实际应用：存储 trait
trait Storage {
    fn save(&mut self, key: &str, value: &str);
    fn load(&self, key: &str) -> Option<String>;
    fn delete(&mut self, key: &str) -> bool;
}

struct FileStorage {
    data: std::collections::HashMap<String, String>,
}

impl FileStorage {
    fn new() -> Self {
        FileStorage {
            data: std::collections::HashMap::new(),
        }
    }
}

impl Storage for FileStorage {
    fn save(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        println!("保存到文件: {} = {}", key, value);
    }
    
    fn load(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
    
    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

// 序列化 trait
trait Serialize {
    fn to_json(&self) -> String;
}

#[derive(Debug)]
struct UserData {
    name: String,
    age: u32,
}

impl Serialize for UserData {
    fn to_json(&self) -> String {
        format!(r#"{{"name": "{}", "age": {}}}"#, self.name, self.age)
    }
}

// 自定义 Clone trait
trait MyClone {
    fn clone(&self) -> Self;
}

impl MyClone for i32 {
    fn clone(&self) -> Self {
        *self
    }
}

impl MyClone for String {
    fn clone(&self) -> Self {
        self.clone()
    }
}
