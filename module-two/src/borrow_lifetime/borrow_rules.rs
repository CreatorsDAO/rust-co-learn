//! 2.1 借用与引用以及借用检查规则
//!

/**

```

    let mut s = String::from("Rust"); // 对于s来说，它将值借用给了s_p
    let mut i = 32;

    let s_p = &mut s; // 对于s_p来说，它引用了s的值
                      // let s_p1 = &mut s; // 对于s_p1来说，它引用了s的值
                      // let s_p3 = &s;

    let i_p2 = &i;
    let i_p = &i;

    // s = String::from("Go");
    *s_p = String::from("Java");

    {
        let c = 'a';

        let c_p = &c;
        println!("{}", c_p);
        // c_p 离开作用域，对应的值（引用）资源会被释放
    }

    // println!("{}", c_p);


```
*/

pub fn borrow_lifetime() {
    println!("");
}
