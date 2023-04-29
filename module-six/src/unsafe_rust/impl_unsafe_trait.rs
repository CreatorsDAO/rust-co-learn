//! 实现不安全的trait
//!

/**

```

unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

struct MyStruct;

unsafe impl UnsafeTrait for MyStruct {
    fn unsafe_method(&self) {
        // 执行不安全的操作
    }
}

fn main() {
    let my_struct = MyStruct;

    unsafe {
        my_struct.unsafe_method();
    }
}


```
*/

pub fn impl_unsafe_trait() {
    println!("");
}
