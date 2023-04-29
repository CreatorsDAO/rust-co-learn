//! 类函数宏
//!

/**

```

use function_like_macro::ImplAddIntegers;

#[derive(ImplAddIntegers)]
struct MyStruct;

fn main() {
    let result = MyStruct::add_integers(3, 5);
    println!("The sum is: {}", result); // The sum is: 8
}


```
*/

pub fn function_like_macro() {
    println!("");
}
