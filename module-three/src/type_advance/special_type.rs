//! 2.4 特殊类型
//!

/**

```

fn main() {

// 1 特殊类型：PhantomData<T>

    use std::marker::PhantomData;

    struct MyType<T> {
        _marker: PhantomData<T>,
    }

    impl<T> MyType<T> {
        fn new() -> MyType<T> {
            MyType {
                _marker: PhantomData,
            }
        }
    }

    fn main() {
        let a: MyType<u32> = MyType::new();
        let b: MyType<String> = MyType::new();
    }

    // 2 特殊类型：Pin<T>

    use std::pin::Pin;

    struct MType {
        data: String,
    }

    impl MType {
        fn new(data: String) -> MType {
            MType { data }
        }

        fn get_data(self: Pin<&Self>) -> &str {
            unsafe { &self.get_ref().data }
        }
    }

    let my_type = MType::new("hello".to_string());
    let pinned = Pin::new(&my_type);
    let data = pinned.get_data();
    println!("{}", data);

}
```
*/

pub fn special_types() {
    println!("");
}
