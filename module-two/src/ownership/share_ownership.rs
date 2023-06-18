//! 1.6 所有权共享
//!

/**

```

    // 1 独占访问资源

    let mut dynamic_source = String::from("content");

    let role1 = dynamic_source;
    // let role2 = dynamic_source; // 资源被 role1 所有，此时role1独占访问
    let role2 = role1; // 只有role1 把所有权移交给 role2， role2 才可以访问

    // 这样做的好处是，可以避免资源被多个变量同时访问，导致资源被修改
    // 坏处是，资源只能被一个变量访问，低效

    use std::rc::Rc;
    use std::sync::Arc;

    // 2 所有权与共享容器 Rc<T>,它适用于单线程

    // 使用共享容器包裹动态资源

    let dynamic_source = vec![1, 2];

    let container = Rc::new(dynamic_source);

    let role1 = container.clone(); // 这里clone方法其实是复制了对资源访问的所有权，而不是资源本身
    let role2 = container.clone();

    // 通过共享容器访问资源,此时资共享资源有三个所有者，可以同时访问
    println!("{:?}", container); // [1,2]
    println!("{:?}", role1); // [1,2]
    println!("{:?}", role2); // [1,2]

    // 3 所有权共享容器 Arc<T>，它适用于多线程

    let dynamic_source = String::from("rust");

    let container = Arc::new(dynamic_source);

    let role1 = container.clone(); // 这里clone方法其实是复制了对资源访问的所有权，而不是资源本身
    let role2 = container.clone();

    // 通过共享容器访问资源,此时资共享资源有三个所有者，可以同时访问
    println!("{:?}", container); // rust
    println!("{:?}", role1); // rust
    println!("{:?}", role2); // rust

    // 4 共享容器与内存管理
    // 注意：Rc<T>和Arc<T>实际上是一种引用计数，每使用clone方法一次，引用计数就会+1，当变量离开作用域时，引用计数会-1，当引用计数为0时，堆内存会被释放
    // 整个过程在编译器看来，每个变量都拥有一个Rc或者Arc。所以并不违反所有权规则
    // 这里提一点:一般情况下，Rust使用栈来管理堆内存。但是Rc和Arc是一种特别的机制，它允许不受栈内存控制的堆内存，也就是允许内存泄露。对于这种泄漏通过引用计数来管理

    // 4.1 通过栈内存来管理堆内存

    {
        let source = String::from("hello");

        let role1 = source;
        println!("{:?}", role1);
        // 丢弃

        // println!("{:?}", source); // 不能再使用source，因为source已经移交了所有权
        // 当role1离开作用域时，会立即丢弃 role1和堆上的数据
    }

    // 4.2 通过引用计数来管理堆内存

    {
        let source = String::from("hello");

        // 使用Rc包裹资源，让堆上资源生命周期更长
        let container = Rc::new(source); // 引用计数 + 1
                                         //
        let role1 = container.clone(); // 引用计数 + 1
        let role2 = container.clone(); // 引用计数 + 1

        // 当变量离开作用域时，role2，role1，container相继离开作用域时，引用计数都会-1，当引用计数为0时，堆上的数据才会被释放
    }

```
*/

pub fn share_ownership() {
    println!("");
}
