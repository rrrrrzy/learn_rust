// 07_structs_and_methods.rs
// Rust 结构体与方法详解

// 基本结构体定义
#[derive(Debug, Clone)] // 自动实现Debug和Clone trait
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// 单元结构体（没有字段）
#[derive(Debug)]
struct AlwaysEqual;

// 带有不同类型字段的结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 结构体方法定义
impl Rectangle {
    // 关联函数（构造器）
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 正方形构造器
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 方法（需要&self参数）
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 方法可以获取不可变引用
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // 方法可以获取可变引用
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    // 方法可以获取所有权
    fn max_dimension(self) -> u32 {
        if self.width > self.height {
            self.width
        } else {
            self.height
        }
    }
    
    // 方法可以接受其他参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 返回字段的引用
    fn width(&self) -> u32 {
        self.width
    }
    
    // 判断是否为正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// 为同一结构体实现多个impl块
impl Rectangle {
    // 计算对角线长度
    fn diagonal(&self) -> f64 {
        ((self.width.pow(2) + self.height.pow(2)) as f64).sqrt()
    }
    
    // 缩放到指定面积
    fn scale_to_area(&mut self, target_area: u32) {
        let current_area = self.area();
        if current_area > 0 {
            let factor = ((target_area as f64) / (current_area as f64)).sqrt();
            self.width = (self.width as f64 * factor) as u32;
            self.height = (self.height as f64 * factor) as u32;
        }
    }
}

// 用于演示的复杂结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

impl Person {
    fn new(name: String, age: u32, address: Address) -> Person {
        Person { name, age, address }
    }
    
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I'm {} years old", self.name, self.age)
    }
    
    fn have_birthday(&mut self) {
        self.age += 1;
    }
    
    fn move_to(&mut self, new_address: Address) {
        self.address = new_address;
    }
    
    fn full_address(&self) -> String {
        format!("{}, {}, {}", 
                self.address.street, 
                self.address.city, 
                self.address.zip_code)
    }
}

impl Address {
    fn new(street: String, city: String, zip_code: String) -> Address {
        Address { street, city, zip_code }
    }
    
    fn is_same_city(&self, other: &Address) -> bool {
        self.city == other.city
    }
}

fn main() {
    println!("=== Rust 结构体与方法详解 ===\n");
    
    // ========== 基本结构体使用 ==========
    println!("--- 基本结构体使用 ---");
    
    // 创建结构体实例
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("用户1: {:?}", user1);
    
    // 访问结构体字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    
    // 创建可变结构体实例
    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user456"),
        active: true,
        sign_in_count: 1,
    };
    
    // 修改可变字段
    user2.email = String::from("newemail@example.com");
    user2.sign_in_count += 1;
    
    println!("修改后的用户2: {:?}", user2);
    
    // ========== 结构体更新语法 ==========
    println!("\n--- 结构体更新语法 ---");
    
    // 使用结构体更新语法创建新实例
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user789"),
        ..user1 // 其余字段使用user1的值
    };
    
    println!("用户3（部分来自用户1）: {:?}", user3);
    
    // 注意：user1的String字段被移动了，所以user1不再可用
    // println!("{:?}", user1); // 这会报错
    
    // ========== 元组结构体 ==========
    println!("\n--- 元组结构体 ---");
    
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    
    println!("黑色: {:?}", black);
    println!("白色: {:?}", white);
    println!("原点: {:?}", origin);
    
    // 访问元组结构体字段
    println!("黑色的红色分量: {}", black.0);
    println!("原点的x坐标: {}", origin.0);
    
    // 解构元组结构体
    let Color(r, g, b) = white;
    println!("白色分量: r={}, g={}, b={}", r, g, b);
    
    // ========== 单元结构体 ==========
    println!("\n--- 单元结构体 ---");
    
    let subject = AlwaysEqual;
    println!("单元结构体: {:?}", subject);
    
    // ========== 方法调用 ==========
    println!("\n--- 方法调用 ---");
    
    // 使用关联函数创建实例
    let rect1 = Rectangle::new(30, 50);
    let square1 = Rectangle::square(25);
    
    println!("矩形1: {:?}", rect1);
    println!("正方形1: {:?}", square1);
    
    // 调用方法
    println!("矩形1面积: {}", rect1.area());
    println!("矩形1周长: {}", rect1.perimeter());
    println!("矩形1对角线: {:.2}", rect1.diagonal());
    println!("矩形1是正方形吗？{}", rect1.is_square());
    
    // 可变方法调用
    let mut rect2 = Rectangle::new(10, 20);
    println!("缩放前: {:?}", rect2);
    rect2.scale(2);
    println!("缩放后: {:?}", rect2);
    
    // 比较方法
    let small_rect = Rectangle::new(5, 10);
    println!("大矩形能容纳小矩形吗？{}", rect2.can_hold(&small_rect));
    
    // 获取所有权的方法（注意rect2在此后不可用）
    let max_dim = rect2.max_dimension();
    println!("最大维度: {}", max_dim);
    // println!("{:?}", rect2); // 这会报错，因为rect2已被移动
    
    // ========== 复杂结构体示例 ==========
    println!("\n--- 复杂结构体示例 ---");
    
    let address1 = Address::new(
        String::from("123 Main St"),
        String::from("Anytown"),
        String::from("12345"),
    );
    
    let mut person1 = Person::new(
        String::from("Alice"),
        25,
        address1,
    );
    
    println!("人员信息: {:?}", person1);
    println!("问候: {}", person1.greet());
    println!("完整地址: {}", person1.full_address());
    
    // 修改人员信息
    person1.have_birthday();
    println!("生日后: {}", person1.greet());
    
    let new_address = Address::new(
        String::from("456 Oak Ave"),
        String::from("Other City"),
        String::from("67890"),
    );
    
    person1.move_to(new_address);
    println!("搬家后地址: {}", person1.full_address());
    
    // ========== 结构体作为函数参数 ==========
    println!("\n--- 结构体作为函数参数 ---");
    
    let rect3 = Rectangle::new(15, 25);
    let rect4 = Rectangle::new(20, 30);
    
    println!("矩形3面积: {}", calculate_area(&rect3));
    println!("矩形面积差: {}", area_difference(&rect3, &rect4));
    
    let larger = larger_rectangle(&rect3, &rect4);
    println!("更大的矩形: {:?}", larger);
    
    // ========== 结构体数组和向量 ==========
    println!("\n--- 结构体集合 ---");
    
    let rectangles = vec![
        Rectangle::new(10, 20),
        Rectangle::new(15, 15),
        Rectangle::new(8, 12),
        Rectangle::square(10),
    ];
    
    println!("矩形列表:");
    for (i, rect) in rectangles.iter().enumerate() {
        println!("  矩形{}: {:?}, 面积: {}, 是正方形: {}", 
                 i + 1, rect, rect.area(), rect.is_square());
    }
    
    let total_area: u32 = rectangles.iter().map(|r| r.area()).sum();
    println!("总面积: {}", total_area);
    
    let squares: Vec<&Rectangle> = rectangles.iter()
        .filter(|r| r.is_square())
        .collect();
    println!("正方形数量: {}", squares.len());
    
    // ========== 结构体字段初始化简写 ==========
    println!("\n--- 字段初始化简写 ---");
    
    let username = String::from("shorthand");
    let email = String::from("shorthand@example.com");
    
    // 当变量名与字段名相同时可以使用简写
    let user4 = User {
        username, // 等价于 username: username,
        email,    // 等价于 email: email,
        active: true,
        sign_in_count: 1,
    };
    
    println!("简写创建的用户: {:?}", user4);
    
    // ========== 工厂函数模式 ==========
    println!("\n--- 工厂函数模式 ---");
    
    let user5 = build_user(
        String::from("factory"),
        String::from("factory@example.com")
    );
    
    println!("工厂创建的用户: {:?}", user5);
    
    println!("\n=== 结构体与方法要点总结 ===");
    println!("1. 结构体使用struct定义，可以包含不同类型的字段");
    println!("2. impl块为结构体定义方法和关联函数");
    println!("3. 方法的第一个参数是&self、&mut self或self");
    println!("4. 关联函数不接受self参数，通常用作构造器");
    println!("5. 可以为同一结构体定义多个impl块");
    println!("6. 结构体更新语法允许基于现有实例创建新实例");
}

// ========== 辅助函数 ==========

// 计算矩形面积的函数
fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.area()
}

// 计算两个矩形面积差
fn area_difference(rect1: &Rectangle, rect2: &Rectangle) -> i32 {
    rect1.area() as i32 - rect2.area() as i32
}

// 返回较大的矩形
fn larger_rectangle<'a>(rect1: &'a Rectangle, rect2: &'a Rectangle) -> &'a Rectangle {
    if rect1.area() > rect2.area() {
        rect1
    } else {
        rect2
    }
}

// 工厂函数
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

// 创建默认矩形
fn default_rectangle() -> Rectangle {
    Rectangle::new(1, 1)
}

// 创建矩形数组
fn create_rectangles(count: usize) -> Vec<Rectangle> {
    (0..count)
        .map(|i| Rectangle::new((i + 1) as u32 * 10, (i + 1) as u32 * 5))
        .collect()
}
