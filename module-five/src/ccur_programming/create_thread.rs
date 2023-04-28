//! 创建线程
//!

/**

```
// main函数是主线程
fn main() {
    // 创建线程

    use std::thread;

    let mut threads = vec![];

    // 创建5个线程，它们和main函数的线程是并行的，并且执行结束的时间不一定
    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Hello from thread {}", i);
        });

        threads.push(handle);
    }

    // 等待线程执行结束
    for thread in threads {
        thread.join().unwrap();
    }
}

```
*/

pub fn create_thread() {
    println!("");
}
