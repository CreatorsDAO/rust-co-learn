//! 3.1 trait
//!
//!

/**

```
    // 1 trait 种类

    // 1.1 空trait

    trait A {}

    // 1.2 有方法的trait

    trait B {
        fn method(&self);
        fn method2(&self);

        // ...
    }

    // 1.3 有关联类型的trait

    trait C {
        type T;

        fn method1(&self) -> Self::T;
    }

    // 1.4 有默认实现的trait

    trait D {
        // 这个方法是默认实现
        fn method1(&self) {
            println!("method1");
        }
        fn consume_method(&mut self);
    }

    // 1.5 有自由方法（函数）的trait

    trait E {
        // 这个方法是默认实现
        fn method1(&self) {
            println!("method1");
        }
        // 这个方法需要手动实现
        fn method2(&self);

        // 这个方法是默认实现
        fn method3() {
            println!("freedom method")
        }

        // 这个方法需要手动实现
        fn method4(a: &str) -> &str;
    }

    // 1.6 trait继承

    trait F: E {
        // method
    }

    // 2 如何实现 trait

    // 2.1 手动实现

    struct Teacher;

    impl Teacher {
        fn method1() {
            print!("这是类型的关联方法");
        }
    }

    Teacher::method1(); // 关联方法调用

    impl A for Teacher {}

    impl B for Teacher {
        fn method(&self) {
            print!("")
        }
        fn method2(&self) {
            print!("")
        }
    }

    let mut t = Teacher;
    t.method(); // 方法通过实例调用
    t.method();

    impl C for Teacher {
        type T = Teacher;

        fn method1(&self) -> Self::T {
            let t = String::from("Teacher");

            // t
            Teacher
        }
    }

    impl D for Teacher {
        fn consume_method(&mut self) {
            // let x = self;
            // let y = self;
        }
    }

    t.consume_method();
    t.consume_method();

    impl E for Teacher {
        fn method2(&self) {}
        fn method4(a: &str) -> &str {
            "Rust"
        }
    }

    Teacher::method4("Go"); // 对trait中自由方法的调用同调用类型的关联方法

    struct Professor;

    // impl F for Professor {}

    impl F for Teacher {}

    // 2.2 使用宏实现
    // 标准库和第三方库中一些trait可以通过派生宏来实现

    #[derive(Default, Clone)]
    struct Student {
        name: String,
        age: u32,
    }

    // 调用方法

    // 可以直接调用trait提供的方法
    let s = Student::default();
    let s1 = s.clone();

```
*/

pub fn trait_intro() {
    println!("");
}
