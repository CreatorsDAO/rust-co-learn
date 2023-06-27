//! 2.4 特殊类型 Pin、PhantomPinned
//!

/**

```
   use std::marker::PhantomPinned;
   use std::pin::Pin;

// 定义一个自引用的结构体。因为它含有指向自身的指针，所以它在内存中不能被移动。
struct SelfReferential {
    i: i32,
    p: *const i32,       // 裸指针，将会指向上述的 i
    _pin: PhantomPinned, // 也是一个零大小的标记类型，阻止 Rust 自动为我们的类型实现 Unpin trait
}

// 注意此时 p 是一个空指针，我们还没有为它分配指向的地址
let mut test = SelfReferential {
    i: 123,
    p: std::ptr::null(),
    _pin: PhantomPinned,
};

// 使用 Pin 包装我们的结构体实例。这样就能保证 test 的内存地址不会在其生命周期中改变。
// 注意：这里使用了 unsafe，因为我们需要保证在 test 被包装为 Pin 后，其地址不会被改变
let mut test = unsafe { Pin::new_unchecked(&mut test) };

// 创建一个裸指针，指向 test 的 i 字段。注意我们使用了 test 的引用版本以保证安全。
let self_ptr: *const i32 = &test.as_ref().get_ref().i;

// 将裸指针存储到 test 的 p 字段。注意我们使用了 unsafe，因为我们正在直接修改内存。

unsafe {
    let mut_ref = Pin::as_mut(&mut test);
    mut_ref.get_unchecked_mut().p = self_ptr;
}

// 打印 test 的 p 字段所指向的内容。注意我们使用了 unsafe，因为我们正在解引用裸指针。
let val = unsafe { *(test.as_ref().get_ref().p) };
println!("val: {}", val); // 输出 "val: 123"
```
*/

pub fn pin() {
    println!("");
}
