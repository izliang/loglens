//! 06 常用集合
//!
//! Vec（有序可重复）、String（字符串）、HashMap（键值）、HashSet（去重）、
//! BTreeMap（有序键值）、VecDeque（双端队列）。
//! 运行：./run.sh 06

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn main() {
    println!("===== 1. Vec 动态数组 =====");
    let mut v: Vec<i32> = Vec::new(); // 也可以是 vec![] 宏
    v.push(3);
    v.push(1);
    v.push(2);
    println!("v = {:?}，长度 {}", v, v.len());

    // 访问：下标会 panic，get 返回 Option
    println!("v[0] = {}", v[0]);
    println!("v.get(10) = {:?}", v.get(10));

    // 修改元素要 mut + 下标
    v[0] = 30;
    println!("改完: {:?}", v);

    // 排序、去重（去重要先排序，否则只能去掉相邻重复）
    v.sort();
    v.dedup();
    println!("排序去重: {:?}", v);

    // 保留满足条件的元素
    v.retain(|&x| x % 10 != 0);
    println!("retain 之后: {:?}", v);

    // 遍历：&v 借用；v 会移动；v.iter_mut() 可改
    for x in &v {
        print!("{} ", x);
    }
    println!();

    println!("\n===== 2. String 常用操作 =====");
    let mut s = String::from("hello");
    s.push_str(" world"); // 追加 &str
    s.push('!'); // 追加单个字符
    println!("{}", s);

    // 拼接：+ 会拿走左边；format! 不拿走任何一方（更常用）
    let a = String::from("foo");
    let b = String::from("bar");
    let c = format!("{}-{}", a, b);
    println!("a={} b={} c={}", a, b, c);

    // 分割与收集
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("单词: {:?}", words);

    // 替换、修剪、大小写
    println!("替换: {}", "a-b-c".replace('-', "_"));
    println!("修剪: [{}]", "  padded  ".trim());
    println!("大写: {}", "rust".to_uppercase());

    // 按字符遍历（不是按字节）
    for ch in "中文".chars() {
        print!("[{}] ", ch);
    }
    println!();

    println!("\n===== 3. HashMap 词频统计 =====");
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        // entry API：不存在就插入 0，然后对值 +1
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("map = {:?}", counts);
    println!("the 出现 {} 次", counts.get("the").copied().unwrap_or(0)); // copied 把 &usize 变 usize

    // 按次数降序排列（HashMap 本身无序，要排序只能先转成 Vec）
    let mut ranked: Vec<(&str, usize)> = counts.iter().map(|(k, v)| (*k, *v)).collect();
    ranked.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(y.0)));
    for (word, n) in &ranked {
        println!("  {:8} {}", word, n);
    }

    // 删除
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("a".to_string(), 1);
    println!("remove(a) = {:?}，剩余 {:?}", m.remove("a"), m);

    println!("\n===== 4. HashSet 去重 =====");
    let nums = vec![1, 1, 2, 3, 3, 3, 4];
    let unique: HashSet<i32> = nums.iter().copied().collect();
    println!("去重后: {:?}，共 {} 个", unique, unique.len());
    println!("包含 3 吗: {}", unique.contains(&3));

    // 集合运算
    let even: HashSet<i32> = [2, 4, 6].into_iter().collect();
    let odd: HashSet<i32> = [1, 3, 5].into_iter().collect();
    let both: HashSet<i32> = unique.intersection(&even).copied().collect();
    println!("与偶数集合的交集: {:?}", both);
    println!("并集大小: {}", even.union(&odd).count());

    println!("\n===== 5. BTreeMap 自动按 key 排序 =====");
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("小明", 90);
    scores.insert("小红", 95);
    scores.insert("阿强", 80);
    for (name, score) in &scores {
        println!("  {} = {}", name, score);
    }
    // 取第一个/最后一个
    println!("第一个: {:?}", scores.first_key_value());
    println!("最高分: {:?}", scores.values().max());

    println!("\n===== 6. VecDeque 双端队列 =====");
    // 队列（FIFO）和栈（LIFO）都能做，两端插入删除都是 O(1)
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("deque = {:?}", deque);
    println!("弹出后面: {:?}", deque.pop_back());
    println!("弹出前面: {:?}", deque.pop_front());
    println!("剩余: {:?}", deque);

    println!("\n===== 7. 结构体集合的综合例子 =====");
    let mut logs: Vec<(&str, u16)> = vec![
        ("INFO", 12),
        ("ERROR", 3),
        ("WARN", 7),
        ("INFO", 21),
        ("ERROR", 5),
    ];
    // 按级别聚合
    let mut by_level: HashMap<&str, u16> = HashMap::new();
    for (level, count) in &logs {
        // level 是 &&str，count 是 &u16，各解一次引用
        *by_level.entry(*level).or_insert(0) += *count;
    }
    // 按总量降序
    logs.sort_by(|a, b| b.1.cmp(&a.1));
    println!("明细（按数量降序）: {:?}", logs);
    println!("按级别汇总: {:?}", by_level);
}

// 小提示：下面的写法很常见，值得记住
//   let v: Vec<i32> = (1..=5).collect();          范围转 Vec
//   let s: String = ['a','b'].iter().collect();   字符转 String
//   counts.entry(k).or_insert(0) += 1;            计数惯用法
