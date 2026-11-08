//! 02 函数与控制流
//!
//! 目标：理解"语句 vs 表达式"，掌握 if / loop / while / for / match。
//! 运行：./run.sh 02

fn main() {
    println!("===== 1. 函数 =====");
    println!("add(2, 3) = {}", add(2, 3));
    println!("带默认行为的问候: {}", greet("小明"));
    println!("计算器: {:?}", calc(7, 3));

    // 早返回（early return）：满足条件就直接 return，后面的代码不执行
    println!("检查年龄 17 -> {}", check_age(17));
    println!("检查年龄 20 -> {}", check_age(20));

    println!("\n===== 2. 语句与表达式 =====");
    // 语句：做了事但不产生值，末尾有分号
    // 表达式：产生值，没有分号
    let y = {
        let x = 3;
        x + 1 // ← 注意没有分号，这个块的值就是 4
    };
    println!("块表达式的值 y = {}", y);

    println!("\n===== 3. if 也是表达式 =====");
    let n = 7;
    // if 的每个分支必须返回相同类型
    let size = if n > 5 { "大" } else { "小" };
    println!("{} 是{}数字", n, size);

    // 注意：Rust 没有"真值"概念，条件必须是 bool
    // if n { }  // ❌ 编译错误，不像 JS 那样自动转换

    // else if 链
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 60 {
        "C"
    } else {
        "D"
    };
    println!("分数 {} -> 等级 {}", score, grade);

    println!("\n===== 4. loop / while / for =====");
    // loop 是无限循环，break 可以带回一个值
    let mut counter = 0;
    let doubled = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 把 20 作为 loop 的值返回
        }
    };
    println!("loop 重试了 {} 次，返回值 {}", counter, doubled);

    // while
    let mut fuel = 3;
    while fuel > 0 {
        println!("  剩余燃料 {}", fuel);
        fuel -= 1;
    }
    println!("  燃料耗尽，起飞");

    // for 遍历范围（最常用，不用自己管下标）
    let mut sum = 0;
    for i in 1..=5 {
        sum += i; // 1..=5 是闭区间，1..5 是左闭右开
    }
    println!("1 到 5 求和 = {}", sum);

    // for 遍历集合，用 .iter() 拿引用，不消耗集合
    let fruits = ["苹果", "香蕉", "橘子"];
    for f in fruits.iter() {
        print!("{} ", f);
    }
    println!();

    // 需要下标时用 enumerate
    for (idx, f) in fruits.iter().enumerate() {
        println!("  {}: {}", idx, f);
    }

    println!("\n===== 5. 带标签的循环 =====");
    // 嵌套循环时，可以用标签指定 break/continue 跳哪一层
    'outer: for i in 1..=4 {
        for j in 1..=4 {
            if i * j > 6 {
                println!("  i={} j={} 乘积超过 6，跳出外层", i, j);
                break 'outer;
            }
        }
    }

    // continue 跳过本次，只影响最近一层（或指定标签）
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!("← 1 到 10 的奇数");

    println!("\n===== 6. match 模式匹配 =====");
    // match 必须穷尽所有情况，编译器会检查
    let code = 404;
    let msg = match code {
        200 => "成功",
        301 | 302 => "重定向",
        404 => "页面不存在",
        500..=599 => "服务器错误", // 可以用区间
        _ => "未知状态码",         // 下划线是兜底
    };
    println!("HTTP {} -> {}", code, msg);

    // 匹配元组：一次看两个值
    let point = (0, -3);
    match point {
        (0, 0) => println!("  原点"),
        (0, y) => println!("  在 y 轴上，y = {}", y),
        (x, 0) => println!("  在 x 轴上，x = {}", x),
        (x, y) => println!("  普通点 ({}, {})", x, y),
    }

    // 带条件守卫（guard）和绑定
    let pair = (2, -3);
    match pair {
        (x, y) if x + y > 0 => println!("  ({}, {}) 两数之和为正", x, y),
        (x, y) if x + y < 0 => println!("  ({}, {}) 两数之和为负", x, y),
        _ => println!("  两数之和为零"),
    }

    // 用 @ 同时绑定和判断范围
    let age = 25;
    match age {
        n @ 0..=17 => println!("  {} 岁，未成年", n),
        n @ 18..=65 => println!("  {} 岁，成年", n),
        n => println!("  {} 岁，老年", n),
    }

    // 匹配字符串
    let cmd = "start";
    match cmd {
        "start" => println!("  启动服务"),
        "stop" => println!("  停止服务"),
        other => println!("  未知命令: {}", other),
    }

    println!("\n===== 7. if let / while let =====");
    // 只关心一种情况时，if let 比 match 简短
    let some_value = Some(42);
    if let Some(v) = some_value {
        println!("  if let 取到值: {}", v);
    }

    // while let：一直匹配直到不匹配
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top); // 输出 3 2 1
    }
    println!("← 用 while let 弹空栈");
}

/// 两个 i32 相加。最后一个表达式没有分号，就是返回值
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 带默认参数的模拟：Rust 没有默认参数，用 Option 或重载函数名实现
fn greet(name: &str) -> String {
    format!("你好，{}！", name)
}

/// 返回元组，模拟"多个返回值"
fn calc(a: i32, b: i32) -> (i32, i32, i32) {
    (a + b, a - b, a * b)
}

/// 早返回：用 return 提前结束，函数体最后仍然是一个表达式
fn check_age(age: u32) -> &'static str {
    if age < 18 {
        return "未成年";
    }
    "成年"
}
