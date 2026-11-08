//! 05 错误处理
//!
//! 核心区分：
//!   - 可预期的失败（文件不存在、解析失败）→ 返回 Result，调用方决定怎么办
//!   - 不应该发生的 bug（数组越界、逻辑矛盾）→ panic!，直接崩
//!
//! Rust 没有异常，所有错误都在类型里。
//! 运行：./run.sh 05

use std::fmt;
use std::io;
use std::num::ParseIntError;

/// 自定义错误类型：一个枚举装下所有可能出错的情况
#[derive(Debug)]
enum AppError {
    NotFound(String),
    InvalidNumber(String),
    Io(io::Error),
}

// Display 决定错误打印出来长什么样（用户看的）
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(name) => write!(f, "找不到：{}", name),
            AppError::InvalidNumber(s) => write!(f, "不是合法数字：{}", s),
            AppError::Io(e) => write!(f, "读写失败：{}", e),
        }
    }
}

// 实现 Error 之后就能和标准库错误体系互操作（比如放进 Box<dyn Error>）
impl std::error::Error for AppError {}

// 实现 From 之后，? 运算符就能自动把底层错误"升级"成 AppError
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::InvalidNumber(e.to_string())
    }
}

fn main() {
    println!("===== 1. panic! 与越界 =====");
    // panic! 是"程序有 bug"的信号，会打印调用栈并终止进程
    // panic!("故意崩溃"); // 取消注释试试

    let v = vec![1, 2, 3];
    println!("v[1] = {}（正常访问）", v[1]);
    // let _ = v[99]; // ❌ 运行时 panic: index out of bounds: the len is 3 but the index is 99
    // 安全写法：
    println!("v.get(99) = {:?}（安全访问，返回 Option）", v.get(99));

    println!("\n===== 2. Option 的取值方式 =====");
    let some: Option<i32> = Some(5);
    let none: Option<i32> = None;

    println!("some.unwrap() = {}", some.unwrap()); // 有值时直接取
    println!("none.unwrap_or(0) = {}", none.unwrap_or(0)); // 给默认值
    println!(
        "none.unwrap_or_else(|| 7 + 1) = {}",
        none.unwrap_or_else(|| 7 + 1)
    ); // 惰性默认值
       // println!("{}", none.unwrap()); // ❌ panic: called `Option::unwrap()` on a `None` value
       // println!("{}", none.expect("这里必须有个值")); // ❌ panic，但带上自定义信息

    println!("\n===== 3. Result 是错误的标准容器 =====");
    // Result<T, E> = Ok(T) | Err(E)
    let good = "42".parse::<i32>();
    let bad = "abc".parse::<i32>();
    println!("\"42\" 解析结果  = {:?}", good);
    println!("\"abc\" 解析结果 = {:?}", bad);

    // 必须处理，不能"忘了"——这正是 Rust 比异常机制安全的地方
    match bad {
        Ok(n) => println!("  解析成功: {}", n),
        Err(e) => println!("  解析失败: {}", e),
    }

    // 只关心成功的情况
    if let Ok(n) = "7".parse::<i32>() {
        println!("  if let 成功取值: {}", n);
    }

    println!("\n===== 4. ? 运算符 =====");
    // ? 的作用：Ok 就取出值继续，Err 就直接 return 给调用方
    // 相当于写了 match { Ok(v) => v, Err(e) => return Err(e.into()) }
    println!("sum_of(\"1,2,3\")   = {:?}", sum_of("1,2,3"));
    println!("sum_of(\"1,x,3\")   = {:?}", sum_of("1,x,3"));
    println!("sum_of(\" 10 , 20 \") = {:?}", sum_of(" 10 , 20 "));

    println!("\n===== 5. Option 转 Result =====");
    let nums = [10, 20, 30];
    println!("查找 20 → {:?}", must_find(&nums, 20));
    println!("查找 99 → {:?}", must_find(&nums, 99));

    println!("\n===== 6. 错误逐层向上传播 =====");
    // run 内部用 ? 把 io 错误一路抛出来，main 决定怎么展示
    match run() {
        Ok(()) => println!("  一切正常"),
        Err(e) => eprintln!("  捕获到错误: {}", e),
    }

    // 也可以就地处理，不让它扩散
    match read_text("/tmp/rust-learn-不存在的文件.txt") {
        Ok(text) => println!("  读到了 {} 字节", text.len()),
        Err(e) => println!("  读取失败（已就地处理，程序继续）: {}", e),
    }

    println!("\n===== 7. main 也可以返回 Result =====");
    println!("（看代码注释：fn main() -> Result<(), Box<dyn Error>>）");
}

/// 解析逗号分隔的数字并求和，任何一步失败都用 ? 提前返回
fn sum_of(text: &str) -> Result<i64, AppError> {
    let mut total = 0;
    for part in text.split(',') {
        let n: i64 = part.trim().parse()?; // ParseIntError -> AppError（靠 From）
        total += n;
    }
    Ok(total)
}

/// 在切片里找目标值的下标；找不到时把 Option 转成带说明的 Result
fn must_find(nums: &[i32], target: i32) -> Result<usize, AppError> {
    nums.iter()
        .position(|&x| x == target)
        .ok_or_else(|| AppError::NotFound(target.to_string()))
}

/// 读文件：? 会自动调用 From<io::Error>，把错误升级成 AppError
fn read_text(path: &str) -> Result<String, AppError> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

/// 一个"典型业务函数"：读文件 → 解析 → 返回结果
fn run() -> Result<(), AppError> {
    let content = read_text("/tmp/rust-learn-不存在的文件.txt")?;
    let value: i64 = content.trim().parse()?;
    println!("  解析出 {}", value);
    Ok(())
}
