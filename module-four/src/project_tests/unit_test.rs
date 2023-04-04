//! 4.3.1 单元测试
//!

/**

```

use std::fs::File;
use std::io::Error;

fn read_file(path: &str) -> Result<File, Error> {
    // 2.1 读取文件
    let file = File::open(path);

    // 2.2 判断文件是否存在
    match file {
        Ok(file) => Ok(file),
        Err(error) => Err(error),
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1 使用 assert! 宏断言结果是抖为 true

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller)); // 可以直接断言，也可以带上提示信息
        assert!(
            larger.can_hold(&smaller),
            "larger is {:?}, smaller is {:?}",
            larger,
            smaller
        );
    }

    // 2 使用 assert_eq! 宏断言两个值相等

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // 3 使用 assert_ne! 宏断言两个值不相等
    #[test]
    fn it_adds_two() {
        assert_ne!(3, add_two(2));
    }

    // 4 使用 should_panic 宏断言函数会 panic

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 5 使用Result<T, E>类型的断言

    #[test]
    fn read_file_should_works() -> Result<(), String> {
        match read_file("rust.txt") {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("file did not exit")),
        }
    }
}

```
*/
pub fn unit_test() {}
