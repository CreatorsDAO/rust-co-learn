//! 错误处理
//!

/**

```
    // 1 类型系统保证函数契约
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // sum(1u32, 2u32) 违反函数契约

    // 2 使用Option处理有值或无值的情况
    // 当某个值可能为无值时，应该使用Option<T>来包裹，以正确处理无值的情况
    fn log(val: f64) -> Option<f64> {
        match val.log2() {
            x if x.is_normal() => Some(x), // 有值情况
            _ => None,                     // 无值情况
        }
    }

    // 当一个值为Option<T>时，经常使用map和and_then等方法来链式处理

    fn double(val: f64) -> f64 {
        val * 2.
    }

    fn square(val: f64) -> f64 {
        val.powi(2 as i32)
    }

    fn inverse(val: f64) -> f64 {
        val * -1.
    }

    fn sqrt(val: f64) -> Option<f64> {
        match val.sqrt() {
            x if x.is_normal() => Some(x),
            _ => None,
        }
    }

    let number = 20.;
    let result = Option::from(number)
        .map(inverse)
        .map(double)
        .map(inverse)
        .and_then(log)
        .map(square)
        .and_then(sqrt);
    match result {
        Some(x) => println!("x was {:?}", x),
        None => println!("this failed"),
    }

    // 3 Result 用于处理成功或失败的情况

    use std::fs::File;
    use std::io::prelude::*;


    fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    let file_path = "example.txt";
    match read_file_contents(file_path) {
        Ok(contents) => println!("File contents: {}", contents), // 成功情况
        Err(error) => println!("Error reading file: {}", error), // 失败情况，抛出错误
    }

    // 4 断言
    // 当你确定某个值一定会出现某种情况时，可以使用断言来终止程序

    fn extend_vec(v: &mut Vec<i32>, i: i32) {
        assert!(v.len() == 3); // 断言
        v.push(i)
    }

    let mut vec = vec![1, 2, 3];
    extend_vec(&mut vec, 4);

    assert_eq!(4, vec[3]); // 断言

    // 5 恐慌
    // 当你确定某个值一定不会出现某种情况时，可以使用恐慌来终止程序
    //

    fn factorial(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    let result = factorial(10);
    println!("Result: {}", result);
    if result < 1000 {
        panic!("Result too large!"); // 使用panic!恐慌
    }

    // 总结：使用Option和Result来处理值或者错误，使用恐慌和断言来终止程序

```
*/

pub fn error_handle() {
    println!("");
}
