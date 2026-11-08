//! 12 异步编程（async / await + tokio）
//!
//! async fn 返回一个 Future：**不调用 .await 就不会执行**。
//! 单线程也能同时等待很多 IO，这是异步的价值（不是"更快"，是"更省线程"）。
//! 运行：./run.sh 12
//!
//! 第一次运行会下载 tokio 及其依赖（已配置国内镜像，通常几秒到几十秒）。

use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};

/// 模拟一次耗时 IO：sleep 不阻塞线程，只是把控制权交回运行时
async fn load(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{} 完成（{}ms）", name, ms)
}

/// 模拟一个会失败的任务
async fn may_fail(ok: bool) -> Result<&'static str, String> {
    sleep(Duration::from_millis(20)).await;
    if ok {
        Ok("成功")
    } else {
        Err(String::from("失败了"))
    }
}

#[tokio::main] // 宏：把 main 包装成 runtime.block_on(async { ... })
async fn main() {
    println!("===== 1. 串行 await：一个等完再等下一个 =====");
    let start = Instant::now();
    let a = load("A", 80).await;
    let b = load("B", 80).await;
    println!("  {} / {}", a, b);
    println!("  串行总耗时: {:?}（约 160ms）", start.elapsed());

    println!("\n===== 2. join! 并发等待：两个一起跑 =====");
    let start = Instant::now();
    let (a, b) = tokio::join!(load("A", 80), load("B", 80));
    println!("  {} / {}", a, b);
    println!("  并发总耗时: {:?}（约 80ms）", start.elapsed());

    println!("\n===== 3. spawn：把任务丢到运行时 =====");
    let start = Instant::now();
    let mut handles = Vec::new();
    for i in 1..=3 {
        let name = format!("任务{}", i);
        // spawn 要求 'static，所以数据要 move 进去
        handles.push(tokio::spawn(async move { load(&name, 50).await }));
    }
    for h in handles {
        // h.await 等的是 JoinHandle，任务本身早已并发跑起来了
        println!("  {}", h.await.unwrap());
    }
    println!("  3 个任务总耗时: {:?}（约 50ms）", start.elapsed());

    println!("\n===== 4. select!：谁先完成就用谁 =====");
    tokio::select! {
        v = load("快任务", 20) => println!("  先完成: {}", v),
        _ = sleep(Duration::from_millis(200)) => println!("  超时分支"), // 不会走到
    }

    println!("\n===== 5. timeout：给任务加超时 =====");
    match timeout(Duration::from_millis(50), load("慢任务", 300)).await {
        Ok(v) => println!("  {}", v),
        Err(_) => println!("  超时了，任务被取消"),
    }

    println!("\n===== 6. Result 与 ? 在 async 里同样适用 =====");
    match run_checks().await {
        Ok(msg) => println!("  {}", msg),
        Err(e) => println!("  出错: {}", e),
    }

    println!("\n===== 7. 异步通道 =====");
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(8);
    for i in 1..=3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("消息{}", i)).await.unwrap();
        });
    }
    drop(tx); // 关闭主发送端，否则 recv 永远等不到 None

    while let Some(msg) = rx.recv().await {
        println!("  收到: {}", msg);
    }

    println!("\n===== 8. spawn_blocking：不得不做阻塞操作时 =====");
    // 文件 IO、CPU 密集计算等阻塞操作，必须放到专用线程池，
    // 否则会卡住整个异步运行时。
    let result = tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(30)); // 模拟阻塞调用
        6 * 7
    })
    .await
    .unwrap();
    println!("  阻塞任务结果: {}", result);

    println!("\n===== 9. async 里不要用 thread::sleep =====");
    // 错误示范（已注释）：
    //   std::thread::sleep(Duration::from_secs(1));  // 卡住整个 worker 线程！
    // 正确做法：tokio::time::sleep(..).await
    println!("  用小步 sleep 观察并发：");
    let start = Instant::now();
    let (x, y) = tokio::join!(load("X", 30), load("Y", 30));
    println!("  {} / {}，耗时 {:?}", x, y, start.elapsed());
}

/// async 函数里也能用 ?，错误类型要和返回类型匹配
async fn run_checks() -> Result<String, String> {
    let first = may_fail(true).await?;
    let second = may_fail(true).await?;
    Ok(format!("两步都{}，第一步{}", second, first))
}

// 心智模型：
//   async fn f() -> T       实际返回 impl Future<Output = T>
//   f()                     只是"创建了任务"，什么都不做
//   f().await                真正开始执行，直到完成或遇到下一个 await
//   运行时（tokio）负责在 Future 就绪前挂起它，去执行别的任务
