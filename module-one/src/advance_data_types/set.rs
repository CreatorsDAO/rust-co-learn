//! 集合
//!

/**

```

    // 1 Vec
    // Vec是动态大小，相比起数组来说，它更加常用
    // Vec中的元素必须相同
    let mut vec1 = Vec::new();
    let mut vec2 = vec![];

    // vec 支持一系列的操作

    // 添加元素
    vec1.push("Rust");
    vec2.push("Go");

    // 当作栈
    vec1.pop();

    // 修改数据
    vec2[0] = "Rust";

    // Vec 和 String一样，数据存放在堆上

    println!("{}", vec2.len()); // 1
    println!("{}", vec2.capacity()); // 4
    println!("{:?}", vec2.as_ptr()); // 0x7fafc9f05b70

    // 2 HashMap
    // HashMap并不是预导入的，需要手动引入当前作用域
    use std::collections::HashMap;

    // 使用new方法创建
    let mut scores = HashMap::new();

    // 插入数据
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 修改数据,修改和插入数据是同一个api
    scores.insert(String::from("Blue"), 100);

    // 访问数据,注意访问的key传递的是引用
    let key = String::from("Blue");
    println!("{:?}", scores[&key])


```
*/

pub fn set() {
    println!("");
}
