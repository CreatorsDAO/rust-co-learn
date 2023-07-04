use lib2::lib2_type::types::create_lib2_struct;
use lib3::lib3_type::types::create_lib3_struct;
pub struct Lib1_struct<T> {
    name: T,
    num: i32,
}

pub fn create_lib1_struct() -> Lib1_struct<String> {
    let s = Lib1_struct {
        name: String::from("lib1"),
        num: 32,
    };

    print!("lib1 struct created! ");

    create_lib2_struct();
    create_lib3_struct();

    s
}
