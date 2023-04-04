//! 复合类型
//!

/**

```

    // 1 元组
    // Rust中的元组可以将各种类型组合起来
    let types = (42, "Rust", true);

    // 可以像访问数组元素一样通过下标索引访问
    println!("num is {}", types.0);

    // 单元类型 （）
    // 单元类型在Rust中是非常重要的类型，如果表达式不返回任何其他值，就隐式地返回单元值，
    // 如没有返回值的函数或者作用域
    let a: () = {};
    fn return_tuple() {}
    let func: () = return_tuple();
    assert_eq!(a, func);

    // 2 数组
    // 通过索引来访问或者修改数组中的元素
    let arr = [1, 2, 3, 4, 5];

    let mut arr1 = [0, 0, 0, 0, 0];
    arr1[0] = 100;
    println!("{:?}", arr1); // [100, 0, 0, 0, 0]

```
*/

pub fn compound() {
    println!("");
}
