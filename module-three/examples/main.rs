/// 1. 文档注释,一般写在当前文件的最顶端
///
///

fn main() {
    // 1. 容器Cell: 通过移进移出值来实现内部可变性
    use std::cell::Cell;
    let can_not_change = "rust";
    let mut can_change = "go";
    // can_not_change = "cpp"; // 不可重新赋值
    can_change = "c"; // 可以更改
                      // Cell<T> 与 RefCell<T> 源码
                      // Cell 容器的使用

    struct Foo {
        x: u32,
        y: Cell<u32>,         // 包裹实现了copy trait的类型
        z: Cell<Vec<String>>, // 包裹未实现copy trait的类型
    }

    let foo = Foo {
        x: 1,
        y: Cell::new(3),
        z: Cell::new(Vec::new()),
    };

    // 修改容器内的变量使用set方法
    foo.y.set(100);
    foo.z.set(vec!["rust".to_owned()]);

    // 获取容器内的变量有两种 实现 copy trait 的可以使用 get和into_inner; 未实现的只能使用into_inner
    assert_eq!(100, foo.y.get());
    assert_eq!(100, foo.y.into_inner());

    // assert_eq!(vec!["rust".to_owned()], foo.z.get()); 不能使用get方法
    assert_eq!(vec!["rust".to_owned()], foo.z.into_inner());

    // 2. 容器RefCell: 通过borrow_mut实现可变性
    // 主要是应用于一些未实现copy trait类型，通过borrow获取值，有运行时开销

    use std::cell::RefCell;
    let vec = vec![1, 2, 3, 4];

    // vec.push(5); // 不能往不可变的数组中增加元素

    let ref_vec = RefCell::new(vec); //包裹动态数组
    println!("{:?}", ref_vec.borrow()); // 不可变借用 使用borrow
    ref_vec.borrow_mut().push(5); // 可变借用改变，使用borrow_mut
    println!("{:?}", ref_vec.borrow());
}
