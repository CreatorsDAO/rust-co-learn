use rand::Rng;
pub struct Lib3_struct<T> {
    name: T,
    num: i32,
}

pub fn create_lib3_struct() -> Lib3_struct<String> {
    let s = Lib3_struct {
        name: String::from("lib1"),
        num: 32,
    };

    print!("lib3 struct created! ");

    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10);
    println!("Generated random number: {}", num);

    s
}
