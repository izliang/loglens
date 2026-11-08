# 12 异步编程（async / await + tokio）

## 这一节要会什么

- `async fn` 返回 `Future`，`await` 才真正执行
- 串行 `await` vs `join!` vs `spawn` 的区别
- `select!`、`timeout`
- 异步通道 `tokio::sync::mpsc`
- 为什么不能在 async 里用 `thread::sleep`
- `spawn_blocking` 的用途

## 运行

```bash
./run.sh 12
```

第一次运行要下载 tokio 依赖（已配 rsproxy 镜像，通常几秒）。之后走缓存，秒开。

## 关键概念

### 1. async 不是"多线程"，是"不阻塞"

```rust
async fn load() -> String { ... }
let fut = load();      // 只是创建 Future，代码一行没跑
let s = fut.await;     // 这里才开始执行
```

一个线程可以同时"等待"成千上万个 Future：等待期间它去跑别的任务。所以异步适合 **IO 密集**（网络、磁盘、数据库），不适合 **CPU 密集**（那种用多线程）。

### 2. 三种并发写法

| 写法 | 语义 | 适合 |
|---|---|---|
| `a.await; b.await;` | 串行，一个等完再下一个 | 有依赖关系 |
| `tokio::join!(a, b)` | 并发，等**全部**完成 | 固定几个任务 |
| `tokio::spawn(a)` | 丢到运行时，返回 `JoinHandle` | 数量不定、后台任务 |

`join!` 返回元组；`spawn` 需要 `'static`（`move`），且任务 panic 会被 `JoinHandle` 捕获。

还有 `try_join!`：任意一个返回 `Err` 就提前结束。

### 3. tokio 的运行时

```rust
#[tokio::main]              // 多线程运行时（默认）
async fn main() { }

#[tokio::main(flavor = "current_thread")]  // 单线程运行时
async fn main() { }
```

`#[tokio::main]` 本质是：

```rust
fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async { ... })
}
```

### 4. 阻塞操作的禁区

```rust
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));  // ❌ 卡住 worker 线程
    std::fs::read_to_string("big.txt");          // ❌ 同上
}
```

正确姿势：

- 等待：`tokio::time::sleep(..).await`
- 文件/CPU：`tokio::task::spawn_blocking(|| { ... }).await`
- 网络/文件：用 tokio 提供的异步版本（`tokio::fs`、`tokio::net`）

### 5. 异步的传染性

调用 `async fn` 的函数自己也必须是 `async`，一路传染到 `main`。这就是为什么异步项目通常从入口开始就是 tokio。

### 6. 常用 tokio 组件

| 需求 | 工具 | 需要 feature |
|---|---|---|
| 延时 | `tokio::time::sleep` | `time` |
| 超时 | `tokio::time::timeout` | `time` |
| 并发等待 | `tokio::join!` / `try_join!` | `macros` |
| 竞速 | `tokio::select!` | `macros` |
| 任务 | `tokio::spawn` | `rt` |
| 通道 | `tokio::sync::mpsc` | `sync` |
| 锁 | `tokio::sync::Mutex` | `sync` |
| 文件 | `tokio::fs` | `fs` |

## 依赖说明

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time", "sync"] }
```

不要无脑 `features = ["full"]`：编译慢、体积大。按需开特性。

## 预期输出（节选）

```
===== 1. 串行 await：一个等完再等下一个 =====
  A 完成（80ms） / B 完成（80ms）
  串行总耗时: 160.3ms（约 160ms）

===== 2. join! 并发等待：两个一起跑 =====
  A 完成（80ms） / B 完成（80ms）
  并发总耗时: 80.2ms（约 80ms）

===== 3. spawn：把任务丢到运行时 =====
  任务1 完成（50ms）
  任务2 完成（50ms）
  任务3 完成（50ms）
  3 个任务总耗时: 50.4ms（约 50ms）

===== 4. select!：谁先完成就用谁 =====
  先完成: 快任务 完成（20ms）

===== 5. timeout：给任务加超时 =====
  超时了，任务被取消
```

耗时数字会有浮动，重点看数量级。

## 试着改改

1. 把第 1 节的串行改成 `join!`，对比耗时。
2. 用 `try_join!` 改写 `run_checks`，让第一个 Err 立即返回。
3. 用 `tokio::select!` 实现"两个请求谁先返回用谁"。
4. 用 `tokio::sync::Mutex` 保护一个计数器，10 个任务各加 1。
5. 写一个 `async fn` 读取文件（`tokio::fs::read_to_string`），加 `timeout` 包裹。
6. 把 `spawn_blocking` 里的 sleep 换成真正的 CPU 计算（比如算斐波那契），观察不阻塞其他任务。

## 常见坑

- 创建了 Future 但没 `.await` → 什么都不发生（编译器会给 `unused_must_use` 警告）。
- 在 async 里用 `std::thread::sleep` → 卡住整个 worker。
- `tokio::spawn` 的闭包没 `move` → 生命周期报错。
- 忘了 `drop(tx)` → `rx.recv().await` 永远等不到 `None`。
- 混用两个版本的 tokio → 类型不兼容，`cargo tree` 检查。
- `#[tokio::main]` 加在普通 `fn main` 上 → 宏展开冲突。
