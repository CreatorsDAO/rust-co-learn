//! 引用
//!

/**

```
    // 1 不可变借用
    let num = 42;
    let immutable_s = &num;

    // 2 不可变借用
    let mut num = 42;
    let mutable_s = &mut num;

    // 使用场景：当类型占用空间比较大时，可以通过引用来访问或者修改数据,尤其是在传递数据的场景下

    let person_tuple = ("Rust", 13, true);

    let ptr = &person_tuple;
    println!("{}", ptr.0); // "Rust"

    let mut arr = ["Rust", "Go", "C++"];

    let arr_ptr = &mut arr;

    arr_ptr[2] = "Java";

    println!("{:?}", arr_ptr) // ["Rust", "Go", "Java"]


```
*/

pub fn quotation() {
    println!("");
}
