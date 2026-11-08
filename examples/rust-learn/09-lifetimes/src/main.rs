//! 09 生命周期
//!
//! 生命周期的作用只有一个：让编译器知道「这个引用能活多久」，
//! 从而保证引用永远不会指向已经被释放的数据。
//! 大多数时候编译器能自动推断，需要你手写 'a 的地方其实不多。
//! 运行：./run.sh 09

use std::fmt::Display;

fn main() {
    println!("===== 1. 为什么需要生命周期 =====");
    // 下面这行如果放开，编译报错：
    //   let r;
    //   {
    //       let x = 5;
    //       r = &x;      // x 在这一行结束时就被释放了
    //   }
    //   println!("{}", r);  // ❌ r 指向已释放的内存
    //
    // Rust 用借用检查器在编译期挡住这种错误，不需要 GC，也没有悬垂指针。

    println!("\n===== 2. 函数签名里的 'a =====");
    let s1 = String::from("长长的字符串");
    let s2 = String::from("短");
    let result = longest(&s1, &s2);
    println!("longest = {}", result);
    // longest 的返回值和两个参数活得一样久（取较短的那个上限），
    // 所以只要 result 还在用，s1/s2 就不能被释放。

    println!("\n===== 3. 省略规则：只有一个引用参数时不用写 =====");
    // fn first_word(s: &str) -> &str 等价于
    // fn first_word<'a>(s: &'a str) -> &'a str
    let text = String::from("hello world");
    println!("first_word = {}", first_word(&text));

    println!("\n===== 4. 结构体里存引用 =====");
    let novel = String::from("很久很久以前，有一个程序员……");
    let first_sentence = novel.split('，').next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("摘录: {}", excerpt.part);
    println!("播报: {}", excerpt.announce("请注意"));

    println!("\n===== 5. 'static：活到程序结束 =====");
    let s: &'static str = "字符串字面量天生就是 'static";
    println!("{}", s);
    println!("{}", static_str());
    println!("常量: {}", GREETING);

    println!("\n===== 6. 多个生命周期参数 =====");
    let a = String::from("aaa");
    let b = String::from("bb");
    let r = longer(&a, &b);
    println!("更长的是: {}", r);

    println!("\n===== 7. 带生命周期约束的泛型 =====");
    println!("{}", announce_and_longest(&a, &b, "提示：两个字符串比较中"));

    println!("\n===== 8. 生命周期不是作用域块 =====");
    // 'a 是「引用有效的最短区间」，不是某对大括号。
    // 它是编译器推导出来的约束，不是运行时真的存在的东西（零成本）。
}

/// 返回值的生命周期 = 两个参数里较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 只有一个引用参数：返回值的生命周期自动跟随它（省略规则 1）
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// 结构体持有引用，必须标生命周期
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// 返回 &str 时，省略规则 2 让它跟随 &self，而不是 'a
    fn announce(&self, announcement: &str) -> &str {
        println!("  注意！{}", announcement);
        self.part
    }
}

const GREETING: &str = "常量里的引用也是 'static";

/// 返回 'static 引用：字符串字面量存在二进制里，永远有效
fn static_str() -> &'static str {
    "我是静态字符串"
}

/// 两个参数各自有生命周期；'b: 'a 表示 y 至少和 'a 活得一样久，
/// 所以 y 可以当成 &'a str 返回（子类型可以"缩短"使用）
fn longer<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        // y 是 &'b str，因为 'b: 'a，在这里可以当作 &'a str 用
        y
    }
}

/// 泛型 + 生命周期：T 必须活得比 'a 长
fn announce_and_longest<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("  {}", announcement);
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// 常见报错与含义：
//   missing lifetime specifier
//     → 有多个引用参数，但返回值是引用，编译器不知道跟随谁
//   `x` does not live long enough
//     → 被引用的值比引用先死，缩短引用的使用范围或交出所有权
//   lifetime may not live long enough
//     → 结构体/泛型的生命周期约束没写对，加 'a 约束
