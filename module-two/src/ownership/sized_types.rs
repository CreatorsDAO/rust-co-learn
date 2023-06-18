//! 1.3 固定大小类型与所有权
//!

/**

```
    // 1 所有权与基本类型

    // 下面的每个值都只有一个所有者

    let num1 = 42;

    let num2 = num1; // num2是一个新的所有者，它的值是 num1值的复制品，num1仍然是一个有效的所有者
    println!("{}", num1); // 42,可以通过 num1 使用值

    // 现在有两个值和对应的两个有效的所有者，num1 和 num2

    println!("num1 addr {}", num1);
    println!("num2 addr {}", num2);

    // 可以看到值的地址也是不相同（佐证num1和num2各拥有一个值）
    // 对于值42来说，它只有一个所有者，因此现在有两个42的值，并且它们的地址是不同的

    println!("num1 addr {:p}", &num1); // 0x7ff7b404dd90
    println!("num2 addr {:p}", &num2); // 0x7ff7b404dd94

    let f1 = 42.0;
    let b1 = true;

    {
        let c1 = '4'; // ‘4’ 这个值的所有者 `c1` 在离开作用域时，值会被丢弃

        let c2 = c1;

        println!("c1 addr {:p}", &c1); // 0x7ff7b404dde8
        println!("c2 addr {:p}", &c2); // 0x7ff7b404ddec
    }

    // println!("{}", c1) // 无法再使用 owner4,因为它已经被丢弃

    // 2 所有权与复合类型

    let arr1: [i32; 3] = [1, 2, 3];
    let arr2 = arr1;

    println!("arr1 addr {:p}", &arr1); // 0x7ff7b404dd90
    println!("arr2 addr {:p}", &arr2); // 0x7ff7b404dd94

    let tuple1 = (32, true, 42.0);
    let tuple2 = tuple1;

    println!("tuple1 addr {:p}", &tuple1); // 0x7ff7b404dd91
    println!("tuple2 addr {:p}", &tuple2); // 0x7ff7b404dd93

    // 3 所有权与指针类型

    // 这里所说的指针是指指向某个内存地址的变量类型，包括引用、裸指针以及函数指针

    // 3.1 引用
    let n = 21;

    let p1 = &n;
    let p2 = p1;

    println!("p1 addr {:p}", &p1); // 0x7ff7ba58c938
    println!("p2 addr {:p}", &p2); // 0x7ff7ba58c940

    // 3.1 裸指针
    // 在rust中使用 `as *const T` 可以将引用转为裸指针

    let num = 32;

    let r_p1 = &num as *const i32;
    let r_p2 = r_p1;

    println!("r_p1 addr {:p}", &r_p1); // 0x7ff7b9ecc940
    println!("r_p2 addr {:p}", &r_p2); // 0x7ff7b9ecc948

    // 3.2 函数指针
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let f_p1: fn(i32, i32) -> i32 = add;
    let f_p2 = f_p1;

    println!("f_p1 addr {:p}", &f_p1); // 0x7ff7b606f9d0
    println!("f_p2 addr {:p}", &f_p2); // 0x7ff7b606f9d8

    // 总结：对于固定大小类型而言，讲一个变量的值赋值给另一个变量时，实际上是开辟了新的内存空间，并把值拷贝过来，下面的几种基本类型也同理

```
*/

pub fn ownership_with_sized_types() {
    println!("");
}
