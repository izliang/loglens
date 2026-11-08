# 05 错误处理

## 这一节要会什么

- `panic!` 和 `Result` 的分工
- `unwrap` / `expect` / `unwrap_or` / `unwrap_or_else` 的区别
- `?` 运算符到底做了什么
- 自定义错误类型：`enum` + `Display` + `Error` + `From`
- 把 `Option` 转成 `Result`

## 运行

```bash
./run.sh 05
```

## 关键概念

### 1. 两种错误，两种处理

| | 可恢复错误 | 不可恢复错误 |
|---|---|---|
| 典型场景 | 文件不存在、JSON 格式错、网络超时 | 数组越界、断言失败、逻辑 bug |
| 手段 | `Result<T, E>` | `panic!` / `unwrap` |
| 结果 | 调用方决定怎么处理 | 线程崩溃（默认展开栈） |

**判断标准**：这个错误是"用户/环境造成的"还是"程序写错了"。前者用 `Result`，后者 panic 也无可厚非。

### 2. Result 就是枚举

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

所以它能用 `match`、`if let`、`?`。没有异常、没有 `try/catch`、没有隐式抛出。

### 3. 取值方式对照

| 写法 | 遇到 `None`/`Err` 时 |
|---|---|
| `.unwrap()` | panic |
| `.expect("说明")` | panic，但打印你的说明 |
| `.unwrap_or(默认值)` | 返回默认值 |
| `.unwrap_or_else(\|\| 计算)` | 调用闭包算默认值 |
| `?` | 提前返回错误给调用方 |
| `.ok()?` | `Result` → `Option` 再传播 |

学习期用 `unwrap` 没关系，写正式代码时逐个换成 `?` 或 `unwrap_or`。

### 4. `?` 的等价展开

```rust
let n: i64 = part.trim().parse()?;
```

大致等于：

```rust
let n: i64 = match part.trim().parse() {
    Ok(v) => v,
    Err(e) => return Err(AppError::from(e)),   // 注意这里的 from
};
```

所以 `?` 能用的前提是：**当前函数返回 `Result`，并且 `E` 实现了 `From<底层错误>`**。这就是为什么我们要手写两个 `impl From<...>`。

### 5. 自定义错误的四件套

```rust
#[derive(Debug)]                 // 1. Debug（Error 要求）
enum AppError { ... }
impl fmt::Display for AppError   // 2. 给人看的描述
impl std::error::Error for AppError  // 3. 接入标准错误体系
impl From<io::Error> for AppError    // 4. 让 ? 能自动转换
```

手写这四件套很啰嗦，真实项目通常用：

- **`thiserror`**：用宏自动生成上面这些（库/业务代码首选）
- **`anyhow`**：不想定义错误类型时，用 `anyhow::Result<T>` 一把梭（应用/脚本首选）

```toml
[dependencies]
thiserror = "2"
anyhow = "1"
```

```rust
#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("找不到：{0}")]
    NotFound(String),
    #[error("读写失败")]
    Io(#[from] std::io::Error),
}
```

### 6. Option → Result

```rust
nums.iter().position(|&x| x == target)
    .ok_or_else(|| AppError::NotFound(target.to_string()))
```

`ok_or` / `ok_or_else` 把"没有"变成"有错误原因的错误"，比 `unwrap` 友好得多。

## 预期输出（节选）

```
v[1] = 2（正常访问）
v.get(99) = None（安全访问，返回 Option）
none.unwrap_or(0) = 0
"42" 解析结果  = Ok(42)
"abc" 解析结果 = Err(ParseIntError { kind: InvalidDigit })
sum_of("1,2,3")   = Ok(6)
sum_of("1,x,3")   = Err(InvalidNumber("invalid digit found in string"))
查找 20 → Ok(1)
查找 99 → Err(NotFound("99"))
  捕获到错误: 读写失败：No such file or directory (os error 2)
```

## 试着改改

1. 把 `run()` 里的路径改成 `/etc/hosts`，观察成功路径（注意后面 `parse` 会失败，正好看错误传播）。
2. 给 `AppError` 加一个 `TooLarge(u64)` 变体，并在 `sum_of` 里当总和超过 100 时返回它。
3. 把 `must_find` 的 `ok_or_else` 改成 `ok_or`，体会两者区别（一个惰性求值，一个立刻构造）。
4. 把 `main` 改成 `fn main() -> Result<(), AppError>`，把 `run()` 的 `?` 直接放到 main 里，观察程序退出码。

## 常见坑

- 在返回 `()` 的函数里用 `?` → 编译错误，因为 `()` 不是 `Result`。
- `unwrap()` 一个 `Err` 会 panic，生产环境里等于埋雷。
- 忘了 `impl From` 就写 `?` → 报 "the trait `From<...>` is not implemented"。
- `Box<dyn Error>` 能吞下所有错误，适合小工具和 `main`，但会丢失具体类型，业务层别用。
