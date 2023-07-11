#[allow(dead_code)]
pub mod code;
mod doc;
pub mod lib1_type;

// use lib1_type::traits::Lib1_trait;// 测试模块

// 导出这个函数
pub use my_add::*;
mod my_add {

    pub fn add() -> i32 {
        2 + 2
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use code::add_two;
    use code::read_file;
    use code::Guess;
    use code::Rectangle;

    // 1 使用 assert! 宏断言结果是 true

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
    // 3 使用 assert_ne! 宏断言两个值不相等

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
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
