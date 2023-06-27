//! 2.1 `Box<T>`
//!

/**

```
// 1 Box<T> 与数据分配

// 在Rust中，你可以使用Box将数据强行存储到堆上

let a = Box::new("rust");
let b = Box::new(42);

// 它也是唯一可以将数据放到堆上的途径

// 2 Box<T> 是一个智能指针
// 它实现了Deref和Drop trait

let s = Box::new("rust");
let s = *s; // 解引用

// 离开作用域时，会自动调用drop方法，释放堆上的数据

// 这个类型比较简单，再次需要强调的是它是众多的Rust基于结构体构和trait造的特殊类型之一

// 3 为什么要把数据存放在堆上？一个链表例子

// 定义链表节点数据结构
enum ListNode<T> {
    Cons(T, Box<ListNode<T>>),
    Nil,
}
// 声明三个节点
let node3 = ListNode::Cons(3, Box::new(ListNode::Nil));
let node2 = ListNode::Cons(2, Box::new(node3));
let list = ListNode::Cons(1, Box::new(node2));

// let list: ListNode<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

// 使用模式匹配解构节点值
match list {
    ListNode::Cons(head, tail) => {
        println!("head: {}", head);
        match *tail {
            ListNode::Cons(head, _) => println!("second item: {}", head),
            ListNode::Nil => println!("there is no second item"),
        }
    }
    ListNode::Nil => println!("list is empty"),
}

```
*/

pub fn boxs() {
    println!("");
}
