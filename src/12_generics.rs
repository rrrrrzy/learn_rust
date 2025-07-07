// 12_generics.rs
// Rust 泛型详解

use std::fmt::Display;
use std::cmp::PartialOrd;

// ========== 泛型结构体 ==========

// 基本泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn new(width: T, height: U) -> Self {
        Rectangle { width, height }
    }
    
    fn dimensions(&self) -> (&T, &U) {
        (&self.width, &self.height)
    }
    
    // 泛型方法可以有不同的泛型参数
    fn mix_point<V, W>(self, other: Point<V>) -> Rectangle<T, W> 
    where
        W: Default,
    {
        Rectangle {
            width: self.width,
            height: W::default(),
        }
    }
}

// 约束泛型的结构体
#[derive(Debug)]
struct Pair<T: Display + PartialOrd> {
    first: T,
    second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
    
    fn larger(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
    
    fn display_comparison(&self) {
        if self.first >= self.second {
            println!("{} >= {}", self.first, self.second);
        } else {
            println!("{} < {}", self.first, self.second);
        }
    }
}

// ========== 泛型枚举 ==========

// 自定义Option类型
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn is_some(&self) -> bool {
        match self {
            MyOption::Some(_) => true,
            MyOption::None => false,
        }
    }
    
    fn unwrap(self) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("在None上调用unwrap"),
        }
    }
    
    fn map<U, F>(self, f: F) -> MyOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyOption::Some(value) => MyOption::Some(f(value)),
            MyOption::None => MyOption::None,
        }
    }
}

// 自定义Result类型
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MyResult<T, E> {
    fn is_ok(&self) -> bool {
        match self {
            MyResult::Ok(_) => true,
            MyResult::Err(_) => false,
        }
    }
    
    fn map<U, F>(self, f: F) -> MyResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyResult::Ok(value) => MyResult::Ok(f(value)),
            MyResult::Err(error) => MyResult::Err(error),
        }
    }
    
    fn and_then<U, F>(self, f: F) -> MyResult<U, E>
    where
        F: FnOnce(T) -> MyResult<U, E>,
    {
        match self {
            MyResult::Ok(value) => f(value),
            MyResult::Err(error) => MyResult::Err(error),
        }
    }
}

// ========== 泛型容器 ==========

#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// 为特定约束实现额外方法
impl<T: Clone> Container<T> {
    fn duplicate_all(&mut self) {
        let cloned_items = self.items.clone();
        self.items.extend(cloned_items);
    }
    
    fn get_cloned(&self, index: usize) -> Option<T> {
        self.items.get(index).cloned()
    }
}

impl<T: Display> Container<T> {
    fn print_all(&self) {
        for (i, item) in self.items.iter().enumerate() {
            println!("  [{}]: {}", i, item);
        }
    }
}

fn main() {
    println!("=== Rust 泛型详解 ===\n");
    
    // ========== 泛型函数 ==========
    println!("--- 泛型函数 ---");
    
    // 基本泛型函数
    let int_vec = vec![1, 2, 3, 4, 5];
    let str_vec = vec!["a", "b", "c"];
    
    println!("整数向量最大值: {:?}", largest(&int_vec));
    println!("字符串向量最大值: {:?}", largest(&str_vec));
    
    // 多参数泛型函数
    let point1 = Point::new(1, 2);
    let point2 = Point::new(3, 4);
    
    let swapped = swap_coordinates(point1, point2);
    println!("坐标交换结果: {:?}", swapped);
    
    // 约束泛型函数
    let numbers = vec![1, 5, 3, 9, 2, 8];
    let strings = vec!["hello", "world", "rust"];
    
    print_sorted(&numbers);
    print_sorted(&strings);
    
    // ========== 泛型结构体使用 ==========
    println!("\n--- 泛型结构体使用 ---");
    
    // 基本Point使用
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    let string_point = Point::new("hello", "world");
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    println!("字符串点: {:?}", string_point);
    
    // 使用特定实现的方法
    println!("浮点数点到原点距离: {:.2}", float_point.distance_from_origin());
    
    // 多参数泛型结构体
    let rect = Rectangle::new(10, 20.5);
    println!("矩形: {:?}", rect);
    println!("矩形尺寸: {:?}", rect.dimensions());
    
    // 约束泛型结构体
    let pair = Pair::new(10, 20);
    println!("数字对: {:?}", pair);
    println!("较大的数: {}", pair.larger());
    pair.display_comparison();
    
    let str_pair = Pair::new("hello", "world");
    println!("字符串对: {:?}", str_pair);
    println!("较大的字符串: {}", str_pair.larger());
    str_pair.display_comparison();
    
    // ========== 泛型枚举使用 ==========
    println!("\n--- 泛型枚举使用 ---");
    
    let some_number = MyOption::Some(42);
    let none_number: MyOption<i32> = MyOption::None;
    
    println!("some_number是否有值: {}", some_number.is_some());
    println!("none_number是否有值: {}", none_number.is_some());
    
    // 使用map方法
    let doubled = some_number.map(|x| x * 2);
    println!("映射结果: {:?}", doubled);
    
    // MyResult使用
    let ok_result = MyResult::Ok(100);
    let err_result: MyResult<i32, &str> = MyResult::Err("错误信息");
    
    println!("ok_result是否成功: {}", ok_result.is_ok());
    println!("err_result是否成功: {}", err_result.is_ok());
    
    let mapped_result = ok_result.map(|x| x * 2);
    println!("Result映射: {:?}", mapped_result);
    
    // ========== 泛型容器使用 ==========
    println!("\n--- 泛型容器使用 ---");
    
    let mut int_container = Container::new();
    int_container.add(1);
    int_container.add(2);
    int_container.add(3);
    
    println!("整数容器: {:?}", int_container);
    println!("容器长度: {}", int_container.len());
    
    // 使用Display约束的方法
    println!("容器内容:");
    int_container.print_all();
    
    // 使用Clone约束的方法
    int_container.duplicate_all();
    println!("复制后的容器:");
    int_container.print_all();
    
    // 字符串容器
    let mut str_container = Container::new();
    str_container.add("hello");
    str_container.add("world");
    str_container.add("rust");
    
    println!("字符串容器内容:");
    str_container.print_all();
    
    // ========== 高级泛型示例 ==========
    println!("\n--- 高级泛型示例 ---");
    
    // 泛型栈
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈: {:?}", stack);
    
    while let Some(item) = stack.pop() {
        println!("弹出: {}", item);
    }
    
    // 泛型映射
    let numbers = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = map_vec(numbers, |x| x * x);
    println!("平方映射: {:?}", squared);
    
    let strings = vec!["hello", "world"];
    let lengths: Vec<usize> = map_vec(strings, |s| s.len());
    println!("长度映射: {:?}", lengths);
    
    // 泛型过滤
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = filter_vec(numbers, |&x| x % 2 == 0);
    println!("偶数过滤: {:?}", evens);
    
    // ========== 泛型约束示例 ==========
    println!("\n--- 泛型约束示例 ---");
    
    // 可比较的类型
    let nums = vec![5, 2, 8, 1, 9];
    if let Some(max) = find_max(&nums) {
        println!("最大值: {}", max);
    }
    
    // 可显示的类型
    let items = vec![1, 2, 3];
    print_items(&items);
    
    // 可克隆的类型
    let original = vec![1, 2, 3];
    let cloned = clone_and_modify(original, |x| x * 2);
    println!("克隆并修改: {:?}", cloned);
    
    // ========== 生命周期与泛型 ==========
    println!("\n--- 生命周期与泛型 ---");
    
    let string1 = String::from("hello");
    let string2 = String::from("world");
    
    let result = longest_generic(&string1, &string2);
    println!("最长的字符串: {}", result);
    
    // 泛型结构体与生命周期
    let important_excerpt = ImportantExcerpt::new("这是一个重要的摘录");
    println!("摘录: {}", important_excerpt.part);
    
    println!("\n=== 泛型要点总结 ===");
    println!("1. 泛型允许编写适用于多种类型的代码");
    println!("2. 使用<T>语法定义泛型参数");
    println!("3. 泛型约束使用where子句或trait bounds");
    println!("4. 单态化使泛型代码没有运行时开销");
    println!("5. 可以在结构体、枚举、函数和方法中使用泛型");
    println!("6. 泛型与生命周期参数可以结合使用");
}

// ========== 泛型函数 ==========

// 基本泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

// 多参数泛型函数
fn swap_coordinates<T>(p1: Point<T>, p2: Point<T>) -> (Point<T>, Point<T>) {
    (p2, p1)
}

// 约束泛型函数
fn print_sorted<T: Display + PartialOrd + Clone>(list: &[T]) {
    let mut sorted = list.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    print!("排序结果: ");
    for (i, item) in sorted.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", item);
    }
    println!();
}

// ========== 泛型数据结构 ==========

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// 泛型映射函数
fn map_vec<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    vec.into_iter().map(f).collect()
}

// 泛型过滤函数
fn filter_vec<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    vec.into_iter().filter(predicate).collect()
}

// 约束泛型函数
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().max()
}

fn print_items<T: Display>(items: &[T]) {
    print!("项目: ");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", item);
    }
    println!();
}

fn clone_and_modify<T, F>(items: Vec<T>, f: F) -> Vec<T>
where
    T: Clone,
    F: Fn(T) -> T,
{
    items.into_iter().map(f).collect()
}

// 生命周期与泛型结合
fn longest_generic<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: PartialOrd,
{
    if x >= y {
        x
    } else {
        y
    }
}

// 带生命周期的泛型结构体
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(text: &'a str) -> Self {
        ImportantExcerpt { part: text }
    }
    
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }
}

// 复杂泛型示例：泛型迭代器
struct Counter<T> {
    current: T,
    max: T,
    step: T,
}

impl<T> Counter<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    fn new(start: T, max: T, step: T) -> Self {
        Counter {
            current: start,
            max,
            step,
        }
    }
}

impl<T> Iterator for Counter<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T>,
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current = self.current + self.step;
            Some(current)
        } else {
            None
        }
    }
}
