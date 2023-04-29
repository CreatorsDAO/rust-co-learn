//! 属性宏
//!

/**

```
    use attribute_macro::show_function_name;

    // 通过 #[show_function_name] 语法应用属性宏
    #[show_function_name]
    fn my_function() {}

    fn main() {
        // 调用已修改的函数
        my_function();
    }

```
*/

pub fn attribute_macro() {
    println!("");
}
