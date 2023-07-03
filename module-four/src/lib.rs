#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]

pub mod error_handle;
// pub mod project_management;
pub mod project_tests;

pub use my_add::*;
mod my_add {

    pub fn add() -> i32 {
        2 + 2
    }
}
