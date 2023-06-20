//! 3.2 trait object
//!
//!

/**

```
    // 1 泛型与trait bound

    trait Animal {
        fn make_sound(&self) -> &'static str;
    }

    trait Food {}

    struct Dog;

    impl Animal for Dog {
        fn make_sound(&self) -> &'static str {
            "Woof!"
        }
    }

    struct Cat;

    impl Animal for Cat {
        fn make_sound(&self) -> &'static str {
            "Meow!"
        }
    }

    struct Pig;

    impl Animal for Pig {
        fn make_sound(&self) -> &'static str {
            "Woof!"
        }
    }

    impl Food for Pig {}

    // trait 作为约束时有三种写法

    fn get_weight<T: Animal + Food>(x: T) {

        // do sth
    }

    fn get_weight1(x: impl Animal + Food) {

        // do sth
    }

    fn get_weight2<T>(x: T)
    where
        T: Animal + Food,
    {
        // do sth
    }

    let d = Dog;
    let c = Cat;
    let p = Pig;

    // get_weight(d);
    // get_weight(c);
    get_weight(p);

    // 2 trait object
    // trait 对象通过指针来创建，如 & 或 Box<T>(一种智能指针，可以把数据存放到堆上)：&dyn Trait or Box<dyn Trait>
    // Box是Rust中唯一可以把数据强制分配到堆上的类型

    // 静态分发:在编译期通过具体类型实例直接调用方法,编译期单态化

    fn animal_make_sound<T: Animal>(a: T) {
        a.make_sound();
    }
    animal_make_sound(d);
    animal_make_sound(c);

    // 动态分发：在运行时先判断类型再查找类型对应方法
    // 特别说明，使用 trait object 会带来运行时开销

    fn animal_make_sound2(animals: Vec<&dyn Animal>) {
        for animal in animals {
            animal.make_sound();
        }
    }

    let d = Dog;
    let c = Cat;

    let animals: Vec<&dyn Animal> = vec![&d, &c];

    animal_make_sound2(animals);

    // 3 trait object 安全
    // trait中方法返回值类型不为 Self
    // trait中方法没有任何泛型类型参数

    pub trait X {
        fn method(&self) -> Self;
    }

    pub trait Y {
        fn print<T: std::fmt::Display>(&self, t: T);
    }

    // fn use_trait_object(t: &dyn X) {}
    // fn use_trait_object2(t: &dyn Y) {}

```
*/

pub fn trait_object() {
    println!("");
}
