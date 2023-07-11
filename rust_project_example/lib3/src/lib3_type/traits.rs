#[allow(dead_code)]
pub trait Lib3Trait {
    fn new(&self) -> Self;
    fn print(&self) {
        println!("Hello,it comes from lib3")
    }
}
