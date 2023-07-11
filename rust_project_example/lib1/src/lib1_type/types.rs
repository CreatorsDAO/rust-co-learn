#[allow(dead_code)]
use lib2::lib2_type::types::create_lib2_struct;
use lib3::lib3_type::types::create_lib3_struct;
pub struct Lib1Struct<T> {
    #[allow(dead_code)]
    name: T,
    #[allow(dead_code)]
    num: i32,
}

pub fn create_lib1_struct() -> Lib1Struct<String> {
    let s = Lib1Struct {
        name: String::from("lib1"),
        num: 32,
    };

    print!("lib1 struct created! ");

    create_lib2_struct();
    create_lib3_struct();

    s
}
