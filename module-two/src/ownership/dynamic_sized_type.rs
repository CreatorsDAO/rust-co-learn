//! 1.5 动态大小类型与所有权机制
//!

/**

```
    // 1 所有权与字符串

    // 我们在前面介绍过，字符串可以存放在程序的只读数据段中或者堆上
    // 一般情况下，字符串字面量存放在只读数据段中的，声明之后很少去修改它
    // 而需要动态变化的字符串我们会把它存放到堆上，并且通过栈内存来管理堆内存

    let ptr_owner = "Rust"; // 存放在只读数据段中
    let heap_ptr_owner = String::from("Rust"); //存放在堆上

    // 1.1 对于存放在只读数据段中的字符串字面量，它的所有权规则和其他基本类型一样,这里不再赘述

    let ptr_copy = ptr_owner;

    // 由于 ptr_owner 和 ptr_copy 的值都是指向相同值的引用，所以它们指向的内存地址是相同的
    println!("{:p}", ptr_owner); // 0x10ac12004
    println!("{:p}", ptr_copy); // 0x10ac12004

    let mut _heap_ptr_old = String::from("Rust"); //存放在堆上

    let heap_ptr_new = _heap_ptr_old;

    // println!("old owner{:?}", _heap_ptr_old); // 无法再通过 _heap_ptr_old 使用值，因为它已经把数据所有权移交给了新的所有者 heap_ptr_new
    println!("new owner{:?}", heap_ptr_new); // heap_ptr_new 可以正常访问到堆上的数据，并且它是唯一的所有者，当它离开作用域时，堆上的数据也会被丢弃

    {
        let owner_old = String::from("rust");
        let owner_new = owner_old;

        // 在此处离开作用域
    }

    // println!("{:?}", owner_new); 无法再通过 owner_new 使用值，因为它已经被丢弃

    _heap_ptr_old = String::from("Go"); // 重新赋值，注意原变量不能使用是因为转移所有权后被标注为空了，而不是立即被清除了

    // 2 所有权与slice

    // 上面的字符串str 实际上是一个特殊的 slice, 它仅代表有效的utf-8序列
    // 而切片中可以包含任何类型的元素，如其他基础类型、自定义类型等, 正如不直接使用 str一样，我们也不直接使用[T],而是使用它的指针（引用）类型，Vec<T>
    // slice中的数据也存放在堆上，Rust中slice内存管理逻辑同存放在堆上的str

    // vec 有两种创建方式：使用宏或者方法
    let str_slice = vec!["rust", "go", "cpp"];
    let u32_slice: Vec<u32> = Vec::new();

    let new_owner1 = str_slice;
    let new_owner2 = u32_slice;

    // println!("{:?}", str_slice); // 无法再通过 str_slice 使用值，因为它已经被丢弃
    // println!("{:?}", u32_slice); // 无法再通过 u32_slice 使用值，因为它已经被丢弃

    println!("{:?}", new_owner1); // 可以通过新的所有者访问到原来的值
    println!("{:?}", new_owner2); // 可以通过新的所有者访问到原来的值

    // 3 总结
    // 当数据存放在堆上时，把所有权赋值给另一个变量，意味着把堆上所有权就会转移给新的所有者，堆上的数据本身没有被复制，原来的所有者不再拥有数据
    // 当数据存放在栈上时，把所有权赋值给另一个变量，意味着把栈上的数据复制了一份给新的所有者，原来的所有者仍然拥有原来的数据

```
*/

pub fn ownership_with_dynamic_sized_types() {
    println!("");
}
