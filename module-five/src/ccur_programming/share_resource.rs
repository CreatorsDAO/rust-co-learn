//! 资源共享
//!

/**

```

   // main函数是主线程
fn main() {
    // 1 通过 channel 共享数据

    use std::sync::mpsc;
    use std::thread;

    // 创建通道（信息接收者和信息发送者）

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        // 在线程中创建变量
        let val1 = String::from("hi");
        let val2 = String::from("hello");

        // 将变量发送给别的线程
        tx1.send(val1).unwrap();
        tx2.send(val2).unwrap();
        // println!("{:?}", val); // 不能再使用
    });

    // 接收数据
    for received in rx {
        println!("Got: {}", received);
    }

    // 2 使用锁共享数据

    use std::sync::{Arc, Mutex};

    // 在使用线程时，我们需要将数据移入线程内，但是一旦移入，数据就不可用了，所以使用引用计数容器Arc共享所有权
    // 同时通过Mutex来保证独占访问

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // 拿到锁
            let mut num = counter.lock().unwrap();
            // 修改数据
            *num += 1;

            // 锁释放
            // lock 调用会一个叫做 MutexGuard 的智能指针
            // 这个智能指针实现了 Deref 和 Drop trait
            // 可以自动解引用以及丢弃值
            // 此处自动调用了 drop()
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    // 3 使用原子类型共享数据

    use std::sync::atomic::{compiler_fence, AtomicBool};

    // 创建一个AtomicBool型的自旋锁，通过Arc包裹以在多线程之间共享
    let spin_lock = Arc::new(AtomicBool::new(false));

    // 创建两个引用到同一个自旋锁的克隆
    let spin_lock_clone = Arc::clone(&spin_lock);
    let sc = Arc::clone(&spin_lock);

    let thread = thread::spawn(move || {
        // 用SeqCst内存顺序将锁状态设为true，表示该锁被占用。SeqCst可以确保此操作对所有线程立即可见，
        // 即无论其他线程在何处，他们都能看到这个改变
        spin_lock_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        println!("spin_lock status a {:?}", sc);

        // 休眠2秒
        let time = std::time::Duration::from_secs(2);
        std::thread::sleep(time);

        // 设置一个编译器栅栏，内存顺序是Release。这意味着这个栅栏之前的所有操作（包括上面的println!和sleep）都会在这个栅栏之前完成。
        // Release语义：保证所有在此之前的操作都先执行完毕，确保在你更改共享数据之前，所有其他线程对这个数据的引用都已经完成
        compiler_fence(std::sync::atomic::Ordering::Release);

        // 使用SeqCst内存顺序将锁状态设为false，表示锁已经释放。SeqCst可以保证这个操作对所有线程立即可见
        spin_lock_clone.store(false, std::sync::atomic::Ordering::SeqCst);
        println!("spin_lock status b {:?}", sc);
    });

    // 主线程在这里会持续检查自旋锁的状态，只要锁的值为true（被占用），就会等待。
    // 这里也使用SeqCst内存顺序来保证锁状态的读取能在多线程中同步
    while spin_lock.load(std::sync::atomic::Ordering::SeqCst) == true {
        println!("spin_lock status c {:?}", spin_lock)
    }

    println!("spin_lock status d {:?}", spin_lock);

    if let Err(e) = thread.join() {
        println!("Thread had an error {:?}", e);
    }
}

```
*/

pub fn share_resource() {
    println!("");
}
