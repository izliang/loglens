//! 08 泛型与 trait
//!
//! 泛型 = 写一份代码适配多种类型；trait = 规定"这些类型必须会什么"。
//! 两者结合就是 Rust 的多态：编译期单态化（泛型）+ 运行时动态分发（trait 对象）。
//! 运行：./run.sh 08

use std::fmt::{self, Display};

fn main() {
    println!("===== 1. 泛型函数 =====");
    // 同一份代码，T 可以是任何实现了 PartialOrd + Copy 的类型
    println!("最大整数: {}", largest(&[3, 7, 2, 9, 4]));
    println!("最大浮点: {}", largest(&[3.1, 7.5, 2.2]));
    println!("最大字符: {}", largest(&['a', 'z', 'm']));

    println!("\n===== 2. 泛型结构体 =====");
    let int_point = Point::new(1, 2);
    let float_point = Point::new(1.5, 2.5);
    println!("int_point = {}", int_point); // 用了下面手写的 Display
    println!("float_point = {}", float_point);
    println!("int_point.x() = {}", int_point.x());
    // 注意：两个 Point 的 T 不同，就是两个不同的类型，不能互相赋值

    println!("\n===== 3. 带约束的 impl 块 =====");
    // 只有 T 同时满足 Display + PartialOrd，才有 cmp_display 方法
    let p1 = Pair::new(3, 8);
    p1.cmp_display();
    // Pair::new("a", "b") 虽然能创建，但没有 cmp_display —— 编译期就没有这个方法

    println!("\n===== 4. 定义 trait =====");
    let article = NewsArticle {
        headline: String::from("Rust 1.94 发布"),
        author: String::from("张三"),
    };
    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("今天也在写 Rust"),
    };
    println!("文章摘要: {}", article.summarize());
    println!("推文摘要: {}", tweet.summarize()); // 用的是 Tweet 自己的实现
    println!("文章预览: {}", article.preview()); // 用的是 trait 的默认实现
    println!("推文预览: {}", tweet.preview());

    println!("\n===== 5. 把 trait 当参数 =====");
    notify(&article); // impl Trait 语法糖
    notify_generic(&tweet); // 等价的泛型写法

    println!("\n===== 6. 返回 impl Trait =====");
    let t = make_tweet();
    println!("工厂返回: {}", t.summarize());

    println!("\n===== 7. trait 对象（动态分发） =====");
    // Box<dyn Summary> 可以放进同一个 Vec，即使底层类型不同
    let items: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(tweet)];
    for item in &items {
        println!("  {}", item.summarize());
    }
    print_all(&items);

    println!("\n===== 8. 综合：where 子句 =====");
    println!("{}", describe(42, "hello"));
    println!("{}", describe(2.5, 'x'));
}

/// 泛型函数：T 必须能比较（PartialOrd）且能按值复制（Copy）
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

/// 泛型结构体
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

/// 只为实现了 Display 的 T 实现 Display
impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

/// 约束写在 impl 上：只有满足条件的 T 才有这个方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("最大值是 {}", self.first);
        } else {
            println!("最大值是 {}", self.second);
        }
    }
}

/// trait 定义：必须实现 summarize，preview 有默认实现
trait Summary {
    fn summarize(&self) -> String;

    fn preview(&self) -> String {
        format!("[预览] {}", self.summarize())
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} —— {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// impl Trait 参数：本质是泛型语法糖
fn notify(item: &impl Summary) {
    println!("  收到通知: {}", item.summarize());
}

/// 完全等价的泛型写法
fn notify_generic<T: Summary>(item: &T) {
    println!("  收到通知(泛型): {}", item.summarize());
}

/// 返回 impl Trait：调用方只关心"能摘要"，不关心具体类型
fn make_tweet() -> impl Summary {
    Tweet {
        username: String::from("@demo"),
        content: String::from("返回了一个实现了 Summary 的值"),
    }
}

/// 接收 trait 对象切片：不同类型可以混在一个集合里
fn print_all(items: &[Box<dyn Summary>]) {
    for item in items {
        println!("  [遍历] {}", item.summarize());
    }
}

/// where 子句：约束多的时候比 <T: A + B + C> 更好读
fn describe<T, U>(a: T, b: U) -> String
where
    T: Display,
    U: Display,
{
    format!("a={} b={}", a, b)
}

// 对比记忆：
//   泛型 T: Trait        → 单态化，编译期为每种类型生成一份代码，零运行时开销，体积变大
//   dyn Trait / Box<dyn> → 虚表调用，运行时多态，一份代码，略有开销
