//! 1.2 trait与迭代器
//!
//!

/**

```
// 1. for 循环与迭代器

// 在rust中，for循环实际上的迭代器的语法糖

// for 循环以及解糖
let values = vec![1, 2, 3, 4, 5];
// 使用 for 循环遍历集合中个每个元素
for x in values {
    println!("{x}");
}
// for 循环解糖后等价如下：
let v = vec![1, 2, 3, 4, 5];
// 先将集合转为迭代器类型
let mut v_iter = v.into_iter();
// 在 loop 循环中使用next方法循环获取集合中下一个元素，当集合中取不到值时使用break终止 loop循环
loop {
    match v_iter.next() {
        Some(x) => println!("{}", x),
        None => break,
    }
}

// 2. 迭代器 trait IntoIterator 和 Iterator
// IntoIterator trait 中的 into_iter方法会返回一个 实现了 Iterator trait 迭代器
// Iterator trait 通过其 next方法来获取集合中的下一个元素

use std::collections::HashMap;
use std::slice::Iter;
use std::slice::IterMut;
use std::vec::IntoIter;


// 如果类型实现了迭代器 trait，则可以使用迭代器中的方法，例如：

let map = HashMap::from([("rust", 1), ("go", 2), ("python", 3)]);
let map_iter = map.into_iter();
let vec: Vec<(&str, i32)> = map_iter.collect();
println!("{:?}", vec); // [("rust", 1), ("go", 2), ("python", 3)]

// 3. 迭代器、借用和所有权
let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
let iter_mut: IterMut<i32> = v.iter_mut(); // 转为  IterMut 结构体, 可变借用
let iter: Iter<i32> = v.iter(); // 转为 Iter 结构体， 不可变借用
let iter_into: IntoIter<i32> = v.into_iter(); // 转为 IntoIter 结构体 ， 获取所有权

// 4. 迭代器适配器
let vec = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = vec
    .iter()
    .map(|&x| x * 3)
    .take(3)
    .filter(|x| *x > 6)
    .collect();
println!("{:?}", doubled); // [9]

// 5 迭代器与迭代器适配器特性：lazy（惰性）
let v = vec![1, 2, 3, 4, 5];
v.iter().for_each(|x| println!("{x}"));
// or
for x in &v {
    println!("{x}");
}

```
*/

pub fn trait_iterator() {
    println!("");
}
