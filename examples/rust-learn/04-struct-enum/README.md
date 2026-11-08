# 04 结构体与枚举

## 这一节要会什么

- 定义 `struct`、创建实例、字段简写、`..` 更新语法
- `impl` 里三种方法：`&self`、`&mut self`、`self`
- 元组结构体，以及自己实现 `Display` / 派生 `Debug`
- `enum` 携带数据 + `match` 穷尽匹配
- `Option<T>` 为什么能取代 `null`

## 运行

```bash
./run.sh 04
```

## 关键概念

### 1. 三种 self

| 写法 | 含义 | 调用后 |
|---|---|---|
| `fn describe(&self)` | 只读借用 | 原值还在 |
| `fn birthday(&mut self)` | 可写借用 | 原值被改 |
| `fn into_name(self)` | 拿走所有权 | 原变量失效 |

关联函数（没有 `self`）用 `类型::函数()` 调用，比如 `User::new(..)`，常用来做构造器。Rust 没有构造函数语法，`new` 只是约定俗成的名字。

### 2. Debug 与 Display

| | 怎么来 | 怎么打印 | 用途 |
|---|---|---|---|
| `Debug` | `#[derive(Debug)]` | `{:?}` / `{:#?}` | 开发期打印、调试 |
| `Display` | 手写 `impl fmt::Display` | `{}` | 给用户看的输出 |

`derive` 是"让编译器自动生成代码"的宏，常用组合：`#[derive(Debug, Clone, PartialEq)]`。

### 3. 结构体更新语法

```rust
let bob = User { name: "Bob".into(), ..alice };
```

`..` 会把剩下的字段从 `alice` **移动**过来。如果结构体里有 `String` 这类非 Copy 字段，`alice` 整体就不能再用了；想保留就 `..alice.clone()`。

### 4. 枚举比 C 强在哪

```rust
enum Message {
    Quit,                        // 无数据
    Move { x: i32, y: i32 },     // 命名字段
    Write(String),               // 元组字段
    ChangeColor(u8, u8, u8),     // 多个字段
}
```

每个变体可以带**不同类型、不同数量**的数据。配合 `match`，编译器保证你处理了所有情况——这就是 Rust 里"用类型表达状态"的基础。

### 5. Option 取代 null

```rust
enum Option<T> { Some(T), None }
```

- 没有 `null`，就没有空指针异常。
- 想取值必须**显式处理** `None`：`match`、`if let`、`unwrap_or`、`?`。
- `Option<&T>` 里的 `&T` 不能是空引用，所以 `Some(x)` 里的 `x` 一定有效。

`unwrap()` 在 `None` 时会 panic，只适合"我确信有值"的场景（比如测试）。

## 预期输出（节选）

```
Alice（17 岁，活跃）
过完生日: Alice（18 岁，活跃）
是否成年: true
(3, 4) 的平方距离 = 25
  圆 的面积 = 3.14
  矩形 的面积 = 6.00
  三角形 的面积 = 10.00
  改色: #FF0000
找 Alice: Some("Alice")
找 Zoe:   None
```

## 试着改改

1. 给 `User` 加一个 `email: Option<String>` 字段，`new` 里传 `None`，再写个 `fn email_or_default(&self) -> &str`。
2. 给 `Shape` 加一个 `Square(f64)` 变体，看看编译器会在哪几处报"非穷尽匹配"。
3. 把 `find_user` 返回值改成 `Option<User>`（克隆一份），体会返回引用和返回所有权的区别。
4. 给 `User` 实现 `Display`，然后用 `{}` 打印它。

## 常见坑

- `#[derive(Debug)]` 忘了加，`{:?}` 直接编译不过。
- 方法名和字段名重名没问题，但 `self.name` 和局部变量 `name` 要分清。
- `..alice` 之后还用 `alice`，会报 "use of moved value"。
- `match` 里用 `_ => {}` 兜底，等于放弃了编译器的穷尽检查，加变体时不会提醒你。
