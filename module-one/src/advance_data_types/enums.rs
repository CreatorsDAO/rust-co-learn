//! 枚举
//!

/**

```
    // 枚举
    // 枚举在形式上和结构体较为相似
    enum Subject {
        Math,
        Chinese,
        English(String),
    }

    // 初始化
    let subject = Subject::English(String::from("English"));

    //标准库中两个比较重要的枚举 Option和 Result
    // Result 用于一些处操作可能遇到错误的场景，比如打开文件时，如果成功，返回文件，遇到错误时返回一个Error
    use std::fs::File;

    let file: Result<File, std::io::Error> = File::open("tmp.txt");


    // Option 用于一些值可能为空的情况
    // 如尝试获取哈希表中某个key所对应的value，当值存在时，返回值，当不存在时返回None

    use std::collections::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let v1: Option<&u32> = map.get("rust"); // None
    println!("{:?}", v1);

    map.insert("rust", 1);
    let v2: Option<&u32>  = map.get("rust");
    println!("{:?}", v2);

```
*/

pub fn enums() {
    println!("");
}
