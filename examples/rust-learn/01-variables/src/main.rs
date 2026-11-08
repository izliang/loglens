//! 01 变量与类型
//!
//! 目标：搞清楚"默认不可变"、遮蔽、基本类型、复合类型，
//!       以及 String 和 &str 的区别（后面会反复遇到）。
//! 运行：./run.sh 01

// 常量：必须标注类型，编译期就确定，通常全大写
const MAX_POINTS: u32 = 100_000;

// 静态变量：也是全局的，但地址固定，生命周期是 'static
static GREETING: &str = "来自静态变量";

fn main() {
    println!("===== 1. 默认不可变 =====");
    let x = 5;
    println!("x = {}", x);
    // x = 6; // 取消注释会报错：cannot assign twice to immutable variable

    // 想改就加 mut
    let mut y = 5;
    y = y + 1;
    println!("y = {}", y);

    println!("\n===== 2. 遮蔽（shadowing）=====");
    // 用同名 let 覆盖前一个变量，连类型都能换——这是 Rust 独有的便利
    let spaces = "   ";
    let spaces = spaces.len(); // 从 &str 变成 usize
    println!("spaces 现在是数字: {}", spaces);

    // 对比：mut 只能改值，不能改类型
    let mut n = 10;
    n = n + 1;
    // n = "文字"; // 报错：mismatched types
    println!("n 改完是 {}", n);

    println!("\n===== 3. 基本类型 =====");
    // 整数：i8/i16/i32/i64/i128/isize，无符号是 u 开头
    // isize/usize 跟机器字长一致，64 位系统上是 64 位
    let a: i32 = -7;
    let b: u8 = 200; // 范围 0..=255
    let c: usize = 3; // 常用于下标、长度
    println!("a={} b={} c={}", a, b, c);

    // 数字可读性：下划线随便加
    let million = 1_000_000;
    println!("一百万 = {}", million);

    // 溢出：debug 模式会 panic，release 模式回绕
    let max_u8: u8 = 255;
    println!("255 + 1 用 wrapping_add = {}", max_u8.wrapping_add(1)); // 0
    println!("用 checked_add = {:?}", max_u8.checked_add(1)); // None，更安全

    // 浮点：f32 / f64，默认 f64
    let price = 19.99_f64;
    println!("price 四舍五入 = {}", price.round());
    println!("0.1 + 0.2 = {}（浮点精度问题）", 0.1 + 0.2);

    // 布尔和字符（char 是 4 字节 Unicode 标量，不等于一个字节）
    let ok: bool = true;
    let emoji = '🦀'; // 螃蟹是 Rust 的吉祥物
    println!("ok={} emoji={} 占 {} 字节", ok, emoji, emoji.len_utf8());

    // 类型转换用 as（可能截断），解析字符串用 parse
    let float_num = 3.9_f64;
    println!("3.9 as i32 = {}（直接截断，不四舍五入）", float_num as i32);
    let parsed: i32 = "42".parse().expect("解析失败");
    println!("字符串转数字 = {}", parsed);

    println!("\n===== 4. 元组 =====");
    let tup: (i32, f64, char) = (500, 6.4, 'x');
    // 解构
    let (width, height, mark) = tup;
    println!("解构: {} {} {}", width, height, mark);
    // 按下标访问（注意是 .0 .1，不是 [0]）
    println!("tup.0 = {}", tup.0);

    // 元组可以返回多个值，这是 Rust 里"多返回值"的标准做法
    let (sum, diff) = add_sub(10, 3);
    println!("10 和 3: 和={} 差={}", sum, diff);

    println!("\n===== 5. 数组 =====");
    // 长度固定，类型统一，存在栈上
    let arr = [1, 2, 3, 4, 5];
    println!("整个数组: {:?}，长度 {}", arr, arr.len());
    println!("第一个元素: {}", arr[0]);
    // arr[10]; // 运行时 panic：index out of bounds

    // 创建全 0 的数组，长度为 5
    let zeros = [0u8; 5];
    println!("zeros = {:?}", zeros);

    // 遍历
    let mut total = 0;
    for v in arr {
        total += v;
    }
    println!("数组求和 = {}", total);

    println!("\n===== 6. String 与 &str =====");
    // &str：借用的字符串切片，不可变，通常来自字面量
    let literal: &str = "我是字面量";

    // String：拥有所有权、可增长、存在堆上
    let mut owned: String = String::from("我");
    owned.push_str("是");
    owned.push('S');
    owned.push_str("tring");
    println!("literal = {}", literal);
    println!("owned   = {}", owned);

    // 长度是"字节数"不是"字符数"，中文一个字占 3 字节
    println!("owned 字节长度 = {}", owned.len());
    println!("owned 字符个数 = {}", owned.chars().count());

    // 常用转换
    let from_literal = literal.to_string(); // &str -> String
    let to_str: &str = &from_literal; // String -> &str（借用）
    println!("from_literal = {}, to_str = {}", from_literal, to_str);

    println!("\n===== 7. 常量与静态 =====");
    println!("常量 MAX_POINTS = {}", MAX_POINTS);
    println!("静态 GREETING = {}", GREETING);
    // 区别：const 在编译期内联复制；static 只有一份实例，可取地址
}

/// 返回两个值：用元组是最简单的做法
fn add_sub(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}
