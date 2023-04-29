//! 派生宏
//!

/**

```
use derive_macro::SimpleDebug;

// 通过 #[derive(SimpleDebug)] 语法应用自定义派生宏
#[derive(SimpleDebug)]
struct TestStruct;

fn main() {
    let test_instance = TestStruct;
    // 调用 Debug trait 的实现
    println!("{:?}", test_instance);
}

```
*/

pub fn derive_macro() {}
