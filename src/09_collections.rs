// 09_collections.rs
// Rust 常见集合类型及常用操作详解

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList};

fn main() {
    println!("=== Rust 集合类型详解 ===\n");
    
    // ========== Vector (动态数组) ==========
    println!("--- Vector (动态数组) ---");
    
    // 创建Vector
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4, 5]; // 使用宏创建
    
    println!("初始vector: {:?}", vec2);
    
    // 添加元素
    vec1.push(10);
    vec1.push(20);
    vec1.push(30);
    vec2.push(6);
    
    println!("添加元素后: vec1={:?}, vec2={:?}", vec1, vec2);
    
    // 访问元素
    let third = &vec2[2]; // 直接索引，可能panic
    println!("第三个元素: {}", third);
    
    match vec2.get(2) { // 安全访问
        Some(value) => println!("安全访问第三个元素: {}", value),
        None => println!("索引超出范围"),
    }
    
    // 遍历Vector
    println!("遍历vector元素:");
    for (i, value) in vec2.iter().enumerate() {
        println!("  索引{}: {}", i, value);
    }
    
    // 可变遍历
    for value in &mut vec2 {
        *value *= 2;
    }
    println!("翻倍后: {:?}", vec2);
    
    // Vector的常用方法
    println!("vector长度: {}", vec2.len());
    println!("vector容量: {}", vec2.capacity());
    println!("是否为空: {}", vec2.is_empty());
    
    // 弹出元素
    if let Some(last) = vec2.pop() {
        println!("弹出的元素: {}", last);
    }
    println!("弹出后: {:?}", vec2);
    
    // Vector切片操作
    let slice = &vec2[1..4];
    println!("切片 [1..4]: {:?}", slice);
    
    // Vector的高级操作
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 过滤
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // 映射
    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("平方: {:?}", squared);
    
    // 折叠
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("和: {}, 积: {}", sum, product);
    
    // ========== String 和字符串处理 ==========
    println!("\n--- String 和字符串处理 ---");
    
    // 创建String
    let mut s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "World".to_string();
    
    // 字符串追加
    s1.push_str("Rust");
    s1.push(' ');
    s1.push_str("Programming");
    
    println!("构建的字符串: {}", s1);
    
    // 字符串连接
    let combined = s2 + " " + &s3; // s2被移动
    println!("连接的字符串: {}", combined);
    
    let formatted = format!("{} {} {}", "Hello", "Rust", "World");
    println!("格式化字符串: {}", formatted);
    
    // 字符串切片和迭代
    let text = "Hello, 世界! 🦀";
    println!("原字符串: {}", text);
    
    // 按字符迭代
    println!("字符迭代:");
    for (i, ch) in text.chars().enumerate() {
        println!("  {}. '{}'", i, ch);
    }
    
    // 按字节迭代
    println!("字节数: {}", text.len());
    
    // 字符串搜索和替换
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("原句: {}", sentence);
    
    if sentence.contains("fox") {
        println!("包含 'fox'");
    }
    
    let replaced = sentence.replace("fox", "cat");
    println!("替换后: {}", replaced);
    
    // 字符串分割
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("单词: {:?}", words);
    
    let parts: Vec<&str> = sentence.split("the").collect();
    println!("按'the'分割: {:?}", parts);
    
    // ========== HashMap (哈希映射) ==========
    println!("\n--- HashMap (哈希映射) ---");
    
    // 创建HashMap
    let mut scores = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);
    
    println!("分数映射: {:?}", scores);
    
    // 从Vector创建HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut team_scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    
    println!("团队分数: {:?}", team_scores);
    
    // 访问值
    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("{}队的分数: {}", team_name, score);
    }
    
    // 遍历HashMap
    println!("所有分数:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // 更新HashMap
    scores.insert(String::from("Blue"), 15); // 覆盖旧值
    
    // 只在键不存在时插入
    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(100); // 不会插入，因为已存在
    
    println!("更新后的分数: {:?}", scores);
    
    // 基于旧值更新
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("单词计数: {:?}", word_count);
    
    // ========== HashSet (哈希集合) ==========
    println!("\n--- HashSet (哈希集合) ---");
    
    let mut set1 = HashSet::new();
    set1.insert("apple");
    set1.insert("banana");
    set1.insert("cherry");
    set1.insert("apple"); // 重复插入，会被忽略
    
    println!("水果集合: {:?}", set1);
    println!("集合大小: {}", set1.len());
    
    // 检查是否包含元素
    if set1.contains("apple") {
        println!("集合包含苹果");
    }
    
    // 集合操作
    let set2: HashSet<&str> = ["banana", "date", "elderberry"].iter().cloned().collect();
    println!("第二个集合: {:?}", set2);
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("交集: {:?}", intersection);
    
    // 并集
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("并集: {:?}", union);
    
    // 差集
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("差集 (set1 - set2): {:?}", difference);
    
    // 对称差集
    let symmetric_diff: HashSet<_> = set1.symmetric_difference(&set2).collect();
    println!("对称差集: {:?}", symmetric_diff);
    
    // ========== BTreeMap (有序映射) ==========
    println!("\n--- BTreeMap (有序映射) ---");
    
    let mut ordered_map = BTreeMap::new();
    ordered_map.insert(3, "three");
    ordered_map.insert(1, "one");
    ordered_map.insert(4, "four");
    ordered_map.insert(2, "two");
    
    println!("有序映射 (按键排序):");
    for (key, value) in &ordered_map {
        println!("  {}: {}", key, value);
    }
    
    // 范围查询
    let range: BTreeMap<_, _> = ordered_map.range(2..=3).collect();
    println!("范围 [2, 3]: {:?}", range);
    
    // ========== VecDeque (双端队列) ==========
    println!("\n--- VecDeque (双端队列) ---");
    
    let mut deque = VecDeque::new();
    
    // 从两端添加元素
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.push_front(-1);
    
    println!("双端队列: {:?}", deque);
    
    // 从两端弹出元素
    if let Some(front) = deque.pop_front() {
        println!("从前端弹出: {}", front);
    }
    
    if let Some(back) = deque.pop_back() {
        println!("从后端弹出: {}", back);
    }
    
    println!("弹出后的双端队列: {:?}", deque);
    
    // ========== 集合的实际应用示例 ==========
    println!("\n--- 实际应用示例 ---");
    
    // 学生成绩管理系统
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", vec![85, 90, 78, 92]);
    student_grades.insert("Bob", vec![76, 88, 83, 79]);
    student_grades.insert("Charlie", vec![92, 87, 94, 89]);
    
    println!("学生成绩管理:");
    for (student, grades) in &student_grades {
        let average: f64 = grades.iter().sum::<i32>() as f64 / grades.len() as f64;
        println!("  {}: {:?}, 平均分: {:.1}", student, grades, average);
    }
    
    // 找出优秀学生（平均分>=85）
    let excellent_students: Vec<&str> = student_grades.iter()
        .filter(|(_, grades)| {
            let avg = grades.iter().sum::<i32>() as f64 / grades.len() as f64;
            avg >= 85.0
        })
        .map(|(name, _)| *name)
        .collect();
    
    println!("优秀学生: {:?}", excellent_students);
    
    // 文本分析示例
    let text = "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.";
    
    // 统计单词频率
    let mut word_frequency = HashMap::new();
    for word in text.split_whitespace() {
        let clean_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        *word_frequency.entry(clean_word).or_insert(0) += 1;
    }
    
    println!("单词频率统计:");
    let mut sorted_words: Vec<_> = word_frequency.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1)); // 按频率降序排序
    
    for (word, count) in sorted_words.iter().take(5) {
        println!("  '{}': {} 次", word, count);
    }
    
    // 数据去重和排序
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("原始数据: {:?}", numbers);
    
    // 使用HashSet去重
    let unique_numbers: HashSet<i32> = numbers.iter().cloned().collect();
    let mut sorted_unique: Vec<i32> = unique_numbers.into_iter().collect();
    sorted_unique.sort();
    
    println!("去重并排序: {:?}", sorted_unique);
    
    // 多种集合类型比较
    println!("\n--- 集合性能特征对比 ---");
    demonstrate_collection_characteristics();
    
    println!("\n=== 集合类型要点总结 ===");
    println!("1. Vector: 动态数组，连续内存存储，支持索引访问");
    println!("2. HashMap: 哈希映射，O(1)平均查找时间，无序");
    println!("3. HashSet: 哈希集合，快速查找和去重");
    println!("4. BTreeMap/BTreeSet: 有序映射/集合，基于B树");
    println!("5. VecDeque: 双端队列，两端操作都很高效");
    println!("6. String: 可增长的UTF-8字符串");
}

// 集合特征演示函数
fn demonstrate_collection_characteristics() {
    // Vector - 适合索引访问和末尾操作
    let mut vec = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec.push(i);
    }
    println!("Vector: 容量预分配，末尾添加O(1)，索引访问O(1)");
    
    // HashMap - 适合键值查找
    let mut map = HashMap::new();
    for i in 0..100 {
        map.insert(format!("key{}", i), i);
    }
    println!("HashMap: 平均O(1)插入和查找，但无序");
    
    // BTreeMap - 适合需要有序的场景
    let mut btree = BTreeMap::new();
    for i in (0..100).rev() {
        btree.insert(i, format!("value{}", i));
    }
    println!("BTreeMap: O(log n)操作，但保持键的有序性");
    
    // VecDeque - 适合队列操作
    let mut deque = VecDeque::new();
    for i in 0..10 {
        if i % 2 == 0 {
            deque.push_back(i);
        } else {
            deque.push_front(i);
        }
    }
    println!("VecDeque: 两端操作都是O(1)，适合队列和栈");
}

// 实用的集合操作函数
fn analyze_text(text: &str) -> (usize, usize, HashMap<char, usize>) {
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    
    let mut char_frequency = HashMap::new();
    for ch in text.chars() {
        if ch.is_alphabetic() {
            *char_frequency.entry(ch.to_lowercase().next().unwrap()).or_insert(0) += 1;
        }
    }
    
    (word_count, char_count, char_frequency)
}

// 集合转换示例
fn collection_conversions() {
    let vec = vec![1, 2, 3, 2, 1];
    
    // Vector 转 HashSet（去重）
    let set: HashSet<i32> = vec.iter().cloned().collect();
    
    // HashSet 转回 Vector
    let mut unique_vec: Vec<i32> = set.into_iter().collect();
    unique_vec.sort();
    
    // Vector 转 HashMap（值作为键）
    let map: HashMap<i32, usize> = vec.iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect();
    
    println!("转换示例完成");
}
