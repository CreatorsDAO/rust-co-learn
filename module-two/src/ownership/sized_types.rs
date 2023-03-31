//! 2.1.1 固定尺寸类型
//!

/**

```
    // 1 所有权与基本类型

    // 下面的每个值都只有一个所有者

    let owner1 = 42;

    let owner11 = owner1; // owner是一个新的所有者，它的值是 owner1值的复制品，owner1仍然是一个有效的所有者
    println!("{}", owner1); // 42,可以通过 owner1 使用值

    // 现在有两个值和对应的两个有效的所有者，owner1 和 owner11

    println!("owner1 addr {}", owner1);
    println!("owner11 addr {}", owner11);

    // 可以看到值的地址也是不相同（佐证owner11和owner1各拥有一个值）
    // 对于值42来说，它只有一个所有者，因此现在有两个42的值，并且它们的地址是不同的

    println!("owner1 addr {:p}", &owner1); // 0x7ff7b404dd90
    println!("owner11 addr {:p}", &owner11); // 0x7ff7b404dd94

    let owner2 = 42.0;
    let owner3 = true;

    {
        let owner4 = '4'; // ‘4’ 这个值的所有者 `owner4` 在离开作用域时，值会被丢弃
    }

    // println!("{}", owner4) // 无法再使用 owner4,因为它已经被丢弃

    // 2 所有权与复合类型

    let arr_owner: [i32; 3] = [1, 2, 3];
    let tuple_owner = (32, true, 42.0);

    // 3 所有权与指针类型

    // 这里所说的指针是指指向某个内存地址的变量类型，包括引用、裸指针以及函数指针

    // 3.1 字符串的引用

    let ptr_owner = "rust";
    let num = 42;

    // 注意: ptr_owner 是字符串引用的所有者，而不是字符串的所有者，这里的`值`就是引用本身

    let ptr_copy = ptr_owner; // 此处所有者 ptr_copy 的值是 ptr_owner 的值的复制品，ptr_owner 仍然是一个有效的所有者

    // 由于 ptr_owner 和 ptr_copy 的值都是指向相同值的引用，所以它们指向的内存地址是相同的
    println!("{:p}", ptr_owner); // 0x10ac12004
    println!("{:p}", ptr_copy); // 0x10ac12004

    // 3.2 基本类型的裸指针
    // 在rust中使用 `as *const T` 可以将引用转为裸指针

    let ptr_owner2 = &num as *const i32;

    // 3.3 函数指针

    fn func() -> i32 {
        0
    }
    let func_ptr = func;

```
*/

pub fn ownership_with_sized_types() {
    println!("");
}
