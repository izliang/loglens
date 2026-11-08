# 11 并发编程（多线程）

## 这一节要会什么

- `thread::spawn` + `join`，线程返回值
- `move` 闭包为什么几乎是线程的标配
- `mpsc` 通道：多生产者单消费者
- `Arc<Mutex<T>>`：多线程共享可变状态
- 原子类型 `AtomicUsize`
- `thread::scope`：借用栈上数据
- `RwLock` 和"用通道分发任务"

## 运行

```bash
./run.sh 11
```

> 输出顺序每次都可能不同，线程是并发调度的，这是正常现象。

## 关键概念

### 1. 线程的基本套路

```rust
let handle = thread::spawn(move || {
    // 干活
    42                     // 返回值
});
let value = handle.join().unwrap();
```

`join()` 返回 `Result<T, Box<dyn Any>>`：线程 panic 了，这里拿到 `Err`。

### 2. 为什么总要 `move`

`spawn` 的闭包要求 `'static`（线程可能活得比创建它的栈帧久），所以不能借用局部变量。`move` 把所有权搬进去，问题解决。

需要"借用"时用 `thread::scope`（Rust 1.63+），它保证作用域结束前所有线程都结束，因此允许借用。

### 3. 通信 vs 共享内存

| 方式 | 工具 | 适合 |
|---|---|---|
| 消息传递 | `mpsc::channel` | 任务分发、流水线、解耦 |
| 共享内存 | `Arc<Mutex<T>>` | 计数器、缓存、共享配置 |
| 原子操作 | `AtomicUsize` 等 | 单个数字的计数/标志位 |
| 读写锁 | `Arc<RwLock<T>>` | 读多写少 |

Go 的名言："不要通过共享内存来通信，而要通过通信来共享内存。"Rust 两种都支持。

### 4. Mutex 的三个要点

```rust
let n = mutex.lock().unwrap();   // 拿锁
*n += 1;                          // 改数据
// n 离开作用域 → 自动解锁
```

1. `lock()` 返回 `MutexGuard`，**drop 时自动解锁**，不会忘记。
2. `unwrap()` 处理"锁中毒"（持锁线程 panic 了）。
3. **锁的范围尽量小**：不要拿着锁做 IO、sleep、发网络请求。

### 5. 死锁的典型写法

```rust
let a = m1.lock().unwrap();
let b = m2.lock().unwrap();   // 另一个线程反着拿 → 互相等待
```

预防：所有线程按**相同顺序**加锁；或用 `try_lock` 带超时。

### 6. Send 与 Sync

| trait | 含义 | 例子 |
|---|---|---|
| `Send` | 值可以**转移**到另一个线程 | `String`、`Vec<T>`、`Arc<T>` |
| `Sync` | `&T` 可以**共享**给多个线程 | `i32`、`Mutex<T>`、`AtomicUsize` |
| 都不是 | 不能跨线程 | `Rc<T>`、`RefCell<T>`、裸指针 |

这两个 trait 是**自动推导**的，你几乎不用手写。编译器靠它们把数据竞争挡在编译期——这就是 Rust 说的"无畏并发"。

### 7. 线程池雏形

多个 worker 共享一个 `Arc<Mutex<Receiver>>`，主线程往通道里发任务：

```rust
let job = { let rx = job_rx.lock().unwrap(); rx.recv() };  // 锁只包 recv
```

注意锁的范围：如果整个 `loop` 都持锁，就变成单线程串行了。

## 预期输出（节选）

```
  子线程: 1
  主线程: 1
  ...
子线程返回: 42
子线程算出的长度: 3
  收到: B-0
  收到: A-0
  ...
  通道已关闭
10 个线程各加 1，结果 = 10
atomic = 10
  作用域线程读到: [1, 2, 3, 4]
  作用域结束后 numbers 仍可用: [1, 2, 3, 4]
  读者看到长度: 3        ← 若写线程先拿到锁，这里可能打印 4
  写后: [1, 2, 3, 4]
  worker 0 处理任务 1
  ...
```

## 试着改改

1. 把 `Arc<Mutex<i32>>` 改成 `Arc<AtomicUsize>`，对比代码量和可读性。
2. 用两个通道实现"生产者 → 处理器 → 消费者"三段流水线。
3. 故意制造死锁（两个 Mutex 反序加锁），用 `try_lock` 修复。
4. 用 `thread::scope` 把一个大数组切成 4 段并行求和。
5. 给 worker 池加一个"优雅退出"信号（`AtomicBool` 或单独的 shutdown 通道）。

## 常见坑

- 没写 `move`：报 "closure may outlive the current function"。
- `rx` 在 `for msg in rx` 里被移动，之后还想用 → 不能了。
- 持有 `MutexGuard` 时又去拿同一把锁 → 死锁（不是编译错误！）。
- `Rc<T>` 跨线程 → 编译错误 "`Rc<String>` cannot be sent between threads safely"。
- 用 `thread::sleep` 模拟异步 → 会阻塞整个线程，异步里要用 `tokio::time::sleep`（见 12）。
