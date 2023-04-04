//! 字符串
//!

/**

```
    /// 1 &str
    /// 字符串字面量实际上存放在程序的只读数据段中，在程序运行时会被加载到内存中读取
    let s = "Hello Rust";
    let mut s1 = "Hello Go";

    s1 = "Hello Rust";
    println!("{}", s1);

    /// 2 String
    /// String 是一个需要动态分配大小的，其大小在代码编译时是未知的，内部的容量是可以随时根据需要变化的。
    // 我们可以通过多种方式创建一个String
    // let s2 = String::new();         // An empty String
    // let s2 = "Hello Rust".to_string();
    // let s2: String = "Hello Rust".into();
    let s2 = String::from("Hello Rust");

    // String 由三部分组成：指向某些字节的指针、长度和容量。指针指向 String 于存储其数据的内部缓冲区。
    // 长度是缓冲区中当前存储的字节数，容量是缓冲区的大小（以字节为单位）。因此，长度将始终小于或等于容量。
    // 可以通过 as_ptr、len、capacity 方法访问这三个量。
    let ptr = s2.as_ptr(); // 返回String的指针
    let len = s2.len(); // 返回String的长度
    let cap = s2.capacity(); // 返回String的容量

    println!("pointer {:?}", ptr);
    println!("len {}", len);
    println!("cap {}", cap);

    /// 3 字符串切片
    /// 字符串本质上一个u8序列，支持切片操作
    let s1 = String::from("Hello Rust");
    let s2 = "Hello Rust";

    let slice1 = &s1[0..5];
    let slice2 = &s2[6..10];

    println!("slice1: {}", slice1); // Hello
    println!("slice2: {}", slice2); // Rust

```
*/

pub fn string() {
    println!("");
}
