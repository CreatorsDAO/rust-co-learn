//! 声明宏
//!

/**

```

// 导入 HashMap，用于存储键值对。
use std::collections::HashMap;
// 导入 lazy_static 宏，用于创建静态变量。
use lazy_static::lazy_static;

// 定义一个名为 `create_map` 的宏，它接受一系列键值对，并在展开时创建一个包含这些键值对的 HashMap。
// 宏使用了匹配表达式 (`$key:expr => $value:expr`) 来捕获键值对，并使用重复模式 (`$()*`) 来插入每个键值对到 HashMap。
macro_rules! create_map {
    // 接受一系列键值对，每个键值对以 `$key:expr => $value:expr` 的形式给出。
    // 最后一个键值对后面可以有一个可选的逗号。
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        // 创建一个新的 HashMap。
        let mut map = HashMap::new();
        // 使用重复模式，对每一个键值对执行以下代码。
        $(
            // 将每个键值对插入到 HashMap 中。
            map.insert($key, $value);
        )*
        // 返回填充好的 HashMap。
        map
    }};
}

fn main() {
    // 使用 lazy_static 宏创建一个名为 `FRUITS` 的静态 HashMap。
    // 这里我们使用 `create_map!` 宏来初始化 HashMap。
    lazy_static! {
        static ref FRUITS: HashMap<&'static str, u32> = create_map! {
            "apple" => 1,
            "banana" => 2,
            "orange" => 3,
            "peach" => 4,
        };
    }

    // 打印 FRUITS 的内容。注意，我们需要使用 *FRUITS 来解引用 FRUITS。
    println!("{:?}", *FRUITS);
}

```
*/

pub fn declarative_macros() {
    println!("");
}

// Cargo.toml中的依赖项
// [dependencies]
// lazy_static = "1.4.0"
