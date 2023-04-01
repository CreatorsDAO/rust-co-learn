use std::fmt::Debug;

/// 1. 文档注释,一般写在当前文件的最顶端

fn main() {
    // 1 trait object

    // trait object 用在当你想返回一个实现了某个trait的类型
    // 语法：&dyn Trait or Box<dyn Trait> // Box是Rust中唯一可以把数据强制分配到堆上的类型，先不展开，后面会介绍

    trait Animal {
        fn speak(&self) -> &'static str;
    }

    struct Dog;
    impl Animal for Dog {
        fn speak(&self) -> &'static str {
            "Woof!"
        }
    }

    struct Cat;
    impl Animal for Cat {
        fn speak(&self) -> &'static str {
            "Meow!"
        }
    }

    fn animal_speak(animal: &dyn Animal) {
        println!("{}", animal.speak());
    }

    fn main() {
        let dog = Dog;
        let cat = Cat;

        animal_speak(&dog);
        animal_speak(&cat);
    }

    // 特别说名，使用 trait 对象 会带来运行时开销
    // 因为在编译时无法确定具体类型，所以编译器需要在运行时动态地查找并调用正确的方法
    // 这涉及到虚函数表（vtable）的概念，每个 trait 对象都有一个指向相应 vtable 的指针
}
