//! 3.4 trait 与所有权
//!
//!

/**

```
    // 1 Copy trait 和 Clone trait

    // 之前我们介绍了所有的固定尺寸类型，当把一个变量赋值给另一个变量时，会发生值的复制

    // owner_one 和 owner_two 现在各自拥有一份值，数据发生了拷贝
    let owner_one = 32;
    let owner_two = owner_one;

    // 但是对于一些动态尺寸大小的类型，比如str和[T],我们在使用它们的指针 String和Vec<T>时，不会发生值的复制，而是会移交所有权

    let owner_one = String::from("hello");
    let owner_two = owner_one;

    // println!("{:?}", owner_one); // 不可通过owner_one访问数据，因为它已经移交了所有权

    // 从trait的角度来讲，就是所有固定尺寸类型都实现了 Copy 和 Clone trait，而动态尺寸类型都没有实现 Copy trait，但大多都实现了Clone trait
    // 并且编译器报错也会告诉你，哪些类型没有实现 Copy trait

    // 如果你想在堆上复制想像使用固定尺寸类型那样一样在堆上复制一份数据，你可以显式调用Clone trait中的 clone方法来实现这一点

    let v = vec![1, 2, 3, 4, 5];

    let v1 = v.clone();
    let v2 = v.clone();
    let v3 = v1.clone();

    // 新变量的地址和原变量的地址各不相同

    println!("{:p}", v.as_ptr()); // 0x7fccb3705b30
    println!("{:p}", v1.as_ptr()); // 0x7fccb3705b50
    println!("{:p}", v2.as_ptr()); // 0x7fccb3705b70
    println!("{:p}", v3.as_ptr()); // 0x7fccb3705b90

    // 2 trait实现与所有权
    // 在自定义 trait中的方法时，你可以根据需要选择要获取类型的不可变引用、可变引用或者所有权

    trait A {
        // 需要手动实现，获取所有权
        fn take_ownership(self);

        // 默认实现，获取不可变引用
        fn take_ref(&self) {
            println!("这个方法获取了类型的不可变引用")
        }

        // 默认实现，获取可变引用
        fn take_mut(&mut self) {
            println!("这个方法获取了类型的可变引用")
        }
    }

    struct X;

    impl A for X {
        fn take_ownership(self) {
            println!("这个方法获取了类型的所有权")
        }

        // 默认实现不用手动实现
    }

    let x = X;

    x.take_ownership(); // 这个方法获取了类型的所有权
                        // x.take_ref();// 不能再使用x,因为上述方法已经获取了所有权

    let mut y = X;
    y.take_ref(); // 这个方法获取了类型的不可变引用
    y.take_mut(); // 这个方法获取了类型的可变引用

    // 特别说明：所有权机制和trait本质上是Rust中两个独立的概念，即使没有trait，所有权机制也是成立的（这也是我们在介绍所有权机制时为什么没有提及trait，因为不需要）
    // 但trait系统让所有权机制更加的显式化了，更好理解，也更好使用
```
*/

pub fn trait_ownership() {
    println!("");
}
