// 04_control_flow.rs
// Rust 流程控制详解

fn main() {
    println!("=== Rust 流程控制详解 ===\n");
    
    // ========== if 表达式 ==========
    println!("--- if 表达式 ---");
    
    let number = 7;
    
    // 基本 if 表达式
    if number < 5 {
        println!("数字小于5");
    } else if number < 10 {
        println!("数字在5到10之间");
    } else {
        println!("数字大于等于10");
    }
    
    // if 是表达式，可以返回值
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("if表达式的结果: {}", result);
    
    // 复杂条件判断
    let x = 4;
    let y = 6;
    
    if x > 0 && y > 0 {
        println!("x和y都是正数");
    }
    
    if x == 4 || y == 4 {
        println!("x或y等于4");
    }
    
    if !(x > 10) {
        println!("x不大于10");
    }
    
    // ========== loop 循环 ==========
    println!("\n--- loop 循环 ---");
    
    let mut counter = 0;
    
    // 无限循环，需要break退出
    let loop_result = loop {
        counter += 1;
        
        if counter == 3 {
            continue; // 跳过本次迭代
        }
        
        if counter == 5 {
            break counter * 2; // 跳出循环并返回值
        }
        
        println!("loop计数器: {}", counter);
    };
    
    println!("loop返回值: {}", loop_result);
    
    // 嵌套循环与标签
    let mut count = 0;
    'outer: loop {
        println!("外部循环: {}", count);
        let mut remaining = 10;
        
        loop {
            println!("  内部循环: {}", remaining);
            if remaining == 9 {
                break; // 只跳出内部循环
            }
            if count == 2 {
                break 'outer; // 跳出外部循环
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    // ========== while 循环 ==========
    println!("\n--- while 循环 ---");
    
    let mut number = 3;
    
    while number != 0 {
        println!("while计数: {}!", number);
        number -= 1;
    }
    
    println!("while循环结束!");
    
    // while 条件循环
    let mut stack = vec![1, 2, 3];
    
    while let Some(top) = stack.pop() {
        println!("从栈中弹出: {}", top);
    }
    
    // ========== for 循环 ==========
    println!("\n--- for 循环 ---");
    
    // 遍历数组
    let array = [10, 20, 30, 40, 50];
    
    for element in array {
        println!("数组元素: {}", element);
    }
    
    // 使用索引遍历
    for (index, value) in array.iter().enumerate() {
        println!("索引{}: 值{}", index, value);
    }
    
    // 范围遍历
    for number in 1..4 {
        println!("范围数字: {}", number);
    }
    
    // 包含结束值的范围
    for number in 1..=3 {
        println!("包含范围: {}", number);
    }
    
    // 反向遍历
    for number in (1..4).rev() {
        println!("反向: {}", number);
    }
    
    // 遍历字符串
    let text = "hello";
    for ch in text.chars() {
        println!("字符: {}", ch);
    }
    
    // 遍历集合
    let vec = vec!["apple", "banana", "cherry"];
    for fruit in &vec {
        println!("水果: {}", fruit);
    }
    
    // ========== match 表达式 ==========
    println!("\n--- match 表达式 ---");
    
    let value = 3;
    
    // 基本 match
    match value {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他数字"), // 默认分支
    }
    
    // match 返回值
    let description = match value {
        1 => "第一",
        2 => "第二", 
        3 => "第三",
        4..=10 => "四到十",
        _ => "其他",
    };
    println!("描述: {}", description);
    
    // 匹配多个值
    let day = 3;
    match day {
        1 | 2 | 3 | 4 | 5 => println!("工作日"),
        6 | 7 => println!("周末"),
        _ => println!("无效的日期"),
    }
    
    // 匹配范围
    let score = 85;
    match score {
        90..=100 => println!("优秀"),
        80..=89 => println!("良好"),
        70..=79 => println!("中等"),
        60..=69 => println!("及格"),
        _ => println!("不及格"),
    }
    
    // 匹配元组
    let point = (0, 1);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在Y轴上，y = {}", y),
        (x, 0) => println!("在X轴上，x = {}", x),
        (x, y) => println!("点({}, {})", x, y),
    }
    
    // 匹配选项类型
    let some_number = Some(5);
    match some_number {
        Some(x) if x > 3 => println!("大于3的数: {}", x),
        Some(x) => println!("数字: {}", x),
        None => println!("没有值"),
    }
    
    // ========== if let 和 while let ==========
    println!("\n--- if let 和 while let ---");
    
    // if let 简化 match
    let some_value = Some(3);
    if let Some(x) = some_value {
        println!("if let 匹配到: {}", x);
    }
    
    // while let 循环
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 3 {
            println!("退出while let循环");
            optional = None;
        } else {
            println!("while let: {}", i);
            optional = Some(i + 1);
        }
    }
    
    // ========== 实用示例 ==========
    println!("\n--- 实用示例 ---");
    
    // 猜数字游戏的核心逻辑示例
    let secret_number = 50;
    let guess = 42;
    
    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("太小了!"),
        std::cmp::Ordering::Greater => println!("太大了!"),
        std::cmp::Ordering::Equal => println!("你赢了!"),
    }
    
    // 处理结果类型
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(value) => println!("成功: {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    // 复杂的控制流组合
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut even_count = 0;
    let mut odd_sum = 0;
    
    for num in numbers {
        if num % 2 == 0 {
            even_count += 1;
            if even_count > 3 {
                break; // 找到3个以上偶数就停止
            }
        } else {
            odd_sum += num;
        }
    }
    
    println!("偶数个数: {}, 奇数和: {}", even_count, odd_sum);
    
    // 使用标签控制嵌套循环
    'search: for i in 1..=5 {
        for j in 1..=5 {
            if i * j > 15 {
                println!("找到乘积大于15的组合: {} × {} = {}", i, j, i * j);
                break 'search;
            }
        }
    }
    
    println!("\n=== 流程控制要点总结 ===");
    println!("1. if是表达式，可以返回值");
    println!("2. loop创建无限循环，while有条件循环，for遍历集合");
    println!("3. match是强大的模式匹配，必须穷尽所有可能");
    println!("4. if let和while let简化简单的模式匹配");
    println!("5. break和continue控制循环流程，可以使用标签");
}
