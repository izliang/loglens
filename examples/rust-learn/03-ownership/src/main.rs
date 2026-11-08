//! 03 所有权、借用与切片
//!
//! 这是 Rust 最核心、也最劝退的一课。核心只有三条规则：
//!   1. 每个值都有一个"所有者"变量；
//!   2. 同一时刻只能有一个所有者；
//!   3. 所有者离开作用域时，值被自动释放（drop）。
//!
//! 运行：./run.sh 03

fn main() {
    println!("===== 1. 栈和堆 =====");
    // 栈：编译期大小已知，读写快，函数返回自动弹出
    // 堆：大小运行时才知道，要申请/释放，所以需要"所有权"来管理
    let x = 5;
    let y = x; // i32 在栈上，直接复制一份
    println!("x={} y={}（i32 是 Copy，两者都能用）", x, y);

    println!("\n===== 2. 移动（move）=====");
    let s1 = String::from("hello");
    let s2 = s1; // 注意：这里是"移动"，不是浅拷贝
                 // println!("{}", s1); // ❌ 取消注释会报错：
                 // error[E0382]: borrow of moved value: `s1`
    println!("s2 = {}（s1 已经被移动走了，不能用）", s2);

    println!("\n===== 3. 克隆（clone）=====");
    // 想要两份独立的数据，必须显式 clone，代价是深拷贝
    let a = String::from("world");
    let b = a.clone();
    println!("a={} b={} 两份数据互相独立", a, b);

    println!("\n===== 4. Copy 类型 =====");
    // 实现了 Copy trait 的类型（整数、浮点、bool、char、只含 Copy 的元组/数组）
    // 赋值时是复制，不会有移动问题
    let n1 = 5;
    let n2 = n1;
    println!("n1={} n2={}", n1, n2);

    let t1 = (1, 2.5, true);
    let t2 = t1;
    println!("t1={:?} t2={:?}", t1, t2);

    println!("\n===== 5. 函数传参也会移动 =====");
    let s = String::from("传进去了");
    takes_ownership(s); // s 的所有权移进函数
                        // println!("{}", s); // ❌ s 已经没了

    let n = 5;
    makes_copy(n); // i32 是 Copy，传的是复制
    println!("n 仍然可用: {}", n);

    println!("\n===== 6. 借用（borrow）=====");
    // 不想转移所有权，就传引用 &T。借用不会释放原值
    let s = String::from("借用示例");
    let len = calculate_length(&s);
    println!("「{}」的长度是 {}，s 仍然可用", s, len);

    println!("\n===== 7. 可变借用 =====");
    let mut text = String::from("开始");
    append_suffix(&mut text); // &mut T 允许函数修改
    println!("text = {}", text);

    println!("\n===== 8. 借用规则 =====");
    // 规则：同一作用域内，要么多个不可变借用，要么一个可变借用
    let s = String::from("规则");
    let r1 = &s;
    let r2 = &s; // 多个 & 没问题
    println!("r1={} r2={}", r1, r2);

    // 可变借用必须独占
    let mut s2 = String::from("独占");
    let m = &mut s2;
    m.push_str("！");
    // println!("{}", s2); // ❌
    // error[E0502]: cannot borrow `s2` as immutable
    //               because it is also borrowed as mutable
    println!("可变借用用完: {}", m);

    // NLL（非词法生命周期）：引用的"最后一次使用"之后，借用就结束了
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("first = {}", first); // first 到这里就不再使用
    v.push(4); // ✅ 所以这里可以可变借用
    println!("v = {:?}", v);

    println!("\n===== 9. 切片 =====");
    // 切片是对一段连续数据的"视图"，不复制数据
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} / {}", hello, world);
    println!("省略写法: {} 和 {}", &s[..5], &s[6..]);

    let first = first_word(&s);
    println!("第一个单词: {}", first);

    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // 类型是 &[i32]
    println!("数组切片 {:?}，长度 {}", slice, slice.len());

    // 字符串切片的下标是"字节"，必须落在字符边界上
    let cn = String::from("中文");
    println!("「中文」字节长度 = {}", cn.len());
    let first_char = &cn[0..3]; // 一个中文字符占 3 字节
    println!("第一个字: {}", first_char);
    // let bad = &cn[0..1]; // ❌ 运行时 panic: byte index 1 is not a char boundary

    println!("\n===== 10. 返回引用需要生命周期 =====");
    // 两个参数都是 &str，返回值也是 &str：编译器需要知道返回的引用跟着谁活
    // 这里先照抄写法，第 09 节专门讲 'a 是什么意思
    println!("较长的那个: {}", longest("abc", "abcd"));
}

/// 接收 String 的所有权：函数结束时 s 被释放
fn takes_ownership(s: String) {
    println!("函数拿到了「{}」，函数结束时它会自动释放", s);
}

/// i32 是 Copy，传进来的是副本，不影响外面的变量
fn makes_copy(n: i32) {
    println!("函数拿到副本 n = {}", n);
}

/// 只借用，不接管所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 可变借用：允许修改，但不允许拿走
fn append_suffix(s: &mut String) {
    s.push_str("，继续");
}

/// 找出第一个空格前的部分，返回的是原字符串的切片（没有复制）
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// 'a 表示：返回的引用活得不能超过 a 和 b 中较短的那个
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
