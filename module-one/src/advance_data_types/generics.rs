//! 泛型
//!

/**

```

    // 1 泛型参数的表示
    // 泛型参数一般用大写字母`T`表示,多个泛型参数可以使用多个大写字母
    // 可以把泛型当作自定义类型，必须先声明才能使用

    // 2 泛型如何使用
    // 2.1 集合 Vec<T>
    // 集合vector就是由泛型提供支持的,它允许我们使用某个具体类型时再指定
    let v1: Vec<u8> = Vec::new();
    let v2: Vec<String> = Vec::new();
    let v3: Vec<bool> = Vec::new();

    // 2.2 泛型结构体
    // 可以声明一个泛型结构体，然后再使用的时候在指定成员的具体类型
    // 注意：必须先在` <> `中声明泛型参数，然后才能使用
    struct Type<T>(T);
    struct Point<A, B> {
        a: A,
        b: B,
    }

    let t1 = Type(42);
    let t2 = Type("rust");

    let p1 = Point { a: 42, b: 42 };
    let p2 = Point { a: 42.1, b: 42.1 };

    // 为泛型结构体实现方法
    // 注意：为泛型结构体实现方法时，impl和结构体后面的泛型声明要保持一致
    impl<A, B> Point<A, B> {
        fn new(a: A, b: B) -> Self {
            Point { a, b }
        }
    }

    // 2.3 泛型枚举
    // 同样，可以定义泛型枚举
    enum Area<A, B, C> {
        Rectangle(A),
        Square(B),
        Circle(C),
    }

    let a1: Area<f64, u32, &str> = Area::Rectangle(42f64);
    let a2: Area<f32, u64, &str> = Area::Square(42u64);
    let a3: Area<f64, u32, &str> = Area::Circle("100 cm^2");

    // 2.4 泛型函数
    // 函数参数也可以是泛型, 当然泛型也需要在 `<>` 中先声明
    fn generics<T, B>(a: T, b: B) -> T {
        a
    }
    generics(32, "rust"); // 32
    generics("rust", 32); // "rust"


```
*/

pub fn generics() {
    println!("");
}
