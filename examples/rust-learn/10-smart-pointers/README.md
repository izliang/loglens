# 10 智能指针

## 这一节要会什么

- `Box<T>`：堆分配、递归类型、`Box<dyn Error>`
- `Deref`：为什么 `*y` 能用，什么是解引用强制转换
- `Drop`：析构顺序、`drop()` 的用法
- `Rc<T>`：引用计数、多所有者
- `RefCell<T>`：内部可变性、运行时借用检查
- `Rc<RefCell<T>>` 组合

## 运行

```bash
./run.sh 10
```

## 关键概念

### 1. 智能指针 vs 普通引用

普通引用 `&T` 只是"借用"，不拥有数据。智能指针**拥有**数据，并在 `Drop` 时负责释放：

| 类型 | 所有权 | 线程安全 | 可变性 | 运行时开销 |
|---|---|---|---|---|
| `Box<T>` | 独占 | ✅ | 需要 `mut` | 一次堆分配 |
| `Rc<T>` | 共享 | ❌ | 只读 | 计数 +1/-1 |
| `Arc<T>` | 共享 | ✅ | 只读 | 原子计数 |
| `RefCell<T>` | 独占 | ❌ | 运行时检查 | 借用标志位 |
| `Mutex<T>` | 共享 | ✅ | 加锁 | 系统锁 |
| `Cell<T>` | 独占 | ❌ | `Copy` 类型直接替换 | 无 |

### 2. Box 解决的两个问题

```rust
// ① 递归类型：大小必须编译期确定
enum List { Cons(i32, Box<List>), Nil }

// ② trait 对象：大小不确定
let e: Box<dyn Error> = ...;
```

### 3. Deref 与解引用强制转换

实现 `Deref` 后，`MyBox<T>` 就能：

- 用 `*y` 取值
- 在需要 `&T` 的地方自动转换（deref coercion）

```rust
fn hello(name: &str) { }
hello(&MyBox::new(String::from("Rust")));
// &MyBox<String> → &String → &str，编译器自动连续解引用
```

### 4. Rc 的规则

- `Rc::clone(&a)` **不复制数据**，只把计数 +1（名字叫 clone 容易误会）。
- 最后一个 `Rc` 被 drop 时，数据才真正释放。
- `Rc<T>` 只提供 `&T`，**不能直接改内容**；要改就套 `RefCell`。
- `Rc` 不是线程安全的，跨线程用 `Arc`。

### 5. RefCell 的规则

```rust
let cell = RefCell::new(5);
let a = cell.borrow();       // 不可变借用，可多个
let b = cell.borrow_mut();   // 可变借用：此时若 a 还活着 → panic!
```

- 编译期检查 → 运行期检查，违反规则从"编译错误"变成"panic"。
- 不想 panic 就用 `try_borrow()` / `try_borrow_mut()`，返回 `Result`。
- 适合"编译器看不出来但逻辑上安全"的场景（如图结构、缓存）。

### 6. 循环引用与 Weak

`Rc` 互相指向会形成环，计数永远不归零 → 内存泄漏。用 `Weak<T>` 打破环：

```rust
let weak = Rc::downgrade(&rc);   // 不增加强引用计数
weak.upgrade()                    // 返回 Option<Rc<T>>，数据已释放则 None
```

### 7. 什么时候用哪个

```
需要堆分配 / 递归类型 / trait 对象      → Box
单线程多方共享只读                      → Rc
单线程共享且要修改                      → Rc<RefCell<T>>
多线程共享只读                          → Arc
多线程共享且要修改                      → Arc<Mutex<T>> 或 Arc<RwLock<T>>
```

## 预期输出（节选）

```
列表: Cons(1, Cons(2, Cons(3, Nil)))
boxed_number = 5，解引用 = 5
*y = 5
  hello, Rust!
  a、b 已创建，即将离开作用域
  释放 b
  释放 a
  作用域结束
  释放 c
初始引用计数: 1
克隆两次后: 3
drop 一个后: 2
修改后: [1, 2, 3, 4]
第二次可变借用成功吗: false
释放后可以再借: true
两个句柄各加一次: 2
```

## 试着改改

1. 用 `Box` 实现一个链表，写 `fn sum(list: &List) -> i32` 递归求和。
2. 用 `Rc<RefCell<i32>>` 写一个"计数器"，让两个函数各自加一。
3. 把 `Noisy` 放进 `Vec`，观察 drop 顺序（Vec 是后进先出）。
4. 用 `Weak` 实现父子节点，避免循环引用导致的内存泄漏。
5. 给 `MyBox` 实现 `DerefMut`，让 `*y = 10` 也能用。

## 常见坑

- `Rc::clone` 以为会深拷贝 → 只是计数 +1。
- `Rc<RefCell<T>>` 里 `borrow()` 没释放又 `borrow_mut()` → 运行时 panic。
- 忘记 `use std::rc::Rc` 或 `use std::cell::RefCell`。
- `dyn Trait` 直接用会报"size unknown"→ 套 `Box`/`&`。
- `Rc` 用在多线程里 → 编译错误（没有实现 `Send`），换 `Arc`。
