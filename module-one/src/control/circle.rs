//! 循环
//!

/**

```

    // 1 使用for循环遍历集合
    // 注意：Rust中的for循环本质上是迭代器的语法糖
    // 迭代器涉及到更复杂的知识点，后续会介绍

    let v = vec![1, 2, 3, 4, 5];

    for num in v {
        println!("{}", num);
    }

    let mut map = std::collections::HashMap::new();
    map.insert("a", 2);
    map.insert("b", 2);
    map.insert("c", 2);

    for kv_pair in map {
        println!("{:?}", kv_pair);
    }

    // 2 使用 loop 执行无限循环，并使用break终止

    let mut x = 0;

    loop {
        println!("{:?}", x);

        if x == 10 {
            break;
        } else {
            x = x + 1;
        }
    }

    // 3 使用 while 执行条件循环

    let mut x = 0;
    while x < 5 {
        println!("x is {}", x);
        x += 1;
    }


```
*/

pub fn circle() {
    println!("");
}
