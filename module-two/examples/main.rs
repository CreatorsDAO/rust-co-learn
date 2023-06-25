fn main() {
    // 1. &str
    let mut a = "rust";
    let size_of_a = std::mem::size_of_val(a);
    let ptr_of_a = a.as_ptr();

    println!("Size of 'rust': {} bytes", size_of_a); // 打印结果：Size of 'rust': 4 bytes
    println!("Address of 'rust': {:p}", ptr_of_a); // 打印结果：Address of 'rust': 0x107e52fa0

    a = "go";
    let size_of_a = std::mem::size_of_val(a);
    let ptr_of_a = a.as_ptr();

    println!("Size of 'go': {} bytes", size_of_a); // 打印结果：Size of 'go': 2 bytes
    println!("Address of 'go': {:p}", ptr_of_a); // 打印结果：Address of 'go': 0x107e52fdb

    let ptr_of_rust = "rust".as_ptr(); // 访问初始的“rust”
    println!("Address of 'rust' after reassignment: {:p}", ptr_of_rust); // 打印结果：Address of 'rust' after reassignment: 0x107e52fa0

    let a = "rust";

    let b = a;

    print!("--a{:p}", a.as_ptr());
    print!("--b{:p}", b.as_ptr());

    use std::mem;

    let mut s = String::with_capacity(10);
    let ptr_before = s.as_ptr();
    println!("Capacity: {}, Address: {:?}", s.capacity(), ptr_before);

    s.push_str("Hello");
    let ptr_during = s.as_ptr();
    println!("Capacity: {}, Address: {:?}", s.capacity(), ptr_during);

    s.push_str(" Worldoooooooooooooooooooooooooooooooooooooooooooooooooooooooooo!");
    let ptr_after = s.as_ptr();
    println!("Capacity: {}, Address: {:?}", s.capacity(), ptr_after);

    // 确保在检查扩容后的地址时，字符串s还没有被drop，否则会出现悬垂指针
    mem::forget(s);
}
