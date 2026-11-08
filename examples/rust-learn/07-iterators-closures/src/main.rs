//! 07 迭代器与闭包
//!
//! 闭包 = 能捕获环境变量的匿名函数；迭代器 = 惰性求值的数据管道。
//! 这两样组合起来，就是 Rust 里最顺手的"数据处理风格"。
//! 运行：./run.sh 07

fn main() {
    println!("===== 1. 闭包基础 =====");
    // |参数| 表达式，类型通常能推断出来
    let add = |a: i32, b: i32| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // 闭包可以捕获外部变量（普通 fn 不行）
    let factor = 3;
    let multiply = |x| x * factor;
    println!("multiply(5) = {}", multiply(5));

    // move 闭包：把捕获的变量所有权搬进闭包里
    let owned = String::from("被闭包拥有");
    let show = move || println!("  show: {}", owned);
    show();
    // println!("{}", owned); // ❌ owned 已经被 move 进闭包

    println!("\n===== 2. Fn / FnMut / FnOnce =====");
    // Fn：只读捕获，可调用多次
    // FnMut：可变捕获，可调用多次
    // FnOnce：消耗捕获的值，只能调用一次
    let mut count = 0;
    {
        let mut inc = || {
            count += 1; // 可变借用 count
            count
        };
        println!("  inc 三次: {} {} {}", inc(), inc(), inc());
    } // inc 在这里离开作用域，借用结束
    println!("  循环外读 count = {}", count);

    // 消耗捕获值的闭包只能是 FnOnce
    let name = String::from("Rust");
    let consume = move || name; // 把 name 移出闭包
    println!("  consume() = {}", consume());
    // println!("{}", consume()); // ❌ 只能用一次

    println!("\n===== 3. 把闭包当参数传 =====");
    // Fn 作为类型约束，表示"可以反复调用的函数"
    println!("  apply(|x| x * 2, 10) = {}", apply(|x| x * 2, 10));

    // FnMut 允许闭包内部状态变化
    let mut total = 0;
    apply_mut(|x| total += x, 5);
    apply_mut(|x| total += x, 7);
    println!("  累加结果 total = {}", total);

    println!("\n===== 4. 迭代器是惰性的 =====");
    // map/filter 这类"适配器"不会立刻执行，只有遇到"消费者"才跑
    let lazy = (1..=5).map(|x| {
        println!("  计算 {} 的平方", x);
        x * x
    });
    println!("  lazy 已经创建，但上面没有打印任何东西");
    let squared: Vec<i32> = lazy.collect(); // 这里才真正执行
    println!("  collect 结果: {:?}", squared);

    println!("\n===== 5. 常用适配器 =====");
    // filter + map + collect
    let even_squares: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).collect();
    println!("  偶数的平方: {:?}", even_squares);

    // sum / product / count
    println!("  1..=100 求和: {}", (1..=100).sum::<i32>());
    println!("  1..=5 求积: {}", (1..=5).product::<i32>());

    // fold：自己写累积逻辑，最灵活
    let total = (1..=10).fold(0, |acc, x| acc + x);
    println!("  fold 求和: {}", total);

    // enumerate 带下标
    for (i, name) in ["甲", "乙", "丙"].iter().enumerate() {
        print!("  {}:{} ", i, name);
    }
    println!();

    // zip 把两个序列配成对，长度取短的
    let pairs: Vec<(i32, &str)> = (1..=3).zip(["a", "b", "c", "d"]).collect();
    println!("  zip: {:?}", pairs);

    // take / skip 截取；chain 拼接
    println!("  前 3 个: {:?}", (1..).take(3).collect::<Vec<i32>>()); // 1.. 是无穷迭代器
    println!("  跳过 2 个: {:?}", (1..=5).skip(2).collect::<Vec<i32>>());
    println!("  拼接: {:?}", (1..=3).chain(10..=12).collect::<Vec<i32>>());

    // flat_map 把嵌套结构摊平
    let words = ["hello world", "rust lang"];
    let flat: Vec<&str> = words.iter().flat_map(|s| s.split(' ')).collect();
    println!("  flat_map: {:?}", flat);

    // windows 滑动窗口（切片专属）
    let data = [1, 2, 4, 7, 11];
    let gaps: Vec<i32> = data.windows(2).map(|w| w[1] - w[0]).collect();
    println!("  相邻差值: {:?}", gaps);

    println!("\n===== 6. 综合：从日志里统计 =====");
    let logs = [
        "INFO  启动完成",
        "ERROR 连接超时",
        "INFO  加载配置",
        "WARN  磁盘剩余 10%",
        "ERROR 写入失败",
    ];
    // 只要 ERROR 行，取出后面的消息，转成大写
    let errors: Vec<String> = logs
        .iter()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line| line.trim_start_matches("ERROR").trim().to_uppercase())
        .collect();
    println!("  错误消息: {:?}", errors);
    println!(
        "  错误条数: {}",
        logs.iter().filter(|l| l.starts_with("ERROR")).count()
    );

    println!("\n===== 7. 自定义迭代器 =====");
    // 只要实现 next()，就自动获得 map/filter/take 等几十个方法
    let mut cd = Countdown::new(3);
    while let Some(n) = cd.next() {
        print!("{} ", n);
    }
    println!();

    for n in Countdown::new(3) {
        print!("{} ", n);
    }
    println!("← for 循环内部就是不断调 next()");

    println!("\n===== 8. 返回 impl Iterator =====");
    let first_three: Vec<u32> = squares(10).take(3).collect();
    println!("  前 3 个平方数: {:?}", first_three);
}

/// 接收任何"能当函数调用"的东西
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

/// FnMut：允许闭包修改自己的捕获状态
fn apply_mut<F>(mut f: F, x: i32)
where
    F: FnMut(i32),
{
    f(x);
}

/// 返回一个迭代器，调用方可以继续 .take() / .filter()
/// impl Iterator 是"我不告诉你具体类型，但保证能迭代"
fn squares(n: u32) -> impl Iterator<Item = u32> {
    (1..=n).map(|x| x * x)
}

/// 自定义迭代器：倒计时
struct Countdown {
    current: u32,
}

impl Countdown {
    fn new(n: u32) -> Self {
        Self { current: n }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            None
        } else {
            let value = self.current;
            self.current -= 1;
            Some(value)
        }
    }
}
