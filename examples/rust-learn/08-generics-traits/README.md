# 08 泛型与 trait

## 这一节要会什么

- 泛型函数、泛型结构体、带约束的 `impl`
- 定义 trait、默认方法、为自定义类型实现 trait
- `impl Trait` 参数与返回值
- `Box<dyn Trait>` 动态分发，和泛型的区别
- `where` 子句

## 运行

```bash
./run.sh 08
```

## 关键概念

### 1. 泛型就是"类型参数"

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T
struct Point<T> { x: T, y: T }
enum Result<T, E> { Ok(T), Err(E) }
```

`<T>` 里的 T 是占位符，调用时才确定。约束（`PartialOrd + Copy`）告诉编译器"T 至少要会这些"。

**Rust 泛型是零成本抽象**：编译期为每个具体类型生成一份代码（单态化），运行时和手写 `i32` 版本一样快。

### 2. 约束可以写在三个地方

```rust
fn f<T: Display>(x: T) { }              // 参数列表里
fn f<T>(x: T) where T: Display { }      // where 子句（约束多时更好读）
impl<T: Display> Point<T> { }           // impl 块上：只对这些 T 提供方法
```

### 3. trait = 接口 / 契约

```rust
trait Summary {
    fn summarize(&self) -> String;                  // 必须实现
    fn preview(&self) -> String {                    // 默认实现，可覆盖
        format!("[预览] {}", self.summarize())
    }
}
```

实现方：

```rust
impl Summary for Tweet {
    fn summarize(&self) -> String { ... }
}
```

trait 里可以放：必须方法、默认方法、关联类型、关联常量。**不能有字段**（字段属于结构体）。

### 4. 泛型 vs trait 对象

| | `fn f<T: Summary>(x: T)` | `fn f(x: &dyn Summary)` |
|---|---|---|
| 分发方式 | 编译期单态化 | 运行时虚表 |
| 性能 | 无额外开销 | 一次指针跳转 |
| 代码体积 | 每种类型一份 | 一份 |
| 能否混放不同类型 | ❌ | ✅ `Vec<Box<dyn Summary>>` |
| 对象安全要求 | 无 | 方法不能有泛型参数、不能返回 `Self` |

经验法则：**能静态就静态，需要异构集合才用 `dyn`**。

### 5. 常见标准库 trait

| trait | 作用 | 怎么获得 |
|---|---|---|
| `Debug` | `{:?}` 打印 | `#[derive(Debug)]` |
| `Display` | `{}` 打印 | 手写 `impl Display` |
| `Clone` | 深拷贝 | `#[derive(Clone)]` 或手写 |
| `Copy` | 按位复制（赋值不移动） | `#[derive(Copy, Clone)]` |
| `PartialEq` / `Eq` | `==` | `#[derive(PartialEq, Eq)]` |
| `PartialOrd` / `Ord` | `<` `>` 排序 | `#[derive(PartialOrd, Ord)]` |
| `Default` | 默认值 | `#[derive(Default)]` |
| `From` / `Into` | 类型转换 | 实现 `From` 自动获得 `Into` |
| `Iterator` | 迭代 | 实现 `next()` |

### 6. 孤儿规则

你只能为**自己的类型**实现**外部的 trait**，或者为自己的类型实现自己的 trait：

```rust
impl Display for MyType { }   // ✅ 自己的类型 + 标准库 trait
impl MyTrait for String { }   // ✅ 外部类型 + 自己的 trait
impl Display for String { }   // ❌ 两个都是外部的
```

## 预期输出（节选）

```
最大整数: 9
最大浮点: 7.5
int_point = (1, 2)
最大值是 8
文章摘要: Rust 1.94 发布 —— 张三
推文摘要: @rustlang: 今天也在写 Rust
文章预览: [预览] Rust 1.94 发布 —— 张三
  收到通知: Rust 1.94 发布 —— 张三
  收到通知(泛型): @rustlang: 今天也在写 Rust
工厂返回: @demo: 返回了一个实现了 Summary 的值
  [遍历] Rust 1.94 发布 —— 张三
  [遍历] @rustlang: 今天也在写 Rust
a=42 b=hello
a=2.5 b=x
```

## 试着改改

1. 给 `Tweet` 也实现 `Display`，然后试试 `impl<T: Display> Pair<T>` 里加更多方法。
2. 定义 `trait Area { fn area(&self) -> f64; }`，为圆形、矩形实现，放进 `Vec<Box<dyn Area>>`。
3. 把 `largest` 改成返回 `Option<&T>`（空切片时不 panic）。
4. 给 `Summary` 加一个默认方法 `fn is_long(&self) -> bool { self.summarize().len() > 20 }`。
5. 用 `impl Trait` 写一个返回迭代器的函数，对比 `-> Vec<T>` 的区别。

## 常见坑

- `Point<i32>` 和 `Point<f64>` 是不同类型，不能放进同一个 `Vec`（除非用 enum 或 `Box<dyn>`）。
- 忘了约束：`largest` 里用 `>` 就必须 `T: PartialOrd`，否则报"cannot compare"。
- `Box<dyn Summary>` 里的方法如果有泛型参数，就不是对象安全的，编译不过。
- 默认方法里调用 `self.summarize()` 没问题，但默认方法不能访问结构体字段。
