/// 1. 文档注释,一般写在当前文件的最顶端

fn main() {
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

    // 3 字符串切片
    // 字符串本质上一个u8序列，支持切片操作
    let s1 = String::from("Hello Rust"); // String本质是一个指向堆上str的指针
    let s2 = "Hello Rust"; //&str 很简答，就是一个 str的引用

    // 使用切片的操作过程：可以理解为先通过指针/引用解引用到str本身，然后通过切片操作符号拿一段出来
    // 但拿出来的片段仍然是str，Rust是静态语言，需要在编译期确定类型的大小，所以还需要引用切片
    // 整个过程等价于下面
    let _x = &(*s1)[0..5]; //等价于 &s1[0..5];
    let _y = &(*s2)[6..10]; //等价于 &s2[6..10]

    let slice1 = &s1[0..5];
    let slice2 = &s2[6..10];

    println!("slice1: {}", slice1); // Hello
    println!("slice2: {}", slice2); // Rust
}
