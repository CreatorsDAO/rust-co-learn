//! 1.3.3.6 函数
//!

/**

```
     // 1 函数定义

    // 没有参数和返回值
    fn foo() {
        println!("foo")
    }

    // 有参数和返回值

    fn bar(s: &str) -> String {
        String::from(s)
    }

    // 参数类型必须显式声明，比如引用或者可变性

    fn foobar(mut s: &str) -> &str {
        s = "rust";
        s
    }

    // 2 函数调用

    foo();
    bar("Rust");
    foobar("go");

    // 3 函数作为参数

    fn a(f: fn() -> u32) -> u32 {
        let value = f();

        value
    }

    fn b() -> u32 {
        42
    }

    // 把函数作为参传给另一个函数

    a(b);

```
*/

pub fn func() {
    println!("");
}
