//! 模式匹配
//!

/**

```
    // 1 match
    // match 是最长用的模式匹配，主要和枚举搭配使用，以匹配不同的枚举成员

    match std::fs::File::open("rust.txtr") {
        Ok(file) => println!("{:?}", file),
        Err(err) => panic!("{}", err),
    }

    // 2 if let
    // if let 可以让我们只关注我们想要的结果

    if let Ok(file) = std::fs::File::open("rust.txtr") {
        println!("{:?}", file);
    }

    // 3 while let
    // 和 if let 类似，只处理正确的结果

    while let Ok(file) = std::fs::File::open("rust.txt") {
        println!("{:?}", file);
    }

    // 4 let
    // let 本身也是一种模式匹配
    // 使用 let 匹配元组中的元素

    let tuple = (42, true, "rust");

    let (x, y, z) = tuple;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

```
*/

pub fn model_match() {
    println!("");
}
