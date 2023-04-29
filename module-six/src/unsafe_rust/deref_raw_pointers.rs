//! 解引用裸指针
//!

/**

```

fn main() {
    let x = 10;
    let y = &x as *const i32;
    let z = 0x12345678 as *const i32; // 假设这是一个无效的内存地址

    unsafe {
        println!("Value of x: {}", *y); // 输出 "Value of x: 10"
        // println!("Value at address 0x12345678: {}", *z); // 不安全！可能导致未定义行为
    }
}

```
*/

pub fn dereferencing_raw_pointers() {
    println!("");
}
