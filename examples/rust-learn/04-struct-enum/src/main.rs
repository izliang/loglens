//! 04 结构体与枚举
//!
//! 目标：用 struct 组织数据，用 impl 挂方法，用 enum + match 表达"多选一"。
//!       Option<T> 就是标准库里的枚举，第 5 节会看到它为什么能消灭空指针。
//! 运行：./run.sh 04

use std::fmt;

// #[derive(...)] 让编译器自动生成常用实现
// Debug：能用 {:?} 打印；Clone：能 .clone()
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

// impl 块里放"方法"（有 self）和"关联函数"（没有 self，类似静态方法）
impl User {
    /// 关联函数：用 User::new(..) 调用，习惯上叫 new
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(), // 字符串字段通常存 String（拥有所有权）
            age,                    // 字段名和变量名相同时可以简写
            active: true,
        }
    }

    /// &self：只读借用，不拿走所有权
    fn describe(&self) -> String {
        let status = if self.active { "活跃" } else { "停用" };
        format!("{}（{} 岁，{}）", self.name, self.age, status)
    }

    /// &mut self：可写借用
    fn birthday(&mut self) {
        self.age += 1;
    }

    /// self：拿走所有权，调用后原变量不能再用
    fn into_name(self) -> String {
        self.name
    }

    /// 判断是否成年，返回 bool
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

// 元组结构体：有类型名，但字段没有名字，用 .0 .1 访问
#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn distance_squared(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1
    }
}

// 为自定义类型实现 Display，就能用 {} 打印
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 枚举：每个变体可以携带不同类型的数据
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 }, // 命名字段
    Rect { width: f64, height: f64 },
    Triangle(f64, f64), // 元组风格：底、高
}

impl Shape {
    // match 必须覆盖所有变体，少一个编译不过
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rect { width, height } => width * height,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "圆",
            Shape::Rect { .. } => "矩形",
            Shape::Triangle(..) => "三角形",
        }
    }
}

fn main() {
    println!("===== 1. 创建和访问 =====");
    let mut alice = User::new("Alice", 17);
    println!("{:?}", alice); // Debug 打印
    println!("{}", alice.describe());

    // 字段访问和修改（变量必须是 mut）
    alice.birthday();
    println!("过完生日: {}", alice.describe());
    println!("是否成年: {}", alice.is_adult());

    println!("\n===== 2. 结构体更新语法 =====");
    // .. 表示"剩下的字段从另一个实例拿"
    let bob = User {
        name: String::from("Bob"),
        ..alice.clone() // 这里用 clone，否则 alice 会被移动
    };
    println!("bob = {:?}", bob);
    println!("alice 仍然可用 = {:?}", alice.name);

    println!("\n===== 3. 元组结构体与 Display =====");
    let p = Point(3, 4);
    println!("{} 的平方距离 = {}", p, p.distance_squared()); // 用了 Display
    println!("{:?}", p); // 同时也有 Debug

    println!("\n===== 4. 枚举 + match =====");
    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rect {
            width: 2.0,
            height: 3.0,
        },
        Shape::Triangle(4.0, 5.0),
    ];
    for s in &shapes {
        println!("  {} 的面积 = {:.2}", s.name(), s.area());
    }

    // 枚举也可以带"不同类型"的载荷，这是它比 C 枚举强的地方
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("你好")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in &msgs {
        match m {
            Message::Quit => println!("  退出"),
            Message::Move { x, y } => println!("  移动到 ({}, {})", x, y),
            Message::Write(text) => println!("  写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("  改色: #{:02X}{:02X}{:02X}", r, g, b),
        }
    }

    println!("\n===== 5. Option<T> 就是枚举 =====");
    // 标准库里定义成：enum Option<T> { Some(T), None }
    // 它强制你处理"没有值"的情况，所以 Rust 没有 null
    let users = vec![alice.clone(), bob.clone()];
    println!(
        "找 Alice: {:?}",
        find_user(&users, "Alice").map(|u| &u.name)
    );
    println!("找 Zoe:   {:?}", find_user(&users, "Zoe").map(|u| &u.name));

    // 几种取值方式
    let found = find_user(&users, "Zoe");
    match found {
        Some(u) => println!("  找到了 {}", u.name),
        None => println!("  没找到 Zoe"),
    }
    println!(
        "  unwrap_or 兜底: {}",
        find_user(&users, "Zoe").map_or("查无此人".to_string(), |u| u.name.clone())
    );

    // ? 会在第 05 节详细讲，这里体现"Option 也能用 ? 传播"
    println!("  用 if let 取值:");
    if let Some(u) = find_user(&users, "Bob") {
        println!("    {}", u.describe());
    }

    println!("\n===== 6. 会拿走所有权的 into_name =====");
    let carol = User::new("Carol", 30);
    let name = carol.into_name(); // carol 被消费
                                  // println!("{:?}", carol); // ❌ 这里已经不能用了
    println!("拿到的名字: {}", name);
}

/// 在切片里按名字查找，返回 Option<&User>
/// 找不到时返回 None，而不是空指针或抛异常
fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
    users.iter().find(|u| u.name == name)
}
