//! 资源共享
//!

/**

```

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

    // 使用原子类型创建一个锁，通过引用计数获得共享所有权
    let spin_lock = Arc::new(AtomicBool::new(false));

    // 引用计数 +1
    let spin_lock_clone = Arc::clone(&spin_lock);

    let sc = Arc::clone(&spin_lock);

    let thread = thread::spawn(move || {

        // 写操作，并指定内存顺序 release 语义：写屏障之前的读写操作不能重排在写屏障之后
        spin_lock_clone.store(true, std::sync::atomic::Ordering::SeqCst);

        println!("spin_lock status a {:?}", sc);
        // 休眠
        let time = std::time::Duration::from_secs(2);
        std::thread::sleep(time);

        compiler_fence(std::sync::atomic::Ordering::Release);
        // 写操作， 并指定内存顺序 release 语义：写屏障之前的读写操作不能重排在写屏障之后
        // 上面有一个写操作，并且下面的指令要求不能在此之后
        spin_lock_clone.store(false, std::sync::atomic::Ordering::SeqCst);
        println!("spin_lock status b {:?}", sc);
    });

    // 读操作 指定内存顺序 acquire 语义 读屏障之后的读写操作不能重排到读写屏障之前
    // 上面的线程中有两条写指令，下面的指令要求之后的读写操作不能在此之前
    while spin_lock.load(std::sync::atomic::Ordering::SeqCst) == false {
        println!("spin_lock status c {:?}", spin_lock)
    }

    println!("spin_lock status d {:?}", spin_lock);

    if let Err(e) = thread.join() {
        println!("Thread had an error {:?}", e);
    }


```
*/

pub fn share_resource() {
    println!("");
}
