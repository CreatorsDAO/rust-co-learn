#[allow(dead_code)]
use std::fs::File;
use std::io::Error;

pub fn read_file(path: &str) -> Result<File, Error> {
    // 2.1 读取文件
    let file = File::open(path);

    // 2.2 判断文件是否存在
    match file {
        Ok(file) => Ok(file),
        Err(error) => Err(error),
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
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
