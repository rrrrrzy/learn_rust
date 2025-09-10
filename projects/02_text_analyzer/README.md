# 项目2：文本分析工具

## 项目概述
本项目旨在巩固函数定义、控制流和基础所有权概念，通过分析文本内容来练习这些核心概念。

## 学习目标
- 掌握函数的定义、参数传递和返回值
- 练习各种控制流结构（if/else、loop、while、for、match）
- 理解字符串和字符的处理
- 学习基础的所有权和借用概念

## 功能要求

### 基础功能
1. **文本统计**: 字符数、单词数、行数统计
2. **字符分析**: 字母、数字、标点符号分布
3. **单词分析**: 最长/最短单词，单词频率
4. **文件处理**: 读取文件内容进行分析

### 进阶功能
1. **语言检测**: 简单的语言类型识别
2. **情感分析**: 基于关键词的简单情感分析
3. **文本清洗**: 去除无关字符，标准化文本
4. **批量处理**: 分析多个文件

## 核心概念练习

### 函数设计
```rust
// 练习函数参数和返回值
fn count_words(text: &str) -> usize {
    // 借用参数，返回计数
}

fn analyze_text(text: String) -> TextStats {
    // 获取所有权，返回结构体
}
```

### 控制流
```rust
// 练习不同的循环结构
for line in text.lines() { }
while condition { }
loop { }

// 练习模式匹配
match character {
    'a'..='z' => category = CharCategory::Lowercase,
    'A'..='Z' => category = CharCategory::Uppercase,
    '0'..='9' => category = CharCategory::Digit,
    _ => category = CharCategory::Other,
}
```

### 所有权基础
```rust
// 练习借用
fn process_text(text: &str) { }

// 练习所有权转移
fn take_ownership(text: String) -> String { }
```

## 运行方式
```bash
cd projects/02_text_analyzer
cargo run
```

## 扩展思路
1. 支持多种文件格式(PDF, DOC等)
2. 添加正则表达式支持
3. 实现文本相似度比较
4. 添加图形化统计报告
5. 支持实时文本流分析
