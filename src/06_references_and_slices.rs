// 06_references_and_slices.rs
// Rust 引用与切片详解

fn main() {
    println!("=== Rust 引用与切片详解 ===\n");
    
    // ========== 引用基础 ==========
    println!("--- 引用基础 ---");
    
    let s1 = String::from("hello");
    let s1_ref = &s1; // 创建s1的引用
    
    println!("原始字符串: {}", s1);
    println!("引用: {}", s1_ref);
    println!("引用指向的值: {}", *s1_ref); // 解引用
    
    // 引用的引用
    let s1_ref_ref = &s1_ref;
    println!("引用的引用: {}", s1_ref_ref);
    println!("双重解引用: {}", **s1_ref_ref);
    
    // ========== 可变引用 ==========
    println!("\n--- 可变引用 ---");
    
    let mut s = String::from("hello");
    println!("原始字符串: {}", s);
    
    {
        let s_ref = &mut s; // 可变引用
        s_ref.push_str(", world");
        println!("通过可变引用修改: {}", s_ref);
    } // s_ref的作用域结束
    
    println!("修改后的字符串: {}", s);
    
    // ========== 引用规则演示 ==========
    println!("\n--- 引用规则演示 ---");
    
    let mut data = String::from("initial");
    
    // 可以有多个不可变引用
    let r1 = &data;
    let r2 = &data;
    let r3 = &data;
    println!("多个不可变引用: {}, {}, {}", r1, r2, r3);
    // 不可变引用的作用域在此结束
    
    // 现在可以创建可变引用
    let r4 = &mut data;
    r4.push_str(" modified");
    println!("可变引用: {}", r4);
    // 可变引用的作用域在此结束
    
    // 再次创建不可变引用
    let r5 = &data;
    println!("新的不可变引用: {}", r5);
    
    // ========== 字符串切片 ==========
    println!("\n--- 字符串切片 ---");
    
    let s = String::from("hello world rust programming");
    
    // 基本切片操作
    let hello = &s[0..5];
    let world = &s[6..11];
    let rust = &s[12..16];
    
    println!("原字符串: {}", s);
    println!("切片: '{}', '{}', '{}'", hello, world, rust);
    
    // 切片语法变体
    let s = String::from("hello");
    
    let slice1 = &s[0..2];    // 明确指定开始和结束
    let slice2 = &s[..2];     // 从开始到索引2
    let slice3 = &s[3..];     // 从索引3到结束
    let slice4 = &s[..];      // 整个字符串
    
    println!("切片变体: '{}', '{}', '{}', '{}'", slice1, slice2, slice3, slice4);
    
    // 中文字符串切片（按字节，需要小心）
    let chinese = String::from("你好世界");
    let slice = &chinese[..6]; // 每个中文字符占3个字节
    println!("中文切片: '{}'", slice);
    
    // 安全的字符串切片
    let s = String::from("hello 世界");
    if let Some(end) = s.char_indices().nth(7).map(|(i, _)| i) {
        let safe_slice = &s[..end];
        println!("安全切片: '{}'", safe_slice);
    }
    
    // ========== 数组切片 ==========
    println!("\n--- 数组切片 ---");
    
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let slice1 = &array[2..5];    // [3, 4, 5]
    let slice2 = &array[..3];     // [1, 2, 3]
    let slice3 = &array[7..];     // [8, 9, 10]
    let slice4 = &array[..];      // 整个数组
    
    println!("原数组: {:?}", array);
    println!("数组切片:");
    println!("  [2..5]: {:?}", slice1);
    println!("  [..3]:  {:?}", slice2);
    println!("  [7..]:  {:?}", slice3);
    println!("  [..]:   {:?}", slice4);
    
    // 可变数组切片
    let mut mut_array = [1, 2, 3, 4, 5];
    {
        let slice = &mut mut_array[1..4];
        slice[0] = 10;
        slice[2] = 30;
        println!("修改可变切片: {:?}", slice);
    }
    println!("修改后的数组: {:?}", mut_array);
    
    // ========== 向量切片 ==========
    println!("\n--- 向量切片 ---");
    
    let vec = vec![10, 20, 30, 40, 50];
    let vec_slice = &vec[1..4];
    
    println!("向量: {:?}", vec);
    println!("向量切片: {:?}", vec_slice);
    
    // 向量的可变切片
    let mut mut_vec = vec![1, 2, 3, 4, 5];
    {
        let slice = &mut mut_vec[2..];
        for item in slice.iter_mut() {
            *item *= 2;
        }
        println!("修改可变向量切片: {:?}", slice);
    }
    println!("修改后的向量: {:?}", mut_vec);
    
    // ========== 切片作为函数参数 ==========
    println!("\n--- 切片作为函数参数 ---");
    
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("第一个单词: '{}'", first);
    
    // 字符串字面量也是切片
    let literal = "hello rust";
    let first = first_word(literal);
    println!("字面量第一个单词: '{}'", first);
    
    // 数组切片函数
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = sum_slice(&numbers[2..8]);
    println!("切片 [2..8] 的和: {}", sum);
    
    let max = find_max(&numbers);
    match max {
        Some(value) => println!("最大值: {}", value),
        None => println!("空切片"),
    }
    
    // ========== 高级切片操作 ==========
    println!("\n--- 高级切片操作 ---");
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    // 单词分割
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("单词列表: {:?}", words);
    
    // 查找包含特定字符的单词
    let words_with_o: Vec<&str> = words.iter()
        .filter(|word| word.contains('o'))
        .cloned()
        .collect();
    println!("包含'o'的单词: {:?}", words_with_o);
    
    // 切片模式匹配
    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.as_slice() {
        [] => println!("空切片"),
        [x] => println!("只有一个元素: {}", x),
        [x, y] => println!("两个元素: {}, {}", x, y),
        [x, .., y] => println!("第一个和最后一个: {}, {}", x, y),
        _ => println!("其他情况"),
    }
    
    // ========== 字符串处理示例 ==========
    println!("\n--- 字符串处理示例 ---");
    
    let sentence = "Rust is a systems programming language";
    
    // 提取每个单词的第一个字符
    let first_chars: String = sentence
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect();
    println!("每个单词的首字母: {}", first_chars);
    
    // 反转每个单词
    let reversed_words: Vec<String> = sentence
        .split_whitespace()
        .map(|word| word.chars().rev().collect())
        .collect();
    println!("反转的单词: {:?}", reversed_words);
    
    // ========== 引用计数和智能指针预览 ==========
    println!("\n--- 智能指针预览 ---");
    
    use std::rc::Rc;
    
    // Rc<T> 允许多个所有者
    let data = Rc::new(String::from("shared data"));
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);
    
    println!("共享数据: {}", data);
    println!("引用计数: {}", Rc::strong_count(&data));
    
    drop(data1);
    println!("删除一个引用后的计数: {}", Rc::strong_count(&data));
    
    println!("\n=== 引用与切片要点总结 ===");
    println!("1. 引用允许使用值而不获取所有权");
    println!("2. &T 是不可变引用，&mut T 是可变引用");
    println!("3. 切片是对连续数据的引用，如 &str 和 &[T]");
    println!("4. 切片提供了安全访问部分数据的方式");
    println!("5. 字符串字面量本身就是切片（&str）");
    println!("6. 切片在函数参数中提供了灵活性");
}

// ========== 辅助函数 ==========

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

// 计算切片的和
fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// 查找切片中的最大值
fn find_max(slice: &[i32]) -> Option<&i32> {
    slice.iter().max()
}

// 字符串切片处理函数
fn extract_middle(s: &str) -> &str {
    let len = s.len();
    if len < 3 {
        s
    } else {
        let start = len / 4;
        let end = len - len / 4;
        &s[start..end]
    }
}

// 安全的字符边界切片
fn safe_substring(s: &str, start: usize, len: usize) -> &str {
    let mut char_indices = s.char_indices();
    
    let start_byte = char_indices
        .nth(start)
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    
    let end_byte = char_indices
        .nth(len.saturating_sub(1))
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    
    &s[start_byte..end_byte]
}

// 分割字符串为行
fn split_lines(text: &str) -> Vec<&str> {
    text.lines().collect()
}

// 过滤和转换切片
fn filter_and_double(slice: &[i32]) -> Vec<i32> {
    slice.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}
