# 13 实战：mini-grep

## 这一节要会什么

把前面 12 节串起来，写一个**真正能用的命令行小工具**：

- `std::env::args()` 解析参数
- `Result` + `?` + `Box<dyn Error>` 统一错误处理
- `BufReader::lines()` 流式读文件（大文件也不怕）
- 统计信息走 stderr，匹配结果走 stdout
- `ExitCode` 设置退出码（和 grep 一致：0 找到 / 1 没找到 / 2 出错）

## 运行

```bash
# 搜索内置示例（不需要文件）
./run.sh 13 ERROR

# 搜索文件，忽略大小写
./run.sh 13 ERROR data/sample.log -i

# 不显示行号
./run.sh 13 ERROR data/sample.log -N

# 查看帮助
./run.sh 13 --help
```

> 文件路径是相对 `13-mini-grep/` 解析的，从 workspace 根目录运行也没问题。

## 它做了什么

```
读参数 → 打开文件（或内置示例）→ 逐行读 → 过滤 → 打印 → 返回退出码
```

对应代码里的函数：

| 函数 | 职责 |
|---|---|
| `main()` | 只处理退出码，把逻辑转给 `run()` |
| `run()` | 主流程，返回 `Result<bool, Box<dyn Error>>` |
| `parse_args()` | 解析参数，错误用 `String` 返回 |
| `resolve_path()` | 相对路径兜底，保证从任意目录都能跑 |

## 关键知识点

### 1. 为什么 main 不直接写逻辑

```rust
fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => { eprintln!("错误: {}", e); ExitCode::from(2) }
    }
}
```

`ExitCode` 是标准库的返回类型（Rust 1.61+），比 `std::process::exit()` 更干净——后者会跳过析构函数。`run()` 里就能自由使用 `?`。

### 2. `Box<dyn Error>`：一行搞定错误类型

`run()` 里可能出现的错误：`io::Error`、参数错误（`String`）。它们都能转成 `Box<dyn Error>`，所以 `?` 一步到位。

代价是丢失了具体类型；生产代码里更常见的是 `anyhow::Error`（应用）或 `thiserror`（库）。

### 3. 流式读取

```rust
let reader = BufReader::new(File::open(path)?);
for line in reader.lines() { ... }
```

`lines()` 每次只分配一行的 `String`，1GB 的日志也不会把内存吃满。对比 `fs::read_to_string()`：简单，但整个文件进内存。

### 4. stdout vs stderr

```rust
println!("{:>4} │ {}", idx + 1, line);   // 结果 → stdout
eprintln!("共扫描 {} 行", total);          // 统计 → stderr
```

这样管道过滤才干净：

```bash
./run.sh 13 ERROR > errors.txt      # errors.txt 里只有匹配行
./run.sh 13 ERROR 2>/dev/null       # 只看结果
```

### 5. 退出码约定

| 码 | 含义 | 用途 |
|---|---|---|
| 0 | 成功且找到 | `if mini-grep ...; then` |
| 1 | 没找到 | 脚本判断"有没有" |
| 2 | 参数/IO 错误 | 和真正的 grep 保持一致 |

### 6. `dyn BufRead` 让两种数据源共存

```rust
let reader: Box<dyn BufRead> = match &opts.file {
    Some(path) => Box::new(BufReader::new(File::open(path)?)),
    None => Box::new(BufReader::new(SAMPLE.as_bytes())),
};
```

`File` 和 `&[u8]` 都实现了 `Read`，`BufReader<T>` 都实现了 `BufRead`，所以能放进同一个 `Box`。

## 预期输出

```bash
$ ./run.sh 13 ERROR
   4 │ 2026-11-07 10:00:05 ERROR 连接数据库失败：connection refused
   6 │ 2026-11-07 10:00:08 ERROR 连接数据库失败：timeout
   9 │ 2026-11-07 10:00:15 ERROR 写入失败：disk full

共扫描 10 行，匹配 3 行
```

（最后一行是 stderr，终端里会混在一起显示。）

## 扩展练习（从易到难）

1. **加 `-c`**：只输出匹配行数（像 `grep -c`）。
2. **支持多个文件**：`Vec<String>` 存文件列表，输出时带文件名前缀。
3. **加 `-v`**：反向匹配（输出不含关键词的行）。
4. **加颜色**：用 ANSI 转义 `\x1b[31m...\x1b[0m` 高亮关键词（只在终端输出时启用）。
5. **支持正则**：加 `regex` 依赖，把 `contains` 换成 `Regex::is_match`。
6. **递归目录**：加 `walkdir` 依赖，遍历所有 `.log` 文件。
7. **换成 clap**：用 `clap` 的 derive 重写参数解析，对比手写版本。
8. **做成 loglens 的第一个模块**：把过滤逻辑抽成 `lib.rs` 里的 `pub fn search(...)`，加单元测试。

## 常见坑

- 用 `s[0..2]` 切片中文 → panic（不在字符边界），用 `chars()`。
- 读大文件用 `read_to_string` → 内存爆掉，用 `BufReader`。
- 把错误 `unwrap()` 掉 → 用户看到的是 panic 堆栈，不是友好提示。
- 忘记处理 `BrokenPipe`：`./run.sh 13 ERROR | head -1` 时，后面的写入会报错。入门可以忽略，进阶要处理。
- 路径写死成相对路径 → 换个目录运行就找不到文件（本案例用 `CARGO_MANIFEST_DIR` 兜底）。
