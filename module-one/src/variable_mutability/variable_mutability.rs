//! 常量和变量
//!
/**

```
    // 1 常量
    // 使用 const 声明; 常量名称使用大写字母; 显式标注类型
    const RUST: &str = "rust";
    const WEIGHT: u64 = 100;

    println!("{},{}",RUST,WEIGHT);

    // 2 变量
    // 使用let 声明,大多数情况下，编译器可以根据上下文推断变量类型
    let name = "rust";
    let age: u32 = 13;

    println!("{},{}",name,age);

    // 3 不变性
    // Rust中变量默认不可变，若需修改变量，需要使用mut关键字声明变量具有可变性
    let _language = "go";
    // _language = "rust"; 无法修改

    // 4 可变性
    // 通过 mut 声明变量
    let mut language = "go";
    language = "rust";

    println!("{}", language);

```
*/

pub fn variable_mutability() {
    println!("");
}
