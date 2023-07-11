#[allow(dead_code)]

pub struct Lib2Struct<T> {
    name: T,
    num: i32,
}

pub fn create_lib2_struct() -> Lib2Struct<String> {
    let s = Lib2Struct {
        name: String::from("lib1"),
        num: 32,
    };

    print!("lib2 struct created! ");

    s
}
