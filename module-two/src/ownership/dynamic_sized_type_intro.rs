//! 1.4 动态大小类型介绍
//!

/**

```

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

    // 2 String

    let mut string_data = String::from("Hello, Rust!");
    let size_of_string = string_data.len();
    let ptr_of_string = string_data.as_ptr();

    println!("Size of string data: {} bytes", size_of_string);
    println!("Address of string data: {:p}", ptr_of_string);

    string_data = String::from("Hello Rust, how are you today?");

    let size_of_string = string_data.len();
    let ptr_of_string = string_data.as_ptr();

    println!("Size of string data: {} bytes", size_of_string);
    println!("Address of string data: {:p}", ptr_of_string);

    // 3 vec

    let mut vec_data = vec![1];
    let size_of_vec = vec_data.len();
    let ptr_of_vec = vec_data.as_ptr();

    println!("Size of vector data: {} bytes", size_of_vec);
    println!("Address of vector data: {:p}", ptr_of_vec);

    vec_data.push(2);
    vec_data.push(3);
    vec_data.push(4);
    vec_data.push(5);
    vec_data.push(6);
    vec_data.push(7);
    vec_data.push(8);
    vec_data.push(9);

    let size_of_vec = vec_data.len();
    let ptr_of_vec = vec_data.as_ptr();

    println!("Size of vector data: {} bytes", size_of_vec);
    println!("Address of vector data: {:p}", ptr_of_vec);

```
*/

pub fn dynamic_sized_types_intro() {
    println!("");
}
