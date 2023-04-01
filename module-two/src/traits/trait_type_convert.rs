//! 2.3.2 trait与类型转换
//!

/**

```
     // 1 类型转换trait：From和Into
    // 实现了上述trait的类型可以相互转换,实际上，只需要实现From trait即可，这意味着只要实现了From trait，就可以使用Into trait

    // 1.1 From

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    // 为自定义类型实现From trait，注意这里Trait带了一个类型参数i32，特指将i32转换为Number

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    // 使用From trait中的from方法将i32转换为Number
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // 1.2 Into

    let int = 5;
    // 使用Into trait中的from方法将i32转换为Number
    let num: i32 = int.into();
    println!("My number is {:?}", num);

    // 为自定义类型实现Into trait，注意这里Trait带了一个类型参数Number，特指将Number转换为i32
    impl From<Number> for i32 {
        fn from(item: Number) -> Self {
            item.value
        }
    }

    let num = Number { value: 30 };

    // 使用Into trait中的into方法将Number转换为i32
    let int1: i32 = num.into();
    let num = Number { value: 30 };
    let int2: i32 = i32::from(num);

    // 与此相似的trait还有 TryFrom 和 TryInto
    // 在实际中，TryFrom 和 TryInto 用的比较多，因为它们可以处理错误，但是实现逻辑和 From 和 Into 是一样

    // 2 AsRef 和 AsMut

    // 通过AsMut获取可变引用:注意这里获取结构体成员的可变引用
    impl AsMut<i32> for Number {
        fn as_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
    }

    let mut num = Number { value: 30 };

    let ref_num = num.as_mut();

    // 通过AsRef获取变量的不可变引用:注意这里获取结构体成员的不可变引用
    impl AsRef<i32> for Number {
        fn as_ref(&self) -> &i32 {
            &self.value
        }
    }

    // 特别说明：以上代码展示并不一定是最佳实践，只是为了介绍知识点而展示的可能性

```
*/

pub fn trait_intro() {
    println!("");
}
