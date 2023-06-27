//! 2.2 可变容器
//!

/**

```

// 1.编译期：通过 mut 显式声明变量的可变性，也叫外部可变性
use std::cell::Cell;
let can_not_change = "rust";
let mut can_change = "go";
// can_not_change = "cpp"; // 不可重新赋值
can_change = "c"; // 可以更改

// 2 一个需要改变不可变变量的例子

// let var1 = 0;
// let mut var2 = 0;

// while var2 <= 10 {
//     if var2 == 10 {
//         var1 = 10;
//     }
//     var2 += 1;
// }

// println!("var1: {}, var2: {}", var1, var2);

// 3. 运行期：通过Cell和RefCell实现可变性，也叫内部可变性
// 3.1 Cell<T> 的修改和读取
struct Foo {
    x: u32,
    y: Cell<u32>,
    z: Cell<Vec<String>>,
}

let foo = Foo {
    x: 1,
    y: Cell::new(3),
    z: Cell::new(Vec::new()),
};

// 修改容器内的变量使用set方法
foo.y.set(100);
foo.z.set(vec!["rust".to_owned()]);

// 读取容器内的变量有两种：固定大小类型可以使用 get和into_inner; 动态大小类型只能使用into_inner
assert_eq!(100, foo.y.get());
assert_eq!(100, foo.y.into_inner());

// assert_eq!(vec!["rust".to_owned()], foo.z.get()); 不能使用get方法
assert_eq!(vec!["rust".to_owned()], foo.z.into_inner());

// 3.2 RefCell<T> 的修改和读取
// 通过borrow_mut实现可变性
// 主要是应用于一些动态大小类型，通过borrow获取值，有运行时开销

use std::cell::RefCell;
let vec = vec![1, 2, 3, 4];

let ref_vec = RefCell::new(vec);

println!("{:?}", ref_vec.borrow()); // 不可变借用 使用borrow
ref_vec.borrow_mut().push(5); // 可变借用改变，使用borrow_mut
println!("{:?}", ref_vec.borrow());

```
*/

pub fn mut_container() {
    println!("");
}
