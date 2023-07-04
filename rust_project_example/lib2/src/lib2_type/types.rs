pub struct Lib2_struct<T> {
    name: T,
    num: i32,
}

pub fn create_lib2_struct() -> Lib2_struct<String> {
    let s = Lib2_struct {
        name: String::from("lib1"),
        num: 32,
    };

    print!("lib2 struct created! ");

    s
}
