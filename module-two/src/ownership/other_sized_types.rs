//! 1.2 其他固定大小类型
//!

/**

```

   use std::mem::size_of;


    // 整数类型
    let int_var: i32 = 10;
    println!("Size of integer: {}", size_of::<i32>());  // 打印结果：Size of integer: 4

    // 浮点数类型
    let float_var: f64 = 10.0;
    println!("Size of float: {}", size_of::<f64>());  // 打印结果：Size of float: 8

    // 布尔类型
    let bool_var: bool = true;
    println!("Size of bool: {}", size_of::<bool>());  // 打印结果：Size of bool: 1

    // 字符类型
    let char_var: char = 'a';
    println!("Size of char: {}", size_of::<char>());  // 打印结果：Size of char: 4

    // 数组类型
    let array_var: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Size of array: {}", size_of::<[i32; 5]>());  // 打印结果：Size of array: 20

    // 元组类型
    let tuple_var: (i32, f64, bool, char) = (10, 10.0, true, 'a');
    println!("Size of tuple: {}", size_of::<(i32, f64, bool, char)>());  // 打印结果：Size of tuple: 24

    // 引用类型
    let ref_var: &i32 = &10;
    println!("Size of reference: {}", size_of::<&i32>());  // 打印结果：Size of reference: 8

    // 裸指针类型
    let raw_pointer_var: *const i32 = &10;
    println!("Size of raw pointer: {}", size_of::<*const i32>());  // 打印结果：Size of raw pointer: 8

    // 函数指针类型
    let fn_pointer_var: fn() = foo;
    println!("Size of function pointer: {}", size_of::<fn()>());  // 打印结果：Size of function pointer: 8


fn foo() {
    println!("This is a function.");
}

```
*/

pub fn other_sized_types() {
    println!("");
}
