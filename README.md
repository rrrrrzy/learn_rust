# Rust 学习系列教程

这是一个系统性的 Rust 编程语言学习教程，包含 16 个核心主题的详细讲解和示例代码。

## 文件列表

1. **01_variables_and_mutability.rs** - 变量和可变性
2. **02_data_types.rs** - 数据类型（标量与复合）
3. **03_functions_and_scope.rs** - 函数与作用域
4. **04_control_flow.rs** - 流程控制
5. **05_ownership_and_borrowing.rs** - 所有权与借用
6. **06_references_and_slices.rs** - 引用与切片
7. **07_structs_and_methods.rs** - 结构体和方法
8. **08_enums_and_pattern_matching.rs** - 枚举与模式匹配
9. **09_collections.rs** - 常见集合类型及常用操作
10. **10_modules_and_packages.rs** - 模块系统和包管理
11. **11_error_handling.rs** - 错误处理
12. **12_generics.rs** - 泛型
13. **13_traits_and_trait_bounds.rs** - Trait 与 Trait Bound
14. **14_lifetimes.rs** - 生命周期
15. **15_standard_library.rs** - 常用标准库函数与实用宏
16. **16_async_programming.rs** - 异步编程（async/await）

## 如何使用

### 方法 1: 单独运行每个文件

将 `main.rs` 的内容替换为你想要学习的文件内容，然后运行：

```bash
cargo run
```

### 方法 2: 编译为独立的二进制文件

在 `Cargo.toml` 中添加 `[[bin]]` 配置：

```toml
[[bin]]
name = "variables"
path = "src/01_variables_and_mutability.rs"

[[bin]]
name = "data_types"
path = "src/02_data_types.rs"

# ... 为每个文件添加类似配置
```

然后可以单独运行：

```bash
cargo run --bin variables
cargo run --bin data_types
```

### 方法 3: 使用条件编译

在 `main.rs` 中使用条件编译：

```rust
fn main() {
    #[cfg(feature = "lesson1")]
    include!("01_variables_and_mutability.rs");
    
    #[cfg(feature = "lesson2")]
    include!("02_data_types.rs");
    
    // ... 其他课程
}
```

然后运行：

```bash
cargo run --features lesson1
cargo run --features lesson2
```

## 学习建议

1. **按顺序学习**：文件按照难度和概念依赖关系排列，建议按顺序学习
2. **动手实践**：每个文件都包含可运行的代码，建议亲自运行和修改
3. **理解注释**：每个文件都有详细的中文注释，帮助理解概念
4. **实验代码**：尝试修改示例代码，观察结果变化
5. **总结要点**：每个文件末尾都有要点总结，可作为复习材料

## 依赖说明

大部分文件只使用 Rust 标准库，但有些文件可能需要额外依赖：

### 15_standard_library.rs
可能需要以下依赖（在 `Cargo.toml` 中添加）：

```toml
[dependencies]
# 基础依赖（可选）
serde = { version = "1.0", features = ["derive"] }
```

### 16_async_programming.rs
需要异步运行时：

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# HTTP 请求示例（可选）
reqwest = { version = "0.11", features = ["json"] }
```

## 每个文件的学习重点

| 文件 | 重点内容 | 难度 |
|------|----------|------|
| 01 | 变量声明、可变性、遮蔽 | ⭐ |
| 02 | 基本数据类型、类型转换 | ⭐ |
| 03 | 函数定义、作用域、闭包 | ⭐⭐ |
| 04 | if/loop/match 控制流 | ⭐⭐ |
| 05 | 所有权系统、借用检查 | ⭐⭐⭐ |
| 06 | 引用类型、切片操作 | ⭐⭐⭐ |
| 07 | 结构体、方法、关联函数 | ⭐⭐ |
| 08 | 枚举、模式匹配、Option/Result | ⭐⭐⭐ |
| 09 | Vec、HashMap、迭代器 | ⭐⭐ |
| 10 | 模块系统、包管理、可见性 | ⭐⭐⭐ |
| 11 | Result/Option、错误传播 | ⭐⭐⭐ |
| 12 | 泛型函数、泛型结构体 | ⭐⭐⭐⭐ |
| 13 | Trait 定义、Trait 约束 | ⭐⭐⭐⭐ |
| 14 | 生命周期注解、借用检查 | ⭐⭐⭐⭐⭐ |
| 15 | 标准库 API、宏编程 | ⭐⭐⭐ |
| 16 | 异步编程、Future、并发 | ⭐⭐⭐⭐⭐ |

## 常见问题

### Q: 编译错误怎么办？
A: 确保你的 Rust 版本是最新的：`rustup update`

### Q: 某些示例无法运行？
A: 检查是否缺少依赖，参考上面的依赖说明

### Q: 如何查看更多文档？
A: 使用 `cargo doc --open` 查看本地文档

### Q: 学习顺序可以调整吗？
A: 前5个文件建议按顺序学习，后面的可以根据兴趣调整

## 额外资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Rust 示例集](https://doc.rust-lang.org/stable/rust-by-example/)
- [Rustlings 练习](https://github.com/rust-lang/rustlings)

## 贡献

如果你发现任何错误或有改进建议，欢迎提交 issue 或 pull request。

## 许可证

本教程采用 MIT 许可证，你可以自由使用、修改和分发。

---

祝你学习 Rust 愉快！🦀
