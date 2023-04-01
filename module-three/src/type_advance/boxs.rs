//! 3.2.1 `Box<T>`
//!

/**

```
    // 1 Box<T> 与数据分配

    // 在Rust中，你可以使用Box将数据强行存储到堆上

    let a = Box::new("rust");
    let b = Box::new(42);

    // 它也是唯一可以将数据放到堆上的途径

    // 2 Box<T> 是一个智能指针
    // 它实现了Deref和Drop trait

    let s = Box::new("rust");
    let s = *s; // 解引用

    // 离开作用域时，会自动调用drop方法，释放堆上的数据

    // 这个类型比较简单，再次需要强调的是它是众多的Rust基于结构体构和trait造的特殊类型之一

```
*/

pub fn boxs() {
    println!("");
}
