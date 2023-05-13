//! 3.4 trait 对象
//!
//!

/**

```
    // 1 trait类型

    // 1.1 空trait

    trait A {}

    // 1.2 有方法的trait

    trait B {
        fn method1(&self);
        fn method2(&self);

        // ...
    }

    // 1.3 有关联类型的trait

    trait C {
        type Type;

        fn method1(&self) -> Self::Type;
    }

    // 1.4 有默认实现的trait

    trait D {
        // 这个方法是默认实现
        fn method1(&self) {
            println!("method1");
        }
        fn method2(&self);
    }

    // 2 如何实现 trait

    // 2.1 手动实现

    struct Book;

    trait Read {
        fn read(&self);
    }

    // 使用impl语法
    impl Read for Book {
        fn read(&self) {
            println!("read book");
        }
    }

    // 注意和为类型实现方法做区别

    impl Book {
        fn read(&self) {
            println!("read book");
        }
    }

    // 2.2 使用宏实现
    // 标准库和第三方库中一些trait可以通过派生宏来实现

    #[derive(Default, Clone)]
    struct Student {
        name: String,
        age: u32,
    }

    // 可以直接调用trait提供的方法
    let s = Student::default();
    let s1 = s.clone();

    // 3 trait约束

    // 3.1 trait继承，如下要求类型必须先实现 Clone和Default trait才能是实现 S trait
    trait S: Clone + Default {
        fn get_age(&self) -> u32;
    }

    impl S for Student {
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    // trait 作为函数参数的约束：只有实现了S trait的泛型才能作为下列函数的参数

    fn person_age<T: S>(s: T) -> u32 {
        s.get_age()
    }

    struct Teacher {
        name: String,
        age: u32,
    }

    let t = Teacher {
        name: "teacher".to_string(),
        age: 30,
    };

    // person_age(t); // t没有实现S trait，所以不能作为参数
    person_age(s); // 可以调用

```
*/

pub fn trait_intro() {
    println!("");
}
