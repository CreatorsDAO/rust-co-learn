//! 1.1 trait与闭包
//!
//!

/**

```
// 1. 回顾三种类型的闭包
// 前面我们介绍过，闭包有三种类型：未捕获环境变量，捕获环境变量不修改，捕获环境变量并修改

// 1.1 未捕获环境变量
let c1 = || println!("didn't catch env var");
c1();

// 1.2 捕获但不修改环境变量
let x = 10;

let c2 = || println!("catch env var but not modify, x = {}", x);

c2();

// 1.3 捕获并修改环境变量

let mut x = 10;
let mut c3 = |a: i32| {
    x = 1 + a;
    println!("catch env var and modify, x = {}", x);
};
c3(10);

// 2. 闭包实现与trait

// 在Rust中，闭包实际上是一个语法糖，它的实现在抽象概念上可以看做是一个匿名结构体，这个结构体会把环境变量捕获成为其成员，并实现Fn/FnMut/FnOnce trait
// Fn/FnMut/FnOnce中各有一个方法分别是call/call_mut/call_once，对应的语义分别是调用不可变闭包、调用可变闭包、调用消费闭包
// 并且Fn/FnMut/FnOnce trait是以次继承的，也就是说实现 Fn trait,必须实现 FnMut trait，实现 FnMut trait,必须实现 FnOnce trait

// 当声明一个闭包时，编译器会根据闭包的类型，自动推导出其实现的trait，一般情况下不需要手动实现

// 3. 闭包作为函数参数传递
// 值得注意的是，在将闭包作为参数在函数中传递时，类型的指定是通过trait来实现的

fn call_fn<F: Fn()>(f: F) {
    f();
}

fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
}

fn call_fn_once<F: FnOnce()>(f: F) {
    f();
}

// 闭包的调用 case 1
// Rust编译器会根据你如何调用推导出闭包的类型，也就是实现哪个trait

let c = || println!("closure");

call_fn_once(c); // 实现了FnOnce trait
call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
call_fn_mut(c); // 实现了FnMut trait,FnOnce trait

// 闭包的调用 case 2

let x = "10";

let c = || println!("get env var {}", x);

call_fn_once(c); // 实现了FnOnce trait
call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
call_fn_mut(c); // 实现了FnMut trait,FnOnce trait

// 闭包的调用 case 3

let mut x = String::from("10");

let mut c = || println!("get env var {x:?}", x = String::from("20"));

call_fn_once(c); // 实现了FnOnce trait
call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
call_fn_mut(c); // 实现了FnMut trait,FnOnce trait

// 4. 闭包作为函数返回

fn return_fn() -> impl Fn() {
    || println!("call_fn")
}

fn return_i32(i: i32) -> i32 {
    32
}

fn return_fn_mut() -> impl FnMut() {
    let x = 10;
    // || println!("call_fn_mut {}", x + 1) // 不能返回局部变量
    move || println!("call_fn_mut {}", x + 1) // 必须把局部变量移入闭包，才能返回（这里实际上发生了数据的复制）
}

fn return_fn_once() -> impl FnOnce() {
    let s = String::from("hello");
    // || println!("call_fn_once {:?}", s)
    move || println!("call_fn_once {:?}", s) // 必须把局部变量移入闭包，才能返回（这里实际上发生了所有权转移）
}
```
*/

pub fn trait_closure() {
    println!("");
}
