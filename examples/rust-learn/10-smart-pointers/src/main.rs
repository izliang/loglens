//! 10 智能指针
//!
//! Box<T>    —— 把值放到堆上，独占所有权
//! Rc<T>     —— 多个所有者（单线程），引用计数
//! RefCell<T>—— 内部可变性，把借用检查从编译期挪到运行期
//! 三者组合 Rc<RefCell<T>> 是单线程"共享可变状态"的经典写法。
//! 运行：./run.sh 10

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    println!("===== 1. Box：堆分配 + 递归类型 =====");
    // 递归 enum 如果不加 Box，大小无法确定，编译不过
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("列表: {:?}", list);
    println!("列表求和: {}", sum(&list));

    let boxed_number = Box::new(5);
    println!(
        "boxed_number = {}，解引用 = {}",
        boxed_number, *boxed_number
    );

    // Box 最常用的场景之一：把错误装起来返回
    let err: Box<dyn std::error::Error> = "出错啦".into();
    println!("Box<dyn Error>: {}", err);

    println!("\n===== 2. Deref：自己实现解引用 =====");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y); // *y 等价于 *(y.deref())
    println!("*y = {}", *y);

    // 解引用强制转换：&MyBox<String> 能自动变成 &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 不需要写 &(*m)[..]

    println!("\n===== 3. Drop：离开作用域自动清理 =====");
    {
        let _a = Noisy { name: "a" };
        let _b = Noisy { name: "b" };
        println!("  a、b 已创建，即将离开作用域");
    } // 逆序释放：b 先，a 后
    println!("  作用域结束");

    // 手动提前释放：std::mem::drop 拿走所有权，函数结束时触发 Drop
    let c = Noisy { name: "c" };
    drop(c);
    println!("  c 已手动释放");

    println!("\n===== 4. Rc：多个所有者 =====");
    let shared = Rc::new(String::from("共享数据"));
    println!("初始引用计数: {}", Rc::strong_count(&shared));

    let clone1 = Rc::clone(&shared); // 只增加计数，不复制字符串
    let clone2 = Rc::clone(&shared);
    println!("克隆两次后: {}", Rc::strong_count(&shared));
    println!("clone1 = {}，clone2 = {}", clone1, clone2);

    drop(clone2);
    println!("drop 一个后: {}", Rc::strong_count(&shared));

    // Rc 是只读共享：想改内容需要配合 RefCell

    println!("\n===== 5. RefCell：内部可变性 =====");
    let cell = RefCell::new(vec![1, 2, 3]);
    // 即使 cell 本身不是 mut，也能通过 borrow_mut 修改内部
    cell.borrow_mut().push(4);
    println!("修改后: {:?}", cell.borrow());

    // 借用规则在运行时检查：同时两个可变借用会 panic
    let first = cell.borrow_mut();
    let second = cell.try_borrow_mut(); // 用 try_ 版本避免直接 panic
    println!("第二次可变借用成功吗: {}", second.is_ok());
    drop(first); // 释放第一个借用
    println!("释放后可以再借: {}", cell.try_borrow_mut().is_ok());

    println!("\n===== 6. Rc<RefCell<T>>：共享且可修改 =====");
    let counter = Rc::new(RefCell::new(0));
    let handles = vec![Rc::clone(&counter), Rc::clone(&counter)];

    for h in &handles {
        *h.borrow_mut() += 1;
    }
    println!("两个句柄各加一次: {}", counter.borrow());
    println!("当前引用计数: {}", Rc::strong_count(&counter));

    // 典型用途：图、双向链表、观察者模式、GUI 状态共享

    println!("\n===== 7. 组合结构：共享节点 =====");
    let node = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
    }));
    let tail = Rc::new(RefCell::new(Node {
        value: 2,
        next: None,
    }));
    node.borrow_mut().next = Some(Rc::clone(&tail));
    println!(
        "node = {}，next = {:?}",
        node.borrow().value,
        node.borrow().next.as_ref().map(|n| n.borrow().value)
    );
}

/// 递归类型：Box 让 List 的大小变成固定值（一个指针）
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// 递归求和：顺带演示如何"拆开" Box 取出里面的值
fn sum(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + sum(next),
        List::Nil => 0,
    }
}

/// 自定义智能指针：实现 Deref 就能用 * 和解引用强制转换
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// 观察 Drop 的调用时机和顺序
struct Noisy {
    name: &'static str,
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  释放 {}", self.name);
    }
}

/// 一个简单的单链表节点（共享所有权版本）
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

/// 接收 &str，传 &MyBox<String> 时编译器自动连续解引用
fn hello(name: &str) {
    println!("  hello, {}!", name);
}

// 对照表：
//   Box<T>         单所有者，堆分配，可存递归类型 / trait 对象
//   Rc<T>          多所有者，单线程，只读共享
//   Arc<T>         多所有者，跨线程（见 11 并发）
//   RefCell<T>     单线程内部可变性，运行时借用检查
//   Mutex<T>       多线程内部可变性，锁（见 11 并发）
//   Rc<RefCell<T>> 单线程共享可变状态
//   Arc<Mutex<T>>  多线程共享可变状态
