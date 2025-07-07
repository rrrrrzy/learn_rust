// 16_async_programming.rs
// Rust 异步编程详解 (async/await)

// 注意：要运行异步代码，需要在Cargo.toml中添加tokio依赖
// [dependencies]
// tokio = { version = "1.0", features = ["full"] }

use std::time::Duration;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 简单的异步函数
async fn hello_async() {
    println!("Hello from async function!");
}

// 带延迟的异步函数
async fn delayed_greeting(name: &str, delay_ms: u64) {
    // 模拟异步延迟
    simulate_delay(delay_ms).await;
    println!("Hello, {}! (延迟 {}ms)", name, delay_ms);
}

// 模拟异步延迟的简单实现
async fn simulate_delay(ms: u64) {
    // 在实际项目中，你会使用 tokio::time::sleep(Duration::from_millis(ms)).await;
    // 这里我们用一个简单的计数来模拟
    for _ in 0..ms * 1000 {
        // 一些计算工作来模拟延迟
    }
}

// 返回 Future 的函数
fn get_number_async() -> impl Future<Output = i32> {
    async {
        simulate_delay(100).await;
        42
    }
}

// 错误处理的异步函数
async fn fallible_async_operation(should_fail: bool) -> Result<String, &'static str> {
    simulate_delay(50).await;
    
    if should_fail {
        Err("操作失败")
    } else {
        Ok("操作成功".to_string())
    }
}

// 并发执行多个异步任务
async fn concurrent_tasks() {
    println!("开始并发任务...");
    
    // 创建多个异步任务
    let task1 = delayed_greeting("Alice", 300);
    let task2 = delayed_greeting("Bob", 200);
    let task3 = delayed_greeting("Charlie", 100);
    
    // 并发执行所有任务（在真实环境中使用 tokio::join!）
    // 这里我们手动模拟
    println!("所有任务开始执行...");
    task3.await;
    task2.await;
    task1.await;
    println!("所有并发任务完成!");
}

// 异步迭代器示例
async fn process_items(items: Vec<i32>) -> Vec<i32> {
    let mut results = Vec::new();
    
    for item in items {
        let processed = process_item_async(item).await;
        results.push(processed);
    }
    
    results
}

async fn process_item_async(item: i32) -> i32 {
    simulate_delay(10).await;
    item * 2
}

// 自定义 Future 实现
struct TimerFuture {
    start_time: std::time::Instant,
    duration: Duration,
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            duration,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start_time.elapsed() >= self.duration {
            Poll::Ready(())
        } else {
            // 在真实实现中，这里需要注册唤醒
            Poll::Pending
        }
    }
}

// 异步流 (Stream) 示例概念
struct NumberStream {
    current: i32,
    max: i32,
}

impl NumberStream {
    fn new(max: i32) -> Self {
        Self { current: 0, max }
    }
    
    async fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            simulate_delay(100).await;
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

// 异步生产者-消费者模式示例
async fn producer_consumer_example() {
    println!("生产者-消费者示例:");
    
    // 模拟生产数据
    let data = produce_data().await;
    println!("生产的数据: {:?}", data);
    
    // 模拟消费数据
    let result = consume_data(data).await;
    println!("消费结果: {}", result);
}

async fn produce_data() -> Vec<i32> {
    println!("  生产者开始工作...");
    simulate_delay(200).await;
    
    let data = vec![1, 2, 3, 4, 5];
    println!("  生产完成!");
    data
}

async fn consume_data(data: Vec<i32>) -> i32 {
    println!("  消费者开始工作...");
    simulate_delay(100).await;
    
    let sum = data.iter().sum();
    println!("  消费完成!");
    sum
}

// 异步错误传播
async fn error_propagation_example() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let result1 = fallible_async_operation(false).await?;
    let result2 = fallible_async_operation(false).await?;
    
    Ok(format!("{} and {}", result1, result2))
}

// 异步递归
async fn async_factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        simulate_delay(10).await;
        n * async_factorial(n - 1).await
    }
}

// 条件编译：只在有tokio时编译
#[cfg(feature = "tokio")]
mod tokio_examples {
    use tokio::time::{sleep, Duration};
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    // 真实的异步延迟
    pub async fn real_delay(ms: u64) {
        sleep(Duration::from_millis(ms)).await;
    }
    
    // 异步文件操作
    pub async fn async_file_operations() -> Result<(), Box<dyn std::error::Error>> {
        // 异步写文件
        let mut file = File::create("async_test.txt").await?;
        file.write_all(b"Hello, async world!").await?;
        file.flush().await?;
        
        // 异步读文件
        let mut file = File::open("async_test.txt").await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        println!("文件内容: {}", contents);
        
        // 清理
        tokio::fs::remove_file("async_test.txt").await?;
        
        Ok(())
    }
    
    // 异步HTTP请求示例（需要reqwest crate）
    #[cfg(feature = "reqwest")]
    pub async fn http_request_example() -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get("https://httpbin.org/get").await?;
        let text = response.text().await?;
        println!("HTTP响应长度: {}", text.len());
        Ok(())
    }
    
    // 并发HTTP请求
    #[cfg(feature = "reqwest")]
    pub async fn concurrent_http_requests() -> Result<(), Box<dyn std::error::Error>> {
        let urls = vec![
            "https://httpbin.org/delay/1",
            "https://httpbin.org/delay/2",
            "https://httpbin.org/delay/1",
        ];
        
        let futures: Vec<_> = urls.into_iter()
            .map(|url| reqwest::get(url))
            .collect();
        
        let responses = futures::future::join_all(futures).await;
        
        for (i, response) in responses.into_iter().enumerate() {
            match response {
                Ok(resp) => println!("请求 {} 完成: {}", i, resp.status()),
                Err(e) => println!("请求 {} 失败: {}", i, e),
            }
        }
        
        Ok(())
    }
}

// 异步运行时模拟器
struct SimpleExecutor;

impl SimpleExecutor {
    fn run<F: Future>(future: F) -> F::Output {
        // 这是一个极简的执行器模拟
        // 在实际情况下，这里会有复杂的任务调度逻辑
        
        // 对于演示，我们只是阻塞等待
        futures::executor::block_on(future)
    }
}

// 主函数 - 同步入口点
fn main() {
    println!("=== Rust 异步编程详解 ===\n");
    
    // ========== 基本异步概念 ==========
    println!("--- 基本异步概念 ---");
    
    // 运行简单异步函数
    futures::executor::block_on(hello_async());
    
    // 带参数的异步函数
    futures::executor::block_on(async {
        delayed_greeting("World", 100).await;
    });
    
    // ========== 异步函数返回值 ==========
    println!("\n--- 异步函数返回值 ---");
    
    let number = futures::executor::block_on(get_number_async());
    println!("异步获取的数字: {}", number);
    
    // ========== 错误处理 ==========
    println!("\n--- 异步错误处理 ---");
    
    futures::executor::block_on(async {
        match fallible_async_operation(false).await {
            Ok(result) => println!("成功: {}", result),
            Err(error) => println!("失败: {}", error),
        }
        
        match fallible_async_operation(true).await {
            Ok(result) => println!("成功: {}", result),
            Err(error) => println!("失败: {}", error),
        }
    });
    
    // ========== 并发执行 ==========
    println!("\n--- 并发执行 ---");
    
    futures::executor::block_on(concurrent_tasks());
    
    // ========== 异步迭代处理 ==========
    println!("\n--- 异步迭代处理 ---");
    
    let items = vec![1, 2, 3, 4, 5];
    let results = futures::executor::block_on(process_items(items));
    println!("处理结果: {:?}", results);
    
    // ========== 自定义Future ==========
    println!("\n--- 自定义Future ---");
    
    println!("开始定时器...");
    let start = std::time::Instant::now();
    futures::executor::block_on(TimerFuture::new(Duration::from_millis(500)));
    println!("定时器完成，耗时: {:?}", start.elapsed());
    
    // ========== 异步流 ==========
    println!("\n--- 异步流 ---");
    
    futures::executor::block_on(async {
        let mut stream = NumberStream::new(5);
        while let Some(number) = stream.next().await {
            println!("流中的数字: {}", number);
        }
    });
    
    // ========== 生产者-消费者 ==========
    println!("\n--- 生产者-消费者 ---");
    
    futures::executor::block_on(producer_consumer_example());
    
    // ========== 错误传播 ==========
    println!("\n--- 错误传播 ---");
    
    futures::executor::block_on(async {
        match error_propagation_example().await {
            Ok(result) => println!("组合结果: {}", result),
            Err(error) => println!("组合失败: {}", error),
        }
    });
    
    // ========== 异步递归 ==========
    println!("\n--- 异步递归 ---");
    
    let factorial_result = futures::executor::block_on(async_factorial(5));
    println!("5的异步阶乘: {}", factorial_result);
    
    // ========== 异步编程模式 ==========
    println!("\n--- 异步编程模式 ---");
    
    demonstrate_async_patterns();
    
    // ========== 性能比较 ==========
    println!("\n--- 性能比较 ---");
    
    performance_comparison();
    
    println!("\n=== 异步编程要点总结 ===");
    println!("1. async/await 提供了编写异步代码的语法糖");
    println!("2. Future trait 是异步编程的核心抽象");
    println!("3. 异步函数返回实现了Future的类型");
    println!("4. .await 用于等待Future完成");
    println!("5. 异步代码需要异步运行时来执行");
    println!("6. 并发不等于并行，异步主要解决IO密集型任务");
    println!("7. 错误处理在异步代码中同样重要");
    println!("8. 异步编程适合高并发、IO密集型应用");
}

// ========== 异步编程模式演示 ==========

fn demonstrate_async_patterns() {
    println!("异步编程模式演示:");
    
    // 模式1: 顺序执行
    futures::executor::block_on(async {
        println!("  顺序执行:");
        let start = std::time::Instant::now();
        
        delayed_greeting("First", 100).await;
        delayed_greeting("Second", 100).await;
        delayed_greeting("Third", 100).await;
        
        println!("  顺序执行耗时: {:?}", start.elapsed());
    });
    
    // 模式2: 并发执行（模拟）
    futures::executor::block_on(async {
        println!("  并发执行:");
        let start = std::time::Instant::now();
        
        // 在真实项目中使用 tokio::join! 或 futures::join!
        let (_, _, _) = futures::join!(
            delayed_greeting("First", 100),
            delayed_greeting("Second", 100),
            delayed_greeting("Third", 100)
        );
        
        println!("  并发执行耗时: {:?}", start.elapsed());
    });
}

// 性能比较
fn performance_comparison() {
    println!("性能比较:");
    
    // 同步版本
    fn sync_operation(id: i32, duration_ms: u64) -> i32 {
        // 模拟同步阻塞操作
        std::thread::sleep(Duration::from_millis(duration_ms));
        id * 2
    }
    
    // 异步版本
    async fn async_operation(id: i32, duration_ms: u64) -> i32 {
        simulate_delay(duration_ms).await;
        id * 2
    }
    
    // 同步执行多个任务
    let start = std::time::Instant::now();
    let sync_results: Vec<i32> = (1..=3)
        .map(|i| sync_operation(i, 100))
        .collect();
    let sync_duration = start.elapsed();
    
    println!("  同步结果: {:?}, 耗时: {:?}", sync_results, sync_duration);
    
    // 异步执行多个任务
    let start = std::time::Instant::now();
    let async_results = futures::executor::block_on(async {
        let futures: Vec<_> = (1..=3)
            .map(|i| async_operation(i, 100))
            .collect();
        
        futures::future::join_all(futures).await
    });
    let async_duration = start.elapsed();
    
    println!("  异步结果: {:?}, 耗时: {:?}", async_results, async_duration);
}

// ========== 高级异步模式 ==========

// 异步状态机示例
enum AsyncStateMachine {
    Start,
    Processing(i32),
    Finished(String),
}

impl AsyncStateMachine {
    async fn run(&mut self) {
        loop {
            match self {
                AsyncStateMachine::Start => {
                    println!("状态机开始");
                    simulate_delay(100).await;
                    *self = AsyncStateMachine::Processing(0);
                }
                AsyncStateMachine::Processing(ref mut count) => {
                    *count += 1;
                    println!("处理中: {}", count);
                    simulate_delay(50).await;
                    
                    if *count >= 3 {
                        *self = AsyncStateMachine::Finished(format!("完成，处理了{}次", count));
                    }
                }
                AsyncStateMachine::Finished(result) => {
                    println!("状态机完成: {}", result);
                    break;
                }
            }
        }
    }
}

// 异步缓存示例
struct AsyncCache {
    data: std::collections::HashMap<String, String>,
}

impl AsyncCache {
    fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    
    async fn get(&self, key: &str) -> Option<String> {
        simulate_delay(10).await; // 模拟网络延迟
        self.data.get(key).cloned()
    }
    
    async fn set(&mut self, key: String, value: String) {
        simulate_delay(15).await; // 模拟写入延迟
        self.data.insert(key, value);
    }
}

// 异步工作队列
struct AsyncWorkQueue {
    tasks: Vec<Box<dyn Fn() -> String + Send + 'static>>,
}

impl AsyncWorkQueue {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    
    fn add_task<F>(&mut self, task: F)
    where
        F: Fn() -> String + Send + 'static,
    {
        self.tasks.push(Box::new(task));
    }
    
    async fn process_all(&mut self) -> Vec<String> {
        let mut results = Vec::new();
        
        for task in self.tasks.drain(..) {
            simulate_delay(50).await;
            let result = task();
            results.push(result);
        }
        
        results
    }
}

// 实际应用中的异步模式
mod real_world_patterns {
    use super::*;
    
    // 重试机制
    pub async fn retry_async<F, Fut, T, E>(
        mut operation: F,
        max_attempts: usize,
    ) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        let mut attempts = 0;
        
        loop {
            attempts += 1;
            
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempts >= max_attempts {
                        return Err(error);
                    }
                    
                    // 指数退避
                    let delay_ms = 100 * 2_u64.pow(attempts as u32 - 1);
                    simulate_delay(delay_ms).await;
                }
            }
        }
    }
    
    // 超时包装
    pub async fn with_timeout<F: Future>(
        future: F,
        timeout_ms: u64,
    ) -> Result<F::Output, &'static str> {
        // 在真实项目中使用 tokio::time::timeout
        // 这里简化实现
        
        let start = std::time::Instant::now();
        let result = future.await;
        
        if start.elapsed() > Duration::from_millis(timeout_ms) {
            Err("超时")
        } else {
            Ok(result)
        }
    }
    
    // 流式处理
    pub async fn stream_process<T, F, Fut>(
        items: Vec<T>,
        processor: F,
    ) -> Vec<T>
    where
        F: Fn(T) -> Fut,
        Fut: Future<Output = T>,
    {
        let mut results = Vec::new();
        
        for item in items {
            let processed = processor(item).await;
            results.push(processed);
        }
        
        results
    }
}
