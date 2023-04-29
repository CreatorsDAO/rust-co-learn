// use derive_macro::SimpleDebug;

// // 通过 #[derive(SimpleDebug)] 语法应用自定义派生宏
// #[derive(SimpleDebug)]
// struct TestStruct;

// fn main() {
//     let test_instance = TestStruct;
//     // 调用 Debug trait 的实现
//     println!("{:?}", test_instance);
// }

// use attribute_macro::show_function_name;

// // 通过 #[show_function_name] 语法应用属性宏
// #[show_function_name]
// fn my_function() {}

// fn main() {
//     // 调用已修改的函数
//     my_function();
// }

use function_like_macro::ImplAddIntegers;

#[derive(ImplAddIntegers)]
struct MyStruct;

fn main() {
    let result = MyStruct::add_integers(3, 5);
    println!("The sum is: {}", result); // The sum is: 8
}
