//! 2.3 生命周期参数
//!

/**

```

    // 可以推断引用的有效性
    // fn return_strings(x: &String) -> &String {
    //     let s = String::from("Rust");

    //     &s
    // }

    let s_p = return_string(&String::from("Rust"));

    fn return_string(x: &String) -> &String {
        x
    }

    struct Foo {
        x: i32,
        y: (i32, bool),
        z: String,
    }

    let f1 = Foo {
        x: 32,
        y: (32, true),
        z: String::from("rust"),
    };

    let f2 = Foo {
        x: 32,
        y: (32, true),
        z: String::from("rust"),
    };

    // 仍然无法编译通过，因为编译器无法推断出参数和返回值的生命周期
    // 这是因为Rust对于函数的检查只会检查签名，而不是函数里面的具体逻辑
    // 对于这个函数返回的引用，因为只从函数签名来看，返回的这个引用不知道是来自于哪里，一个可能是来自于两个参数的某一个，另一种情况是可能来自于某个内部定义的变量，这就会导致悬垂指针

    // 无法推断引用的有效性
    // fn bar(x: &Foo, y: &Foo) -> &Foo {
    //     x
    // }

    // 通过生命周期参数显示指定参数的生命周期

    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用

    let s: &'static str = "I have a static lifetime.";

    // 返回变量的生命周期和至少和y一样长
    fn bar<'a, 'b>(x: &'a Foo, y: &'b Foo) -> &'b Foo {
        y
    }

    fn bar1<'b>(x: &Foo, y: &'b Foo) -> &'b Foo {
        y
    }

    // 'a:'b,表示'a不短于'b, 只能返回x
    fn bar2<'a: 'b, 'b>(x: &'a Foo, y: &'b Foo) -> &'a Foo {
        x
        // y
    }

    // x，y 都可以
    fn bar3<'a: 'b, 'b>(x: &'a Foo, y: &'b Foo) -> &'b Foo {
        // x
        y
    }

    // 调用

    let f1 = Foo {
        x: 32,
        y: (32, true),
        z: String::from("rust"),
    };
    {
        let f2 = Foo {
            x: 32,
            y: (32, true),
            z: String::from("rust"),
        };

        bar(&f1, &f2);
        bar1(&f1, &f2);
        bar2(&f1, &f2);
        bar3(&f1, &f2);
    }


```
*/

pub fn lifetime_params() {
    println!("");
}
