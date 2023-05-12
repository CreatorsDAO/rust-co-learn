//! 闭包
//!

/**

```
    // 1 闭包定义

    // 闭包可以捕获环境变量,并且根据其对环境变量的操作可以分为以下三类

    let c1 = || print!("未捕获环境变量");

    let v = "rust";
    let c2 = || print!("捕获环境变量但不修改 {}", v);

    let mut s0 = String::from("hello");

    // 闭包的参数写在 ｜｜ 中

    let mut c3 = |s: String| {
        s0 = s + v;
        println!("捕获并修改环境变量 {:?}", s0)
    };

    // 2 闭包的调用

    // 闭包的调用同函数一样

    c1();
    c2();
    c3(String::from("rust"));

```
*/

pub fn closure() {
    println!("");
}
