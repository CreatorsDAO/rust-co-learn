//! 结构体
//!

/**

```
    // 1 结构体
    // Rust中的结构体有三种

    // 1.1 常规结构体
    struct Language {
        name: String,
        birth: u32,
        is_popular: bool,
    }
    // 1.2 元组结构体
    struct Rust(String);

    // 1.3 单元结构体
    struct Go;

    // 结构体赋值
    let l = Language {
        name: String::from("rust"),
        birth: 2010,
        is_popular: true,
    };

    // 访问结构体成员
    println!("{:?}", l.name);

    // 为结构体实现方法
    impl Rust {
        // Self 代表结构体本身
        fn new() -> Self {
            Rust(String::from("Rust"))
        }

        fn print() {
            let rust = Rust::new();
            println!("{:?}", rust.0)
        }
    }

    // 调用结构体实现的宏方法
    let r = Rust::new();
    Rust::print();

```
*/

pub fn structs() {
    println!("");
}
