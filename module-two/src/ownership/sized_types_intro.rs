//! 1.1 固定大小类型介绍
//!

/**

```

    let mut a: i32 = 32;

    // 获取 'a' 变量的大小（改变前）
    let size_before_a = std::mem::size_of_val(&a);
    println!("Size of 'a' before change: {} bytes", size_before_a); // 打印结果：Size of 'a' before change: 4 bytes

    // 打印 'a' 变量的地址（改变前）
    let address_before_a = &a as *const i32;
    println!("Address of 'a' before change: {:?}", address_before_a);

    a = 64;

    // 获取 'a' 变量的大小（改变后）
    let size_after_a = std::mem::size_of_val(&a);
    println!("Size of 'a' after change: {} bytes", size_after_a); // 打印结果：Size of 'a' after change: 4 bytes

    // 打印 'a' 变量的地址（改变后）
    let address_after_a = &a as *const i32;
    println!("Address of 'a' after change: {:?}", address_after_a);

    let mut t = ('a', 32, true, 42.1);

    // 获取 't' 变量的大小（改变前）
    let size_before_t = std::mem::size_of_val(&t);
    println!("Size of 't' before change: {} bytes", size_before_t); // 打印结果：Size of 't' before change: 24 bytes

    // 打印 't' 变量的地址（改变前）
    let address_before_t = &t as *const (_, _, _, _);
    println!("Address of 't' before change: {:?}", address_before_t);

    t = ('b', 64, false, 84.2);

    // 获取 't' 变量的大小（改变后）
    let size_after_t = std::mem::size_of_val(&t);
    println!("Size of 't' after change: {} bytes", size_after_t); // 打印结果：Size of 't' after change: 24 bytes

    // 打印 't' 变量的地址（改变后）
    let address_after_t = &t as *const (_, _, _, _);
    println!("Address of 't' after change: {:?}", address_after_t);

```
*/

pub fn sized_types_intro() {
    println!("");
}
