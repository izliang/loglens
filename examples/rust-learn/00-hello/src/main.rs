//! 00 最小可运行程序
//!
//! 目标：确认工具链可用，认识 println! 的几种写法。
//! 运行：./run.sh 00  或  cargo run -p hello

fn main() {
    // 最基本的输出，末尾自动换行
    println!("你好，Rust！");

    // print! 不换行，注意 Rust 不会自动加空格
    print!("这一行");
    print!("不会换行");
    println!();

    // eprintln! 输出到标准错误流，终端里通常显示为红色
    eprintln!("这条走 stderr，常用于打日志和报错");

    // ---- 格式化占位符 ----
    let name = "初学者";
    let day = 1;

    println!("{} 学习 Rust 的第 {} 天", name, day); // 按顺序填充
    println!("{0} 又 {0}，位置参数可以重复用", name); // 按下标填充
    println!("{name} 用命名参数更直观"); // 直接用变量名（Rust 1.58+）

    // {:?} 是"调试"格式，能打印元组、数组、结构体
    let point = (3, 4);
    println!("调试输出: {:?}", point);
    // {:#?} 是带缩进的美化版本，打印嵌套结构时特别好用
    println!("美化输出: {:#?}", point);

    // ---- 数字格式 ----
    println!("二进制: {:b}", 42);
    println!("补零: {:05}", 42); // 宽度 5，左边补 0
    println!("保留两位: {:.2}", 123.456);
    println!("左对齐: [{:<6}]", "ab"); // 宽度 6 左对齐
    println!("右对齐: [{:>6}]", "ab");

    // ---- 一些便利宏 ----
    let v = vec![1, 2, 3]; // 创建 Vec
    println!("vec! 宏创建的向量: {:?}", v);
    let s = format!("用 format! 拼出来的字符串: {} + {}", name, day);
    println!("{}", s);

    // dbg! 会打印"文件:行号 = 值"，并原样返回，适合临时排查
    let doubled = dbg!(day * 2);
    println!("dbg! 的返回值: {}", doubled);
}
