// 05_ownership_and_borrowing.rs
// Rust 所有权与借用系统详解

fn main() {
    println!("=== Rust 所有权与借用系统详解 ===\n");
    
    // ========== 所有权基础 ==========
    println!("--- 所有权基础 ---");
    
    // 栈上的数据（实现了Copy trait）
    let x = 5;
    let y = x; // 复制，x仍然有效
    println!("栈数据复制: x = {}, y = {}", x, y);
    
    // 堆上的数据（String类型）
    let s1 = String::from("hello");
    let s2 = s1; // 移动（move），s1不再有效
    // println!("{}", s1); // 编译错误！s1已经被移动
    println!("堆数据移动: s2 = {}", s2);
    
    // 克隆堆数据
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝，s3仍然有效
    println!("克隆数据: s3 = {}, s4 = {}", s3, s4);
    
    // ========== 所有权与函数 ==========
    println!("\n--- 所有权与函数 ---");
    
    let s = String::from("function");
    takes_ownership(s); // s被移动到函数中
    // println!("{}", s); // 编译错误！s已经被移动
    
    let x = 5;
    makes_copy(x); // x被复制到函数中
    println!("复制后x仍然有效: {}", x); // x仍然有效
    
    // 函数返回值也会转移所有权
    let s1 = gives_ownership();
    println!("获得所有权: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2被移动，返回新的所有权
    println!("转移所有权: {}", s3);
    
    // ========== 引用与借用 ==========
    println!("\n--- 引用与借用 ---");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递引用，不转移所有权
    println!("字符串 '{}' 的长度是 {}", s1, len); // s1仍然有效
    
    // 可变引用
    let mut s = String::from("hello");
    change(&mut s); // 传递可变引用
    println!("修改后: {}", s);
    
    // ========== 借用规则 ==========
    println!("\n--- 借用规则 ---");
    
    let mut s = String::from("hello");
    
    // 规则1：可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("多个不可变引用: {} 和 {}", r1, r2);
    
    // 规则2：不可变引用的作用域结束后，可以创建可变引用
    let r3 = &mut s;
    println!("可变引用: {}", r3);
    
    // 规则3：不能同时拥有可变和不可变引用
    let s = String::from("hello");
    let r1 = &s; // 不可变引用
    let r2 = &s; // 不可变引用
    println!("不可变引用: {}, {}", r1, r2);
    // r1 和 r2 的作用域在此结束
    
    let r3 = &mut s; // 可变引用
    println!("可变引用: {}", r3);
    
    // ========== 悬垂引用 ==========
    println!("\n--- 悬垂引用防护 ---");
    
    // 以下代码会编译错误，因为会创建悬垂引用
    // let reference_to_nothing = dangle(); // 编译错误！
    
    // 正确的做法：返回所有权而不是引用
    let string = no_dangle();
    println!("正确返回所有权: {}", string);
    
    // ========== 切片 ==========
    println!("\n--- 切片 ---");
    
    let s = String::from("hello world");
    let hello = &s[0..5]; // 字符串切片
    let world = &s[6..11];
    println!("切片: '{}' 和 '{}'", hello, world);
    
    // 切片语法糖
    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];   // 等价于 &s[0..2]
    let slice3 = &s[3..];   // 等价于 &s[3..s.len()]
    let slice4 = &s[..];    // 等价于 &s[0..s.len()]
    
    println!("切片语法: '{}', '{}', '{}', '{}'", slice1, slice2, slice3, slice4);
    
    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片: {:?}", slice);
    
    // ========== 实用示例 ==========
    println!("\n--- 实用示例 ---");
    
    // 查找第一个单词
    let sentence = String::from("hello world rust");
    let word = first_word(&sentence);
    println!("第一个单词: '{}'", word);
    
    // 字符串字面量就是切片
    let s: &str = "Hello, world!";
    let word = first_word(s);
    println!("字面量第一个单词: '{}'", word);
    
    // 所有权转移示例
    let data = vec![1, 2, 3, 4, 5];
    let processed = process_data(data);
    // println!("{:?}", data); // 编译错误！data已被移动
    println!("处理后的数据: {:?}", processed);
    
    // 借用示例
    let data = vec![1, 2, 3, 4, 5];
    let sum = calculate_sum(&data);
    println!("数据: {:?}, 和: {}", data, sum); // data仍然有效
    
    // 可变借用示例
    let mut numbers = vec![1, 2, 3];
    double_values(&mut numbers);
    println!("翻倍后: {:?}", numbers);
    
    // ========== 所有权模式 ==========
    println!("\n--- 所有权模式 ---");
    
    // 1. 通过参数传递所有权
    let s = String::from("hello");
    let len = length_taking_ownership(s);
    println!("长度: {}", len);
    
    // 2. 通过借用避免所有权转移
    let s = String::from("hello");
    let len = length_borrowing(&s);
    println!("字符串: {}, 长度: {}", s, len);
    
    // 3. 返回所有权
    let s = create_string();
    println!("创建的字符串: {}", s);
    
    // 4. 转移并返回所有权
    let s1 = String::from("hello");
    let s2 = append_world(s1);
    println!("追加后: {}", s2);
    
    println!("\n=== 所有权与借用要点总结 ===");
    println!("1. 每个值都有一个所有者，所有者离开作用域时值被释放");
    println!("2. 值可以被移动（move）或复制（copy）");
    println!("3. 引用允许使用值而不获取所有权");
    println!("4. 可变引用在同一时间只能有一个");
    println!("5. 不能同时拥有可变和不可变引用");
    println!("6. 切片是对连续序列的引用");
}

// ========== 函数定义 ==========

// 获取所有权
fn takes_ownership(some_string: String) {
    println!("接收所有权: {}", some_string);
} // some_string在此处被释放

// 复制值
fn makes_copy(some_integer: i32) {
    println!("复制值: {}", some_integer);
} // some_integer在此处离开作用域，但因为是复制，没有特殊处理

// 返回所有权
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回所有权
}

// 接收并返回所有权
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回所有权
}

// 计算长度（借用）
fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开作用域，但因为没有所有权，不会释放

// 修改借用的值
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 危险的悬垂引用（编译错误）
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回s的引用，但s即将被销毁
// }

// 正确的做法：返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回所有权
}

// 查找第一个单词
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 所有权转移示例
fn process_data(mut data: Vec<i32>) -> Vec<i32> {
    data.push(6);
    data
}

// 借用示例
fn calculate_sum(data: &Vec<i32>) -> i32 {
    data.iter().sum()
}

// 可变借用示例
fn double_values(data: &mut Vec<i32>) {
    for value in data.iter_mut() {
        *value *= 2;
    }
}

// 所有权模式示例
fn length_taking_ownership(s: String) -> usize {
    s.len()
}

fn length_borrowing(s: &String) -> usize {
    s.len()
}

fn create_string() -> String {
    String::from("created")
}

fn append_world(mut s: String) -> String {
    s.push_str(" world");
    s
}
