//! 实现不安全的trait
//!

/**

```

unsafe trait UnsafeTrait {
    unsafe fn unsafe_method(&self);
}

struct MyStruct {
    value: i32,
}

unsafe impl UnsafeTrait for MyStruct {
    unsafe fn unsafe_method(&self) {
        let mut raw_ptr = self as *const Self as *mut Self;
        (*raw_ptr).value = 42;
    }
}

fn main() {
    let my_struct = MyStruct { value: 10 };

    unsafe {
        my_struct.unsafe_method();
    }
}

```
*/

pub fn impl_unsafe_trait() {
    println!("");
}
