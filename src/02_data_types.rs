// 02_data_types.rs
// Rust 数据类型详解：标量与复合类型

fn main() {
    println!("=== Rust 数据类型详解 ===\n");
    
    // ========== 标量类型 ==========
    println!("--- 标量类型 ---");
    
    // 1. 整数类型
    let signed_8: i8 = -128;        // 8位有符号整数 (-128 到 127)
    let unsigned_8: u8 = 255;       // 8位无符号整数 (0 到 255)
    let signed_32: i32 = -2147483648; // 32位有符号整数（默认整数类型）
    let unsigned_64: u64 = 18446744073709551615; // 64位无符号整数
    
    println!("整数类型:");
    println!("i8: {}, u8: {}, i32: {}, u64: {}", signed_8, unsigned_8, signed_32, unsigned_64);
    
    // 整数字面量的不同表示法
    let decimal = 98_222;           // 十进制，可使用下划线分隔
    let hex = 0xff;                 // 十六进制
    let octal = 0o77;               // 八进制
    let binary = 0b1111_0000;       // 二进制
    let byte = b'A';                // 字节（仅限u8）
    
    println!("字面量: 十进制={}, 十六进制={}, 八进制={}, 二进制={}, 字节={}", 
             decimal, hex, octal, binary, byte);
    
    // 2. 浮点类型
    let float_32: f32 = 3.14159;    // 32位浮点数
    let float_64: f64 = 2.718281828; // 64位浮点数（默认浮点类型）
    
    println!("浮点类型: f32={}, f64={}", float_32, float_64);
    
    // 3. 布尔类型
    let is_rust_awesome: bool = true;
    let is_learning: bool = false;
    
    println!("布尔类型: is_rust_awesome={}, is_learning={}", is_rust_awesome, is_learning);
    
    // 4. 字符类型
    let letter: char = 'R';
    let emoji: char = '🦀';
    let chinese: char = '中';
    
    println!("字符类型: letter='{}', emoji='{}', chinese='{}'", letter, emoji, chinese);
    
    // ========== 复合类型 ==========
    println!("\n--- 复合类型 ---");
    
    // 5. 元组类型
    let tuple: (i32, f64, char) = (500, 6.4, 'R');
    println!("完整元组: {:?}", tuple);
    
    // 元组解构
    let (x, y, z) = tuple;
    println!("解构元组: x={}, y={}, z={}", x, y, z);
    
    // 通过索引访问元组元素
    let first = tuple.0;
    let second = tuple.1;
    println!("索引访问: tuple.0={}, tuple.1={}", first, second);
    
    // 空元组（单元类型）
    let unit: () = ();
    println!("单元类型: {:?}", unit);
    
    // 6. 数组类型
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组: {:?}", array);
    
    // 相同值初始化数组
    let zeros = [0; 10]; // 创建包含10个0的数组
    println!("零数组前5个元素: {:?}", &zeros[0..5]);
    
    // 数组访问
    let first_element = array[0];
    let last_element = array[4];
    println!("数组访问: 第一个={}, 最后一个={}", first_element, last_element);
    
    // 数组长度
    println!("数组长度: {}", array.len());
    
    // 7. 字符串类型
    let string_literal: &str = "Hello, Rust!";     // 字符串字面量（字符串切片）
    let owned_string: String = String::from("Hello, World!"); // 拥有所有权的字符串
    
    println!("字符串字面量: {}", string_literal);
    println!("String类型: {}", owned_string);
    
    // 8. 切片类型
    let slice: &[i32] = &array[1..4]; // 数组切片
    println!("数组切片: {:?}", slice);
    
    let string_slice: &str = &owned_string[0..5]; // 字符串切片
    println!("字符串切片: {}", string_slice);
    
    // ========== 类型转换 ==========
    println!("\n--- 类型转换 ---");
    
    // 显式类型转换（as关键字）
    let integer = 65u8;
    let character = integer as char;
    let float = integer as f64;
    
    println!("类型转换: {}(u8) -> '{}'(char), {}(f64)", integer, character, float);
    
    // 解析字符串为数字
    let parsed_number: i32 = "42".parse().expect("不是一个有效的数字");
    println!("字符串解析: {}", parsed_number);
    
    // ========== 类型推导 ==========
    println!("\n--- 类型推导 ---");
    
    let inferred_int = 42;          // Rust推导为i32
    let inferred_float = 3.14;      // Rust推导为f64
    let inferred_bool = true;       // Rust推导为bool
    
    println!("类型推导: int={}, float={}, bool={}", inferred_int, inferred_float, inferred_bool);
    
    // 类型注解
    let explicit: u64 = 100;
    let vector: Vec<i32> = vec![1, 2, 3];
    
    println!("显式类型: u64={}, Vec<i32>={:?}", explicit, vector);
    
    println!("\n=== 数据类型要点总结 ===");
    println!("1. 标量类型：整数、浮点数、布尔值、字符");
    println!("2. 复合类型：元组、数组、字符串、切片");
    println!("3. Rust是静态类型语言，具有强大的类型推导能力");
    println!("4. 使用as进行显式类型转换，parse()解析字符串");
}
