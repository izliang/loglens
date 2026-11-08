# 06 常用集合

## 这一节要会什么

- `Vec`：增删改查、排序、`retain`
- `String`：拼接、分割、`chars()` 按字符遍历
- `HashMap`：`entry API` 计数，遍历转 `Vec` 再排序
- `HashSet`：去重与集合运算
- `BTreeMap`：需要 key 有序时用它
- `VecDeque`：双端队列

## 运行

```bash
./run.sh 06
```

## 集合速查

| 类型 | 特点 | 典型场景 | 复杂度 |
|---|---|---|---|
| `Vec<T>` | 连续存储、可增长 | 几乎所有"列表" | 尾部 O(1)，中间插删 O(n) |
| `String` | UTF-8 字节序列 | 文本 | 拼接均摊 O(1) |
| `HashMap<K, V>` | 无序、哈希查找 | 计数、索引、缓存 | 平均 O(1) |
| `HashSet<T>` | 无序、不重复 | 去重、成员判断 | 平均 O(1) |
| `BTreeMap<K, V>` | **按 key 有序** | 需要排序输出 | O(log n) |
| `VecDeque<T>` | 两端高效增删 | 队列、滑动窗口 | 两端 O(1) |

## 关键概念

### 1. Vec 的访问方式

```rust
let v = vec![1, 2, 3];
v[10]        // 直接 panic
v.get(10)    // 返回 Option<&T>，安全
```

规则：**能用 `get` 就用 `get`**，除非你 100% 确定下标合法。

### 2. HashMap 的 entry API

```rust
*counts.entry(word).or_insert(0) += 1;
```

这一行完成了"没有就插入 0，然后加一"。等价于先 `contains_key` 再 `insert`，但只查一次表，而且是官方推荐写法。

其他变体：

| 方法 | 行为 |
|---|---|
| `or_insert(v)` | 不存在则插入 v |
| `or_insert_with(\|\| f())` | 惰性构造默认值 |
| `or_default()` | 插入 `Default::default()`（数字 0、字符串空） |
| `and_modify(\|v\| *v += 1)` | 存在时修改 |

### 3. HashMap 无序，要排序先转 Vec

```rust
let mut ranked: Vec<(&str, usize)> = counts.iter().map(|(k, v)| (*k, *v)).collect();
ranked.sort_by(|a, b| b.1.cmp(&a.1));
```

`sort_by` 里 `b.cmp(&a)` 就是降序。要多个条件用 `.then(...)` 接。

### 4. String 的三个"长度"

| 方法 | 结果 | 说明 |
|---|---|---|
| `s.len()` | 字节数 | `"中文".len() == 6` |
| `s.chars().count()` | 字符数 | `"中文".chars().count() == 2` |
| `s.split_whitespace().count()` | 词数 | 按空白切 |

Rust 的字符串是 UTF-8，**不能按下标随机访问字符**，只能 `chars()` 遍历或按字节切片（必须落在字符边界）。

### 5. 遍历时能不能改

| 写法 | 能否修改元素 | 集合之后还能用吗 |
|---|---|---|
| `for x in &v` | ❌ | ✅ |
| `for x in &mut v` | ✅ | ✅ |
| `for x in v` | 拿走所有权 | ❌ |
| `v.retain(\|x\| ...)` | 边遍历边删 | ✅ |

## 预期输出（节选）

```
v = [3, 1, 2]，长度 3
v.get(10) = None
排序去重: [1, 2, 3]
单词: ["hello", "world!"]
the 出现 2 次
  fox      2
  the      2
  brown    1
去重后: {4, 3, 2, 1}，共 4 个
并集大小: 6
```

> `HashSet` 的打印顺序每次运行可能不同，这是哈希随机化的正常现象。

## 试着改改

1. 统计一段中文文本里每个字出现的次数（`text.chars()`，注意 `HashMap<char, usize>`）。
2. 把 `ranked` 的排序改成"次数相同时按字母序升序"（`then` 已经写好了，反着试一下）。
3. 用 `BTreeMap` 重写词频统计，观察输出是否自动按字母排序。
4. 用 `VecDeque` 实现一个简单的"最近 3 条日志"滑动窗口。

## 常见坑

- `for x in v` 之后再用 `v` → moved。
- 一边 `for x in &v` 一边 `v.push(..)` → 借用冲突（E0502）。
- `HashMap` 的 key 必须实现 `Hash + Eq`，`f64` 不行（没有 `Eq`）。
- `String` 用 `s[0]` 取字符会编译错误，必须 `chars().nth(0)` 或切片。
