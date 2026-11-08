//! 11 并发编程（多线程）
//!
//! Rust 的并发安全是编译期保证的：数据竞争会直接编译失败。
//! 核心工具：thread / mpsc 通道 / Arc<Mutex<T>> / 原子类型 / 作用域线程。
//! 运行：./run.sh 11

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("===== 1. spawn + join =====");
    // spawn 创建线程，join 等待它结束并拿到返回值
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(15));
        }
        42 // 闭包最后一行就是线程的返回值
    });

    for i in 1..=2 {
        println!("  主线程: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    let result = handle.join().unwrap(); // unwrap：线程 panic 时这里会拿到 Err
    println!("子线程返回: {}", result);

    println!("\n===== 2. move 闭包把数据交给线程 =====");
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("  子线程收到: {:?}", data);
        data.len()
    });
    // println!("{:?}", data); // ❌ data 的所有权已经移进线程
    println!("子线程算出的长度: {}", handle.join().unwrap());

    println!("\n===== 3. mpsc 通道（多生产者、单消费者） =====");
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // 克隆发送端，可以给多个线程用

    thread::spawn(move || {
        for i in 0..3 {
            tx.send(format!("A-{}", i)).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });
    thread::spawn(move || {
        for i in 0..3 {
            tx2.send(format!("B-{}", i)).unwrap();
        }
    });

    // 两个发送端都随线程结束而 drop，循环自动结束
    for msg in rx {
        println!("  收到: {}", msg);
    }
    println!("  通道已关闭");

    println!("\n===== 4. Arc<Mutex<T>>：多线程共享可变状态 =====");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 只增加引用计数
        handles.push(thread::spawn(move || {
            let mut n = counter.lock().unwrap(); // 拿锁，离开作用域自动解锁
            *n += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("10 个线程各加 1，结果 = {}", *counter.lock().unwrap());

    println!("\n===== 5. 原子类型：比锁更轻 =====");
    let atomic = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let atomic = Arc::clone(&atomic);
        handles.push(thread::spawn(move || {
            // fetch_add 是一条原子指令，不用加锁
            atomic.fetch_add(1, Ordering::SeqCst);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("atomic = {}", atomic.load(Ordering::SeqCst));

    println!("\n===== 6. thread::scope：借用栈上的数据 =====");
    // 普通 spawn 要求 'static，作用域线程允许借用外部变量
    let numbers = vec![1, 2, 3, 4];
    thread::scope(|s| {
        s.spawn(|| {
            println!("  作用域线程读到: {:?}", numbers);
        });
        s.spawn(|| {
            println!("  另一个线程求和: {}", numbers.iter().sum::<i32>());
        });
    }); // scope 结束时自动 join 所有线程
    println!("  作用域结束后 numbers 仍可用: {:?}", numbers);

    println!("\n===== 7. RwLock：读多写少 =====");
    let shared = Arc::new(RwLock::new(vec![1, 2, 3]));

    let readers: Vec<_> = (0..3)
        .map(|_| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || shared.read().unwrap().len())
        })
        .collect();

    let writer = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            shared.write().unwrap().push(4);
        })
    };

    for r in readers {
        println!("  读者看到长度: {}", r.join().unwrap());
    }
    writer.join().unwrap();
    println!("  写后: {:?}", shared.read().unwrap());

    println!("\n===== 8. 用通道传任务（线程池雏形） =====");
    let (job_tx, job_rx) = mpsc::channel::<u32>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let mut workers = Vec::new();
    for id in 0..3 {
        let job_rx = Arc::clone(&job_rx);
        workers.push(thread::spawn(move || {
            loop {
                // 锁只包住 recv()，不要包住整个任务处理
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(n) => println!("  worker {} 处理任务 {}", id, n),
                    Err(_) => break, // 发送端全部关闭，退出
                }
            }
        }));
    }

    for n in 1..=6 {
        job_tx.send(n).unwrap();
    }
    drop(job_tx); // 关闭通道，worker 的 recv 才会返回 Err

    for w in workers {
        w.join().unwrap();
    }
}

// 两个关键 trait（自动实现，不用手写）：
//   Send    —— 值可以跨线程转移所有权
//   Sync    —— &T 可以跨线程共享
// Rc<T> / RefCell<T> 不是 Send/Sync，所以编译器不允许它们跨线程 —— 这就是"无畏并发"。
