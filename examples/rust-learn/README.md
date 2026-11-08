# Rust 学习示例库

按知识点拆成独立的小项目，每个目录都能单独运行、单独修改。所有说明都是中文，代码里有大量注释。

## 快速开始

```bash
cd examples/rust-learn

./run.sh              # 列出所有示例
./run.sh 00           # 运行第 0 个
./run.sh 03           # 运行所有权示例
./run.sh 13 error data/sample.log -i   # 带参数运行小项目
```

也可以完全不用脚本：

```bash
cargo run -p hello        # 在 workspace 根目录（examples/rust-learn）执行
cd 03-ownership && cargo run   # 或者进入某个目录单独运行
```

> 整个目录是一个 Cargo workspace，只编译一次共享的依赖，第一次会慢几秒，之后秒开。
>
> 环境还没配好（rustup / 国内镜像 / 编辑器插件）？先看 [Rust 环境搭建说明](../../docs/rust-env-setup.md)。

## 目录总览

| 编号 | 目录 | 知识点 | 建议时长 |
|---|---|---|---|
| 00 | `00-hello` | 最小程序、`cargo` 基本命令、打印与格式化 | 20 分钟 |
| 01 | `01-variables` | 变量、可变性、遮蔽、基本类型、元组与数组、字符串两种形态 | 1 小时 |
| 02 | `02-control-flow` | 函数、语句与表达式、`if`/`match`/`loop`、返回值 | 1 小时 |
| 03 | `03-ownership` | 所有权、移动、克隆、借用、切片 —— **Rust 最重要的一课** | 2 小时 |
| 04 | `04-struct-enum` | 结构体、方法、枚举、`Option`、模式匹配 | 2 小时 |
| 05 | `05-error-handling` | `panic!`、`Result`、`?`、自定义错误类型 | 2 小时 |
| 06 | `06-collections` | `Vec`、`String`、`HashMap`、`HashSet`、`BTreeMap` | 1.5 小时 |
| 07 | `07-iterators-closures` | 闭包三种 `Fn`、迭代器惰性、`map`/`filter`/`fold` | 2 小时 |
| 08 | `08-generics-traits` | 泛型、`trait`、默认方法、`impl Trait`、`dyn Trait` | 2 小时 |
| 09 | `09-lifetimes` | 生命周期标注、省略规则、结构体持引用、`'static` | 2 小时 |
| 10 | `10-smart-pointers` | `Box`、`Rc`、`RefCell`、`Weak`、`Deref`/`Drop` | 2 小时 |
| 11 | `11-concurrency` | 线程、`channel`、`Arc<Mutex<T>>`、原子类型 | 2 小时 |
| 12 | `12-async` | `async/await`、`tokio`、并发等待 | 2 小时（可跳过，第 6 周再学） |
| 13 | `13-mini-grep` | 综合实战：命令行参数 + 流式读文件 + 过滤 | 3 小时 |

**顺序建议：00 → 03 必须按顺序打牢，06/07 可以并行，12 放最后。**

## 每个目录的结构

```
03-ownership/
├── Cargo.toml     # 项目清单（一般不用改）
├── README.md      # 这一节在讲什么、怎么跑、看完该会什么
└── src/main.rs    # 带注释的示例代码，从上往下顺序执行
```

## 怎么用这些示例

1. **先跑一遍**：看输出，对照 README 里的"预期输出"。
2. **再改一遍**：把注释里的"试着改改"亲手做一次，尤其是故意触发编译错误。
3. **最后默写**：关掉代码，用空白文件重写核心片段。

Rust 的编译器报错信息是很好的老师。看到红色错误不要慌，先读最后一段 `help:`，它经常直接写出改法。

## 常见问题

**Q: 编译特别慢？**
第一次编译要下载依赖（已配好国内镜像，通常几秒）。之后只有改动才重编。

**Q: 想临时验证一段代码，不想建项目？**
用 `rustc` 直接编译单文件：

```bash
rustc /tmp/a.rs -o /tmp/a && /tmp/a
```

**Q: 看到 `warning: unused variable` 要管吗？**
学习期可以忽略，正式代码用 `cargo clippy` 清理。

**Q: 为什么 `cargo clippy` 会报一些风格警告？**
示例里有些写法是**故意**保留的（比如 `y = y + 1`、`push_str("!")`、`fold`），为了对照讲解。真实项目里按 clippy 的建议改即可。clippy 的 correctness 级错误会阻断编译，本仓库已全部清零。

**Q: 在哪查标准库 API？**
`cargo doc --open`，或 https://doc.rust-lang.org/std/ 。学 `Iterator` 的时候把方法列表从头翻一遍，收益很大。
