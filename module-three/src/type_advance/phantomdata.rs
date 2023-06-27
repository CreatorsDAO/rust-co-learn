//! 2.3 特殊类型 PhantomData
//!

/**

```
use std::marker::PhantomData;
    use std::ops::Deref;

    struct MyType<T> {
        data: *const T,
        _marker: PhantomData<T>,
    }

    impl<T> MyType<T> {
        fn new(t: T) -> MyType<T> {
            MyType {
                data: &t,
                _marker: PhantomData,
            }
        }
    }

    impl<T> Deref for MyType<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.data }
        }
    }

    impl<T> Drop for MyType<T> {
        fn drop(&mut self) {
            println!("Dropping MyType instance!");
        }
    }

    let resource: MyType<bool> = MyType::new(true);
    let another_resource: MyType<i32> = MyType::new(32);

    print!("{:?}", unsafe { *(resource.data) });
    print!("   {:?}", unsafe { *(another_resource.data) });

    let my_instance: MyType<i32> = MyType::new(33);
    // 执行到这里时，my_instance 将会离开作用域并被销毁，调用我们自定义的 drop 方法。
```
*/

pub fn phantomdata() {
    println!("");
}
