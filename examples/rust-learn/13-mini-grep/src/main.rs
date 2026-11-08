//! 13 实战：mini-grep（命令行 + 流式读文件 + 过滤）
//!
//! 把前面学的都用上：参数解析、Result/错误处理、BufRead 流式读取、
//! 字符串匹配、退出码、stdout/stderr 分流。
//! 运行：./run.sh 13 [关键词] [文件] [-i] [-N]
//!   ./run.sh 13 ERROR            # 搜索内置示例
//!   ./run.sh 13 ERROR data/sample.log -i
//!   ./run.sh 13 --help

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "\
mini-grep —— 一个简化版 grep

用法:
  mini-grep <关键词> [文件] [选项]

选项:
  -i        忽略大小写
  -N        不显示行号（默认显示）
  -h, --help 显示帮助

不指定文件时，搜索内置示例日志。

退出码:
  0  找到匹配
  1  没有匹配
  2  参数或 IO 错误

示例:
  ./run.sh 13 ERROR
  ./run.sh 13 ERROR data/sample.log -i
";

/// 没有文件参数时用来演示的内置日志
const SAMPLE: &str = "\
2026-11-07 10:00:01 INFO  服务启动，监听 127.0.0.1:8080
2026-11-07 10:00:02 INFO  加载配置 config/default.toml
2026-11-07 10:00:03 WARN  磁盘剩余空间 10%
2026-11-07 10:00:05 ERROR 连接数据库失败：connection refused
2026-11-07 10:00:06 INFO  重试中（第 1 次）
2026-11-07 10:00:08 ERROR 连接数据库失败：timeout
2026-11-07 10:00:09 INFO  切换到备用数据库
2026-11-07 10:00:12 WARN  备用数据库延迟较高
2026-11-07 10:00:15 ERROR 写入失败：disk full
2026-11-07 10:00:16 INFO  已暂停写入，等待清理
";

#[derive(Debug)]
struct Options {
    pattern: String,
    file: Option<String>,
    ignore_case: bool,
    line_numbers: bool,
}

fn main() -> ExitCode {
    // 主函数只负责"退出码"，具体逻辑丢给 run()，这样可以用 ?
    match run() {
        Ok(true) => ExitCode::SUCCESS,  // 0：有匹配
        Ok(false) => ExitCode::from(1), // 1：无匹配（和 grep 一致）
        Err(e) => {
            eprintln!("错误: {}", e);
            ExitCode::from(2) // 2：出错
        }
    }
}

fn run() -> Result<bool, Box<dyn std::error::Error>> {
    let opts = parse_args()?;

    // BufReader 按行读，内存占用和文件大小无关（流式）
    let reader: Box<dyn BufRead> = match &opts.file {
        Some(path) => {
            let path = resolve_path(path);
            Box::new(BufReader::new(File::open(&path)?))
        }
        None => Box::new(BufReader::new(SAMPLE.as_bytes())),
    };

    let needle = if opts.ignore_case {
        opts.pattern.to_lowercase()
    } else {
        opts.pattern.clone()
    };

    let mut total = 0usize;
    let mut matched = 0usize;

    for (idx, line) in reader.lines().enumerate() {
        let line = line?; // 每行的 IO 错误也用 ? 往上抛
        total += 1;

        // contains 需要同类型：忽略大小写时把两边都转小写
        let haystack = if opts.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        if haystack.contains(&needle) {
            matched += 1;
            if opts.line_numbers {
                println!("{:>4} │ {}", idx + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }

    // 统计信息走 stderr，这样 `mini-grep x file > out.txt` 只会重定向匹配行
    eprintln!("\n共扫描 {} 行，匹配 {} 行", total, matched);

    Ok(matched > 0)
}

fn parse_args() -> Result<Options, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    let pattern = match args.first() {
        Some(p) if p == "-h" || p == "--help" => {
            println!("{}", USAGE);
            std::process::exit(0);
        }
        Some(p) => p.clone(),
        None => return Err(String::from("缺少关键词（用 --help 查看用法）")),
    };

    let mut opts = Options {
        pattern,
        file: None,
        ignore_case: false,
        line_numbers: true,
    };

    for arg in &args[1..] {
        match arg.as_str() {
            "-i" => opts.ignore_case = true,
            "-n" => opts.line_numbers = true,
            "-N" => opts.line_numbers = false,
            other if other.starts_with('-') => {
                return Err(format!("未知选项: {}（用 --help 查看用法）", other));
            }
            other => {
                if opts.file.is_some() {
                    return Err(String::from("一次只能指定一个文件"));
                }
                opts.file = Some(other.to_string());
            }
        }
    }

    Ok(opts)
}

/// 相对路径优先按当前目录找，找不到再按 crate 目录找。
/// 这样无论从 workspace 根目录还是案例目录运行 run.sh 都能工作。
fn resolve_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    let from_crate = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    if from_crate.exists() {
        return from_crate;
    }
    direct // 都不存在就交给 File::open 报错，错误信息更清楚
}
