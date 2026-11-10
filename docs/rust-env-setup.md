# Rust 开发环境搭建 + 国内依赖方案 + 学习路线

> 针对本机实测生成：macOS 15.7.4 / Intel i9-9880H / 16 线程 / 32 GB / 剩余 357 GB。
> 所有网络数字都来自下面的实测命令，换网络后请重测。
>
> ✅ **本机当前状态（2026-11-08 21:00 已执行）**：`~/.cargo/config.toml` 已写入 rsproxy 镜像，
> 实测 `cargo fetch` 3.6 s，下载日志显示 `(registry 'mirror-sparse')`，镜像已生效。
> 剩余可选项：`brew install cmake pkg-config`、按需装 cargo 工具、VS Code 装 rust-analyzer。

---

## 一、系统体检结论

| 项目 | 现状 | 结论 |
|---|---|---|
| 系统 | macOS 15.7.4 (Darwin 24.6.0)，x86_64 | ✅ Intel 机器，target 固定 `x86_64-apple-darwin` |
| CPU / 内存 | Intel i9-9880H，8 核 16 线程，32 GB | ✅ 编译 Rust 很够用，`cargo build -j16` 无压力 |
| 磁盘 | 932 GB，剩 357 GB | ✅ target 目录动辄几个 GB，无风险 |
| 命令行工具 | Xcode Command Line Tools 15.5，SDK 在 `/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk` | ✅ 链接器正常（hello world 3.1 秒编译通过） |
| Rust 工具链 | **已装 rustc/cargo 1.99.1**（2026-10 更新），stable-x86_64-apple-darwin | ✅ 不用重装，只差镜像配置 |
| 组件 | clippy / rustfmt / rust-src / rust-analyzer 全有 | ✅ 开箱即用 |
| 包缓存 | `~/.cargo` 390 MB，423 个 crate（以前编过 Tauri 项目） | ✅ 说明已能编译大中型项目 |
| 编辑器 | VS Code、Cursor、IntelliJ IDEA 都在，`cursor` 命令可用 | ✅ 装个 rust-analyzer 插件即可 |
| 其他 | Homebrew 7.0.7、Node 24、Docker 29、git 2.39.5 | ✅ 生态齐全 |

**一句话结论：机器和工具链都没问题，唯一的短板是网络——`~/.cargo/config.toml` 没配，所有 crate 默认走 `static.crates.io`，实测被限速到几十 KB/s。**

---

## 二、网络实测（这是要不要配镜像的唯一依据）

| 目标 | 实测 | 说明 |
|---|---|---|
| `index.crates.io`（稀疏索引） | 0.37 s / 200 | 索引本身不慢 |
| `static.crates.io` 下 serde（79 KB） | 0.75 s，约 **105 KB/s** | 慢 |
| `static.crates.io` 下 tokio（800 KB） | **23.2 s，约 36 KB/s** | 非常慢 |
| `rsproxy.cn` 下 serde | 0.40 s，约 200 KB/s | 快 |
| `rsproxy.cn` 下 tokio | **0.48 s，约 1.7 MB/s** | 比官方快 **~48 倍** |
| rsproxy 冷启动拉 24 个 crate（45 MB） | **2.5 s** | 引子：这才是正常体验 |
| `github.com`（HTTPS 握手） | 8 s | 慢，但 `git@github.com` SSH 认证正常 |
| `mirrors.ustc.edu.cn/rust-static` | 0.47 s / 200 | rustup 镜像可用（你的 `.zshrc` 已在用） |
| `static.rust-lang.org` | 10 s+ | 官方 rustup 源很慢 |

各镜像的稀疏索引 `config.json` 全部返回 200：rsproxy / USTC / TUNA / 阿里云 /  SJTU 都可用，**按吞吐选 rsproxy**（字节跳动维护），备用 USTC 或阿里云。

> 对照组：同一份 `tokio` 依赖，默认源跑了 7 分钟以上还没结束（最后被我终止），镜像 2.5 秒。这就是为什么国内必须配镜像。

---

## 三、当前缺什么

1. ~~**`~/.cargo/config.toml` 不存在** → crate 下载走官方 CDN。~~ **已修复，本次已写入（见 Step 1）。**
2. `cmake` 没装 → `aws-lc-rs`、`libz-sys`、部分 `-sys` 包编译会失败（`pkg-config` 同理）。
3. rustup 镜像只配在 `.zshrc` 里，**GUI 启动的进程（Cursor、IDEA）读不到**，用 `~/.zshenv` 更稳。
4. 没有 `cargo-edit` / `cargo-watch` / `cargo-nextest` 这类提效工具（可选）。

---

## 四、搭建步骤（约 15 分钟）

### Step 0 备份

```bash
cp ~/.cargo/config.toml ~/.cargo/config.toml.bak 2>/dev/null || true
```

### Step 1 配 cargo 镜像（核心，必做）

新建 `~/.cargo/config.toml`：

```toml
# ~/.cargo/config.toml
[source.crates-io]
replace-with = "rsproxy-sparse"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

# 让 `cargo publish --registry rsproxy` / 私有源也能用
[registries.rsproxy]
index = "sparse+https://rsproxy.cn/index/"

[net]
# git 依赖走系统 git，能复用 SSH key，也方便走代理
git-fetch-with-cli = true

[alias]
b  = "build"
r  = "run"
t  = "test"
c  = "check"
cl = "clippy --all-targets -- -D warnings"
f  = "fmt"
```

> ⚠️ 我实测踩到的坑：同时写 `[source.rsproxy]` 和 `[source.rsproxy-sparse]` 指向同一个 URL 会直接报
> `source 'rsproxy' defines source registry 'rsproxy-sparse', but that source is already defined by 'rsproxy-sparse'`。
> **同一个 registry 只能定义一次**，上面这份是验证过能跑的。

也可以用仓库里的脚本一键完成（幂等、自动备份）：

```bash
bash scripts/setup-rust-cn.sh            # 只配镜像
bash scripts/setup-rust-cn.sh --with-tools   # 顺带装常用 cargo 工具（要编译几分钟）
```

### Step 2 rustup 镜像

`.zshrc` 里已有：

```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

能用就先别动。建议把这四条挪到 `~/.zshenv`（GUI 程序也生效），或者换成 rsproxy 统一风格：

```bash
export RUSTUP_DIST_SERVER=https://rsproxy.cn
export RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup
```

验证：`rustup update` 应该在十几秒内完成，而不是卡住。

### Step 3 补系统依赖

```bash
brew install cmake pkg-config
```

后面遇到 `-sys` 包（openssl-sys、libz-sys、aws-lc-rs）基本都靠这两个。用 OpenSSL 时再 `brew install openssl@3` 并 `export OPENSSL_DIR=$(brew --prefix openssl@3)`。

### Step 4 常用工具（可选，按需装）

```bash
cargo install cargo-edit cargo-watch cargo-nextest   # 每个编译 1~3 分钟
rustup component add rust-analyzer                   # 已装可跳过
```

| 工具 | 用途 | 是否必装 |
|---|---|---|
| `cargo-edit` | `cargo add serde --features derive` 直接改 Cargo.toml | 强烈推荐 |
| `cargo-watch` | `cargo watch -x check -x test` 保存即检查 | 推荐 |
| `cargo-nextest` | 更快的测试运行器 | 中后期再说 |
| `cargo-expand` | 展开宏，看 derive 生成了什么 | 学 trait/宏时很有用 |
| `sccache` | 编译缓存，多项目共享 target | 单项目没必要 |

### Step 5 编辑器

- **VS Code / Cursor**：装 `rust-analyzer`（官方）+ `CodeLLDB`（调试）+ `Even Better TOML`。
  推荐设置：`"rust-analyzer.check.command": "clippy"`、`editor.formatOnSave: true`、打开 inlay hints（能直接看到类型和生命周期，学习期非常有用）。
- **IntelliJ IDEA**：装 Rust 插件（官方，需 Ultimate 或社区版新版），调试体验比 VS Code 好。
- 不要用 Vim/Neovim 起步，学习期会被配置分心。

### Step 6 验收清单

```bash
rustc -V && cargo -V && rustup show
cargo new /tmp/hello && cd /tmp/hello
cargo add serde --features derive && cargo build   # 能秒级下载 = 镜像生效
cargo clippy && cargo fmt --check
```

全部通过就可以开始了。

---

## 五、国内依赖处理的完整策略

### 5.1 原理

Cargo 的 **source replacement**：`[source.crates-io] replace-with = "xxx"` 让本机把所有 `crates.io` 的请求改投到镜像。只影响本机，`Cargo.lock` 里记录的仍是 `registry+https://github.com/rust-lang/crates.io-index`，所以**提交 lock 文件不会污染同事的环境**。

### 5.2 镜像怎么选 / 怎么切

| 镜像 | 稀疏索引地址 | 备注 |
|---|---|---|
| rsproxy（字节） | `sparse+https://rsproxy.cn/index/` | **首选**，实测 1.7 MB/s，同步快 |
| USTC | `sparse+https://mirrors.ustc.edu.cn/crates.io-index/` | 老牌稳定，备用 |
| 阿里云 | `sparse+https://mirrors.aliyun.com/crates.io-index/` | 备用 |
| TUNA | `sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/` | 备用 |
| SJTU | `sparse+https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/` | 备用 |

切换只需改 `config.toml` 里那一行 `registry = "sparse+..."`。想临时绕过镜像：

```bash
cargo build --config 'source.crates-io.replace-with="crates-io"'
```

（不建议长期这么用，官方源在这台机器上慢到不可用。）

### 5.3 git 依赖

`git = "https://github.com/..."` 的依赖**不走 crates 镜像**。你的 GitHub SSH 已验证可用，所以：

```toml
[net]
git-fetch-with-cli = true
```

再配合 `~/.gitconfig`：

```ini
[url "git@github.com:"]
    insteadOf = https://github.com/
```

这样所有 https 的 git 依赖自动转成 SSH，绕开 8 秒的 HTTPS 握手。**代价**：没配 SSH key 的环境（CI）会失败，CI 里单独用 `https_proxy` 或 ghproxy。

### 5.4 私有 registry / 公司内网

```toml
[registries.corp]
index = "sparse+https://nexus.corp/repository/crates/index/"
# token 用 `cargo login --registry corp` 存在 ~/.cargo/credentials.toml
```

私有包用 `dep = { version = "1", registry = "corp" }`。注意：`replace-with` 只对 `crates-io` 生效，私有 registry 不受影响。

### 5.5 离线 / vendor（出差别踩）

```bash
cargo vendor            # 把依赖拷进 vendor/，离线编译
cargo build --offline
```

`~/.cargo/registry/cache` 已经有 390 MB 缓存，日常 `--offline` 大多能跑。

### 5.6 常见坑（按遇到概率排序）

1. **镜像同步延迟**：刚发布的 crate 版本镜像上可能 404。等 3–10 分钟，或临时用官方源 `cargo add foo@=1.2.3`。
2. **索引损坏**：删掉 `~/.cargo/registry/index/` 下对应镜像目录（如 `rsproxy.cn-*`）再重试；`cargo clean` 只清 target，不清索引。
3. **sparse 协议要求 cargo ≥ 1.68**：你是 1.99，无影响；老 CI 镜像要注意。
4. **同一 registry 定义两次**会直接报错（见 Step 1 的坑）。
5. **代理与镜像打架**：设了 `https_proxy` 又配镜像，流量会先绕代理再到镜像，反而更慢。先 `env | grep -i proxy` 排查。
6. **`cargo install` 很慢**：它要完整编译。优先用 `cargo binstall`（直接下 GitHub Release 预编译包），或干脆不装。
7. **GUI 里环境变量丢失**：Cursor/IDEA 从 Dock 启动不会读 `.zshrc`，rustup 镜像变量请放 `~/.zshenv`。
8. **Intel Mac 的 target**：只有 `x86_64-apple-darwin`。想跑 Apple Silicon 需要 `rustup target add aarch64-apple-darwin` 交叉编译，但本机不能原生执行，别在这上面花时间。

---

## 六、学习路线（8 周，配合本仓库 loglens 实战）

### 阶段划分

| 周 | 主题 | 关键点 | 产出 |
|---|---|---|---|
| 1 | 语法 + 所有权 | 移动/借用/可变借用规则、`String` vs `&str` | rustlings 前 40 题 |
| 2 | 结构体/枚举/模式匹配/错误处理 | `Result`/`?`/`Option`、`match` 穷尽性 | rustlings 剩余 |
| 3 | 泛型/trait/生命周期 | trait 作为接口、`impl Trait`、生命周期标注 | 手写一个泛型缓存 |
| 4 | 集合/迭代器/闭包/智能指针 | 迭代器惰性求值、`Box`/`Rc`/`RefCell` | 用迭代器重写阶段 1 代码 |
| 5 | 并发 | `thread`/`channel`/`Arc<Mutex<T>>`、`rayon` 数据并行 | 并行统计文件行数 |
| 6 | 模块/包/工作区 + 异步入门 | `cargo` 进阶、`tokio` 基础、`async/await` | 一个并发抓取小工具 |
| 7–8 | loglens 实战 | CLI、流式 IO、正则、序列化、TUI | 可用的 v0.4 |

**每天 1–2 小时足够，关键是每天写代码，不是每天看书。**

### loglens 迭代计划（仓库是空的，正好从零开始）

| 版本 | 功能 | 用到的知识 | 依赖 |
|---|---|---|---|
| v0.1 | `loglens <file> --grep <kw>`：过滤 + 行号 + 高亮 | 文件 IO、`BufRead`、`clap` derive、错误处理 | `clap`、`anyhow` |
| v0.2 | 支持正则、多模式、`--before/--after` 上下文 | `regex`、`VecDeque` 环形缓冲、生命周期 | `regex`、`thiserror` |
| v0.3 | 并行扫描多文件 + 按级别/时间聚合 + `--json` 输出 | `rayon`、`serde`、迭代器管道 | `rayon`、`serde_json`、`chrono` |
| v0.4 | TUI 实时看日志（或 `axum` 起个 Web） | 异步、`ratatui`/`axum`、`tracing` | `ratatui` / `axum`、`tokio` |
| v1.0 | 压测、benchmark、发布 release | `criterion`、CI、交叉编译 | — |

建议顺序：**先 v0.1 跑通再谈其他**，第一个能跑的 CLI 比读三本书更有用。

### 学习资源（按使用频率排序）

1. **《Rust 程序设计语言》中文版** — https://kaisery.github.io/trpl-zh-cn/ （官方书，第 4、10、13 章反复读）
2. **Rust 圣经** — https://course.rs/ （中文原创，习题和实战章节更贴近国内读者）
3. **Rustlings** — https://github.com/rust-lang/rustlings （每天 5 题，编译错误驱动学习）
4. **标准库文档** — https://doc.rust-lang.org/std/ （查 `Iterator`、`Option`、`Result` 的方法链）
5. **Rust by Example** — https://doc.rust-lang.org/rust-by-example/
6. **cheats.rs** — https://cheats.rs/ （语法速查，贴墙上）
7. **Tokio 教程** — https://tokio.rs/tokio/tutorial （第 6 周再开始）

### 三个忠告

- **和编译器做朋友**：Rust 的报错信息是最好的老师，`cargo check` 一遍遍改，不要绕过 `unsafe`。
- **先同步后异步**：前 6 周别碰 `async`，绝大多数"Rust 太难"的挫败来自过早接触生命周期 + 异步叠加。
- **用小项目收口**：每学一个概念就在 loglens 里用一次，不然两周后全忘。

---

## 七、速查表

```bash
cargo new demo --bin          # 新建可执行项目
cargo add serde --features derive
cargo check                   # 秒级语法/类型检查（主力）
cargo clippy -- -D warnings   # 静态检查
cargo fmt                     # 格式化
cargo run -- --help
cargo test
cargo build --release         # 优化构建
cargo tree -d                 # 查重复依赖
cargo tree -i serde           # 谁依赖了 serde
cargo update -p serde         # 单独升级
cargo doc --open              # 本地文档
```

| 概念 | 一句话 |
|---|---|
| 所有权 | 一个值同一时刻只有一个所有者，离开作用域即释放 |
| 借用 | `&T` 可多个，`&mut T` 只能一个，且不能与 `&T` 共存 |
| 生命周期 | 标注"这个引用活得至少和那个一样久"，不改变实际寿命 |
| trait | 接口/能力，可静态分发（泛型）或动态分发（`dyn`） |
| `?` | 错误提前返回 + 自动 `From` 转换 |
| `Clone` vs `Copy` | `Copy` 是隐式按位复制，`Clone` 要显式调用 |
