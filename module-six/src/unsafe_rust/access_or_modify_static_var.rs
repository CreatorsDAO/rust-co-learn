//! 访问或者修改不可变的静态变量
//!

/**

```

static mut COUNTER: i32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    increment_counter();
    unsafe {
        println!("Counter value: {}", COUNTER); // 输出 "Counter value: 1"
    }
}

```
*/

pub fn access_or_modify_static_var() {
    println!("");
}
