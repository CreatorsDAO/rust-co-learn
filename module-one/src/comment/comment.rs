//! 文档注释、单行、多行注释
//!

/// 这个函数用于计算两个整数的和。
///
/// # Examples
///
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
fn add(x: i32, y: i32) -> i32 {
    x + y
}

/**

```

    /// 1. 文档注释,一般写在当前文件的最顶端

fn main() {
    /*
       2. 多行注释
       Point 结构体代表二维坐标系下的一个点，通过坐标可以求的任意一点到原点的距离
    */
    struct Point(u32, u32);

    // 3. 单行注释
    // 求某一点到原点距离的平方

    fn distance_square(p: Point) -> u32 {
        p.0 * p.0 + p.1 * p.1
    }

    let p = Point(3, 4);
    distance_square(p);
}


```
*/

pub fn comments() {
    println!("");
}
