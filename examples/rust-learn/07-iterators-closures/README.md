# 07 迭代器与闭包

## 这一节要会什么

- 闭包怎么写、怎么捕获环境变量、`move` 意味着什么
- `Fn` / `FnMut` / `FnOnce` 三者的区别
- 迭代器**惰性**：适配器不执行，消费者才执行
- `map` / `filter` / `fold` / `collect` / `zip` / `take` / `flat_map`
- 自己实现 `Iterator`

## 运行

```bash
./run.sh 07
```

## 关键概念

### 1. 闭包 vs 函数

| | 普通 `fn` | 闭包 `\|x\| ...` |
|---|---|---|
| 捕获环境变量 | ❌ | ✅ |
| 类型 | 函数指针 | 每个闭包有唯一匿名类型 |
| 存进结构体 | 用 `fn` 指针 | 用泛型或 `Box<dyn Fn>` |
| 语法 | `fn f(x: i32) -> i32` | `\|x\| x + 1` |

闭包的类型是编译器生成的匿名类型，所以**只能通过泛型或 trait 对象存起来**：

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
```

### 2. 三种 Fn

| Trait | 捕获方式 | 能调用几次 | 典型场景 |
|---|---|---|---|
| `Fn` | 只读借用 `&T` | 多次 | 大多数回调 |
| `FnMut` | 可变借用 `&mut T` | 多次 | 累加器、计数器 |
| `FnOnce` | 拿走所有权 | **一次** | 把值移出闭包 |

规律：**`Fn` 可以当 `FnMut` 用，`FnMut` 可以当 `FnOnce` 用**（能力递减）。

### 3. 迭代器是惰性的

```rust
let it = (1..=5).map(|x| println!("算 {}", x));
// 此时什么都不打印！
let v: Vec<_> = it.collect();   // 这一刻才开始执行
```

- **适配器**（返回新迭代器）：`map`、`filter`、`take`、`skip`、`chain`、`zip`、`enumerate`、`flat_map`、`rev`
- **消费者**（返回一个值）：`collect`、`sum`、`product`、`count`、`fold`、`max`、`min`、`any`、`all`、`find`、`for_each`

没有消费者，迭代器什么都不会做。

### 4. 几个高频组合

```rust
// 过滤 + 转换 + 收集
let v: Vec<i32> = nums.iter().filter(|&&x| x > 0).map(|&x| x * 2).collect();

// 求和
let total: i32 = nums.iter().sum();

// 计数
let n = logs.iter().filter(|l| l.starts_with("ERROR")).count();

// 找第一个
let first = nums.iter().find(|&&x| x > 10);

// 自定义累积
let s = nums.iter().fold(0, |acc, &x| acc + x);
```

注意 `iter()` 产生的是 `&T`，所以闭包里经常要 `|&&x|` 或 `|&x|`——这是新手最容易卡住的地方。也可以先 `.copied()` 或 `.cloned()` 把引用变成值。

### 5. 自定义迭代器

```rust
impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> { ... }
}
```

实现 `next()` 之后，`map` / `filter` / `take` / `collect` 等**全部自动可用**，这就是 trait 的威力。

## 预期输出（节选）

```
add(2, 3) = 5
  inc 三次: 1 2 3
  循环外读 count = 3
  lazy 已经创建，但上面没有打印任何东西
  计算 1 的平方
  ...
  偶数的平方: [4, 16, 36, 64, 100]
  1..=100 求和: 5050
  zip: [(1, "a"), (2, "b"), (3, "c")]
  flat_map: ["hello", "world", "rust", "lang"]
  相邻差值: [1, 2, 3, 4]
  错误消息: ["连接超时", "写入失败"]
```

## 试着改改

1. 把 `(1..=10).filter(..).map(..)` 改成先转 `Vec` 再 `for` 循环，对比可读性。
2. 用 `fold` 实现 `filter` + `map` 的效果，体会"万能的 fold"。
3. 给 `Countdown` 加一个 `size_hint`，看看有没有区别（提示：可选）。
4. 写 `fn word_count(text: &str) -> HashMap<&str, usize>`，全程用迭代器表达。
5. 把 `apply_mut` 的 `total` 累加改成统计调用次数，观察 `total` 在闭包外能否读取。

## 常见坑

- 以为 `map` 会立刻执行 → 忘了 `collect()`，什么都没发生。
- `iter()` 给的是 `&T`，闭包里 `x * 2` 报类型错 → 用 `|&x|` 或 `.copied()`。
- 闭包借用了变量，之后又想改这个变量 → E0502，缩小作用域或改用 `move`。
- `collect()` 的目标类型不明确 → 用 `collect::<Vec<_>>()` 或给变量标类型。
