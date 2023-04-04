//!  标量类型
//!

/**

```
    // 1 整数类型
    // Rust 中整数类型分为有符号和无符号类型；长度分为8位，16位，32位，64位，128位
    // 特殊的整数类型: usize 和 isize，与系统架构相关，32位的系统则为32位，64位的系统为64位
    let integer: i32 = 42;
    let s: usize = 100;

    // 2 浮点类型
    // Rust 的浮点型是 f32 和 f64，大小分别为 32 位和 64 位。默认浮点类型是 f64
    // 浮点型都是有符号的
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 3 布尔类型
    let t = true;
    let f: bool = false;

    // 4 字符类型 char
    // Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

```
*/

pub fn scalar() {
    println!("");
}
