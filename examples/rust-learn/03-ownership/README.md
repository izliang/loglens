# 03 所有权、借用与切片

> 这一节是 Rust 的分水岭。看懂它，后面 80% 的报错都能自己解决。

## 这一节要会什么

- 三条所有权规则
- 什么是"移动（move）"，为什么 `let s2 = s1` 之后 `s1` 不能用了
- `clone` 和 `Copy` 分别在什么时候用
- 借用 `&T` / `&mut T` 的规则（为什么可变借用必须独占）
- 切片 `&str` / `&[T]` 是"视图"不是复制

## 运行

```bash
./run.sh 03
```

## 核心模型

### 三条规则

1. 每个值都有**唯一**的所有者变量。
2. 同一时刻**只能有一个**所有者。
3. 所有者离开作用域，值被**自动释放**（`drop`）。

Rust 靠这三条在**编译期**完成内存管理，不需要 GC，也不用手写 `free`。

### 赋值到底发生了什么

| 类型 | `let b = a;` 之后 | 原因 |
|---|---|---|
| `i32`、`bool`、`char`、`(i32, f64)` | `a` 和 `b` 都能用 | 实现了 `Copy`，按位复制 |
| `String`、`Vec`、`Box` | `a` **不能再用** | 发生了 move，所有权转移 |
| 想保留两份 | `a.clone()` | 显式深拷贝，有性能代价 |

`String` 由三部分组成：**指针 + 长度 + 容量**，数据在堆上。赋值只复制这三元组，如果两个变量都以为自己该释放堆内存，就会 double free。Rust 的解法是：把 `a` 标记为失效。

### 借用规则（记住这一句就够）

> **同一作用域内，要么有任意多个 `&T`，要么有唯一一个 `&mut T`，二者不能共存。**

```rust
let mut s = String::from("hi");
let r = &s;
s.push('!');   // ❌ 因为 r 还活着，不能可变借用
println!("{}", r);
```

这条规则直接消灭了数据竞争：读的时候没人能写，写的时候没人能读。

### 引用 vs 所有权

| 写法 | 含义 | 函数结束后原值 |
|---|---|---|
| `fn f(s: String)` | 拿走所有权 | 被释放，外面不能再用 |
| `fn f(s: &String)` | 只读借用 | 还在 |
| `fn f(s: &mut String)` | 可写借用 | 还在，可能被改过 |

**经验法则**：函数参数能用 `&str` 就别用 `&String`，能用引用就别传所有权。

### 切片是视图

```rust
let s = String::from("hello world");
let w = &s[6..11];   // 只是指向原数据的两个数字：起始位置 + 长度
```

切片的代价是几乎为零（不复制），代价是**原值在切片存活期间不能被修改**。

> 字符串切片的下标是**字节**偏移，必须落在字符边界上。中文一个字 3 字节，`&s[0..1]` 会在运行时 panic。

## 你会遇到的两类报错

```
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:20:20
   |
14 |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`
15 |     let s2 = s1;
   |              -- value moved here
20 |     println!("{}", s1);
   |                    ^^ value borrowed here after move
```

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

读这两段报错的技巧：**看 `-->` 指向的行，再看 `help:` 段落**，编译器通常直接告诉你"考虑用 clone"或"把借用作用域缩小"。

## 预期输出（节选）

```
x=5 y=5（i32 是 Copy，两者都能用）
s2 = hello（s1 已经被移动走了，不能用）
「借用示例」的长度是 12，s 仍然可用
first = 1
v = [1, 2, 3, 4]
hello / world
第一个单词: hello
较长的那个: abcd
```

## 试着改改

1. 把第 2 节被注释的 `println!("{}", s1);` 放开，读一遍 E0382 报错。
2. 把 `let m = &mut s2;` 之后的 `println!("{}", s2)` 放开，读 E0502。
3. 把 `calculate_length` 的参数改成 `&str`，调用处改成 `calculate_length(&s)`，确认也能编译（更通用）。
4. 把 NLL 那段里的 `println!("first = {}", first);` 挪到 `v.push(4)` 之后，看看会发生什么。
5. 写一个 `fn last_word(s: &str) -> &str`，返回最后一个空格之后的部分。

## 常见坑

- 以为 `let s2 = s1` 是复制 → 其实 String 是移动。
- 循环里 `for x in vec` 会把 `vec` 移走，之后就不能用了；要保留就用 `&vec` 或 `vec.iter()`。
- 函数返回局部变量的引用 → 编译不过（值已经被释放），要么返回所有权，要么参数里传进来。
- `&s[0..n]` 里 n 用字符数算 → 中文会 panic，要用字节边界。
