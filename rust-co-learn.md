# 文档说明

本文档的行文逻辑：先介绍一些基础知识，然后再层层递进。原则是尽量不在前面内容中引入比较高阶的知识点。但这样有一个坏处是一个知识点不能一次性讲透，比如函数，我们在模块一中只介绍了它的声明和样式，在模块二才介绍参数参数的生命周期、返回值类型和 trait，在模块三还在介绍它和闭包的关系等等。这样的取舍是为了不在前面就引入后面的内容，增加理解难度，但是本质上 Rust 语言的各种特性都是深度嵌合在一起的。所以在学习时读者可以根据自己的个人习惯学习，你可以尝试自行扩展，一次搞清楚某个知识点的所有情况，也可以先搞明白一部分，后面再不断补充和完善

本文档的知识深度：总体而言，本文档主要是从应用出发介绍 Rust 的各个知识点，对核心原理没有做过多的深入介绍，希望深入的朋友可以参阅其他资料

本文档的扩展资料：为了照顾到部分朋友觉得前期太少或者不够深入，所以列出了扩展资料，有兴趣的朋友可以深入阅读

# 模块一：初识 Rust

## 1.1 安装 Rust

#### 1.1.1 安装 Rust

`rustup`是一个管理 Rust 版本以及相关工具的命令行工具，你可以通过它来安装 Rust 开发环境

[在 Linux 或 macOS 上安装 `rustup`](https://rustwiki.org/zh-CN/book/ch01-01-installation.html#在-linux-或-macos-上安装-rustup)

[在 Windows 上安装 `rustup`](https://rustwiki.org/zh-CN/book/ch01-01-installation.html#在-windows-上安装-rustup)

#### 1.1.2 更新和卸载

```bash
rustup update # 更新
```

```bash
rustup self uninstall # 卸载
```

#### 1.1.3 rustc

rustc 是 Rust 的编译器，如下是一些使用案例：

**查看 Rust 版本**

```bash
rustc --version # 查看已安装的Rust的版本
```

**编译 Rust 代码**

使用 rustc 来直接编译代码为二进制程序，然后运行，例如：

```bash
mkdir module-one # 随便创建一个文件夹
cd module-one # 进入文件夹
touch main.rs # 随便创建一个.rs结尾的文件
```

rust-co-learn/module-one/main.rs

```rust
// 写一个简单的main函数
fn main() {
    println!("Hello Rust")
}
```

**编译和运行**

```bash
rustc main.rs
ls # 列出当前目录下的所有文件
main    main.rs # `main`为编译后的可执行程序
./main # 使用 `./filename` 直接运行程序
Hello Rust # 输出结果
```

**扩展资料**

1. [官方文档关于 rustc 的介绍](https://rustwiki.org/zh-CN/book/ch01-02-hello-world.html)

## 1.2 使用 Cargo

`Cargo`是 Rust 的构建系统和包管理器,类似于 python 的包管理工具 pip，但比它更强大。可以用它来创建、编译和运行 Rust 项目

以下是一个例子：

```bash
➜  module-one git:(main) ✗ cargo new hello_rust --lib # 创建一个 library crate
     Created library `hello_rust` package
➜  module-one git:(main) ✗ cargo new hello_cargo --bin # 创建一个 binary crate
     Created binary (application) `hello_cargo` package
➜  module-one git:(main) ✗ ls
hello_cargo hello_rust
➜  module-one git:(main) ✗ cd hello_cargo
➜  hello_cargo git:(main) ✗ ls
Cargo.toml src
➜  hello_cargo git:(main) ✗ cargo build --release # 编译，这里不再使用 rustc 了
➜  hello_cargo git:(main) ✗ cargo run # 编译+运行
Hello, world!
```

**扩展资料**

1. [官方文档中使用 Cargo 构建项目的详细介绍](https://rustwiki.org/zh-CN/book/ch01-03-hello-cargo.html)
2. [Cargo Book：Cargo 使用大全](https://doc.rust-lang.org/cargo/)
3. [Rust 中的 crate 以及如何使用 Cargo 管理包含了多个 crate 的项目](https://zhuanlan.zhihu.com/p/614506900)

## 1.3 Rust 基础知识

### 1.3.1 变量和可变性

```rust
 // 1 常量
    // 使用 const 声明; 常量名称使用大写字母; 显式标注类型

    const RUST: &str = "rust";
    const WEIGHT: u64 = 100;

    println!("{},{}",RUST,WEIGHT);

    // 2 变量
    // 使用let 声明,大多数情况下，编译器可以根据上下文推断变量类型

    let name = "rust";
    let age: u32 = 13;

    println!("{},{}", name, age);

    let s = "32";

    let s_to_i32 = s.parse::<i32>().unwrap();
    let s_to_u32 = s.parse::<u32>().unwrap();

    let s_to_unknown = s.parse::<i64>().unwrap();

    print!("{},{}:", s_to_i32, s_to_u32);

    // 3 不变性
    // Rust中变量默认不可变，若需修改变量，需要使用mut关键字声明变量具有可变性

    let _language = "go";
    // _language = "rust"; 无法修改

    // 4 可变性
    // 通过 mut 声明变量

    let mut language = "go";
    language = "rust";

    println!("{}", language);

    // 5 变量遮蔽
    let language = 32;
    println!("{}", language);
```

**扩展资料**

1. [官方文档关于变量遮蔽的介绍](https://rustwiki.org/zh-CN/book/ch03-01-variables-and-mutability.html)
2. [通过可变容器让变量获得可变性](https://zhuanlan.zhihu.com/p/611487702)

### 1.3.2 基础数据类型

Rust 是强类型语言，每个值都有确切的类型

#### 标量类型

```rust
 // 1 整数类型

    // Rust 中整数类型分为有符号和无符号类型；长度分为8位，16位，32位，64位，128位
    // 特殊的整数类型: usize 和 isize，与系统架构相关，32位的系统则为32位，64位的系统为64位

    let integer: i32 = 42;
    let s: usize = 100;

    // 2 浮点类型
    // Rust 的浮点型是 f32 和 f64，大小分别为 32 位和 64 位。默认浮点类型是 f64
    // 浮点型都是有符号的

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 3 布尔类型

    let t = true;
    let f: bool = false;

    // 4 字符类型 char
    // Rust 的字符类型大小为 4 个字节，表示的是一个 Unicode 标量值

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let char_size = std::mem::size_of::<char>();
```

**扩展资料**

1. [Rust 官方文档关于基础类型的详细介绍](https://rustwiki.org/zh-CN/book/ch03-02-data-types.html)

#### 复合类型

Rust 中的复合类型主要有元组和数组

```rust
 // 1 元组
    // Rust中的元组可以将各种类型组合起来
    let types = (42, "Rust", true);

    // 可以通过下标索引访问
    println!("num is {}", types.0);

    // 单元类型 （）
    // 单元类型在Rust中是非常重要的类型，如果表达式不返回任何其他值，就隐式地返回单元值，
    // 如没有返回值的函数或者作用域

    let a: () = {};
    fn return_tuple() {}
    let func: () = return_tuple();
    assert_eq!(a, func);

    // 2 数组
    // 通过索引来访问或者修改数组中的元素

    let arr = [1, 2, 3, 4, 5];

    let mut arr1 = [0, 0, 0, 0, 0];
    arr1[0] = 100;
    println!("{:?}", arr1); // [100, 0, 0, 0, 0]
```

**扩展资料**

1. [官方文档中关于复合类型的介绍](https://rustwiki.org/zh-CN/book/ch03-02-data-types.html)
2. [关于 Rust 中类型与表达式的详细介绍，此课程为付费课程，但是强烈推荐](https://time.geekbang.org/course/detail/100060601-289993)

### 1.3.3 进阶数据类型

#### 字符串

Rust 中的字符串比较复杂，有多种形式，适用于不同的场景。核心是需要掌握 `&str` 和 `String`

Rust 在编译代码时需要在编译期就能够确定类型的大小，而字符串 str 本身是动态大小的，因而日常中我们更多使用的是字符串的引用 `&str` 和 `String`

```rust
    // 1 &str：字符串字面量的引用
    // 字符串字面量实际上存放在程序的只读数据段中，在程序运行时会被加载到内存中读取
    let s = "Hello Rust";
    let mut s1 = "Hello Go";

    s1 = "Hello Rust";
    println!("{}", s1);

    // 2 String：字符串切片的引用
    // String 通过指针指向存放在堆上的字符串

		// 有多种方式可以在堆上创建字符串
    // let s2 = String::new();         // 空字符串
    // let s2 = "Hello Rust".to_string();
    // let s2: String = "Hello Rust".into();
    // let s2 = String::from("Hello Rust");

    // 可以使用ptr、len、cap获取String的指针、长度和容量

    let cap = s2.capacity();
    let len = s2.len();
    let ptr = s2.as_ptr();

    println!("len {}", len);
    println!("cap {}", cap);
    println!("pointer {:?}", ptr);

    // 3 字符串切片
    // 字符串本质上一个u8序列，支持切片操作

    let s1 = String::from("Hello Rust");
    let s2 = "Hello Rust";

    let slice1 = &s1[0..5];
    let slice2 = &s2[6..10];

    println!("slice1: {}", slice1); // Hello
    println!("slice2: {}", slice2); // Rust
```

**扩展资料**

1. [官方文档中关于字符串的更多解释](https://rustwiki.org/zh-CN/book/ch08-02-strings.html)
2. [一些字符串练习的小例子](https://github.com/rust-lang-cn/rustlings-cn/tree/main/exercises/strings)
3. [官方文档中关于切片的更多内容](https://rustwiki.org/zh-CN/book/ch04-03-slices.html)

#### 引用

Rust 中的引用类型是一等公民，并且和借用指同一个概念。从可变性上可以分为可变引用和不可变引用

```rust
    // 1 不可变借用
    let num = 42;
    let immutable_s = &num;

    // 2 可变借用
    let mut num = 42;
    let mutable_s = &mut num;

    // 当类型占用空间比较大时，可以通过引用来访问或者修改数据,尤其是在传递数据的场景下

    let person_tuple = ("Rust", 13, true);

    let ptr = &person_tuple;
    println!("{}", ptr.0);

    let mut arr = ["Rust", "Go", "C++"];

    let arr_ptr = &mut arr;

    arr_ptr[2] = "Java";

    println!("{:?}", arr_ptr) // ["Rust", "Go", "Java"]
```

**扩展资料**

1.[Rust 中引用和指针的区别](https://zhuanlan.zhihu.com/p/614970269)

2.[官方文档中对引用的更多介绍](https://rustwiki.org/zh-CN/book/ch04-02-references-and-borrowing.html)

#### 集合

两个重要的集合 Vec 和 HashMap，这里的集合指的是它们都聚集了多个同类型的元素

```rust
    // 1 Vec
    // Vec是动态大小，相比起数组来说，它更加常用
    // Vec中的元素必须相同

    let mut vec1 = Vec::new();
    let mut vec2 = vec![];

    // vec 支持一系列的操作

    // 添加元素
    vec1.push("Rust");
    vec2.push("Go");

    // 当作栈
    vec1.pop();

    // 修改数据
    vec2[0] = "Rust";

    // Vec 和 String一样，数据存放在堆上

    println!("{}", vec2.len()); // 1
    println!("{}", vec2.capacity()); // 4
    println!("{:?}", vec2.as_ptr()); // 0x7fafc9f05b70

    // 2 HashMap

    // HashMap并不是预导入的，需要手动引入当前作用域
    use std::collections::HashMap;

    // 使用new方法创建
    let mut scores = HashMap::new();

    // 插入数据
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 修改数据,修改和插入数据是同一个api
    scores.insert(String::from("Blue"), 100);

    // 访问数据,注意访问的key传递的是引用
    let key = String::from("Blue");
    println!("{:?}", scores[&key])
```

**扩展资料**

1. [官方文档中对集合的更多介绍](https://rustwiki.org/zh-CN/book/ch08-00-common-collections.html)

#### 结构体

```rust
    // 1 结构体
    // Rust中的结构体有三种

    // 1.1 常规结构体
    struct Language {
        name: String,
        birth: u32,
        is_popular: bool,
    }

    // 1.2 元组结构体
    struct Rust(String);

    // 1.3 单元结构体
    struct Go;

    // 2 为结构体实现方法
    impl Rust {
        // Self 代表结构体本身
        fn new() -> Self {
            Rust(String::from("Rust"))
        }

        fn print() {
            let rust = Rust::new();
            println!("{:?}", rust.0)
        }
    }

    // 3 方法调用
    let r = Rust::new();

    Rust::print();

    // 4 访问结构体成员
    println!("{:?}", r.0)
```

**扩展资料**

1. [官方文档对于结构体的介绍](https://rustwiki.org/zh-CN/book/ch05-00-structs.html)

#### 枚举

```rust
    // 枚举在形式上和结构体较为相似
    enum Subject {
        Math,
        Chinese,
        English(String),
    }

    // 初始化
    let subject = Subject::English(String::from("English"));

    //标准库中两个比较重要的枚举 Option和 Result

    // Result 用于一些处操作可能遇到错误的场景，比如打开文件时，如果成功，返回文件，遇到错误时返回一个Error
    use std::fs::File;

    let file: Result<File, std::io::Error> = File::open("tmp.txt");

    // Option 用于一些值可能为空的情况
    // 如尝试获取哈希表中某个key所对应的value，当值存在时，返回值，当不存在时返回None

    use std::collections::HashMap;

    let map: HashMap<&str, u32> = HashMap::new();
    let v: Option<&u32> = map.get("rust");
```

**扩展资料**

1. [官方文档中对于枚举的介绍](https://rustwiki.org/zh-CN/book/ch06-01-defining-an-enum.html)

#### 函数

```rust
    // 1 函数定义
    // 1.1 没有参数和返回值的函数
    fn foo() {
        println!("foo")
    }

    // 1.2 有参数和返回值的函数

    fn bar(s: &str) -> String {
        String::from(s)
    }

    // 1.3 参数类型必须显式声明，比如引用或者可变性

    fn foobar(mut s: &str) -> &str {
        s = "rust";
        s
    }

    // 2 函数调用

    foo();
    bar("Rust");
    foobar("go");

    // 3 函数作为参数

    fn a(f: fn() -> u32) -> u32 {
        let value = f();

        value
    }

    fn b() -> u32 {
        42
    }

    // 把函数作为参传给另一个函数

    a(b);
```

**扩展资料**

1.[官方文档中关于函数的介绍](https://rustwiki.org/zh-CN/book/ch03-03-how-functions-work.html)

#### 闭包

```rust
    // 1 闭包定义

    // 闭包可以捕获环境变量,并且根据其对环境变量的操作可以分为以下三类

    let c1 = || print!("未捕获环境变量");

    let v = "rust";
    let c2 = || print!("捕获环境变量但不修改 {}", v);

    let mut s0 = String::from("hello");

    // 闭包的参数写在 ｜｜ 中

    let mut c3 = |s: String| {
        s0 = s + v;
        println!("捕获并修改环境变量 {:?}", s0)
    };

    // 2 闭包的调用

    // 闭包的调用同函数一样

    c1();
    c2();
    c3(String::from("rust"));
```

**扩展资料**

1.[官方文档中关于闭包的介绍](https://rustwiki.org/zh-CN/book/ch13-01-closures.html)

#### 泛型

Rust 语言支持泛型编程，在实际操作中会大量涉及到泛型。泛型提供了抽象能力，让代码复用性更强。泛型一般和其它数据结构结合使用

```rust
    // 1 泛型参数的表示

    // 泛型参数一般用大写字母`T`表示,多个泛型参数可以使用多个大写字母

    // 学习泛型时可以把泛型当作自定义类型，它必须先声明才能使用

    // 2 泛型如何使用

    // 2.1 集合 Vec<T>
    // 集合vector就是由泛型提供支持的,它允许我们使用某个具体类型时再指定

    let v1: Vec<u8> = Vec::new();
    let v2: Vec<String> = Vec::new();
    let v3: Vec<bool> = Vec::new();

    // 2.2 泛型结构体

    // 可以声明一个泛型结构体，然后再使用的时候在指定成员的具体类型
    // 注意：必须先在` <> `中声明泛型参数，然后才能使用

    struct Type<T>(T);
    struct Point<A, B> {
        a: A,
        b: B,
    }

    let t1 = Type(42);
    let t2 = Type("rust");

    let p1 = Point { a: 42, b: 42 };
    let p2 = Point { a: 42.1, b: 42.1 };

    // 为泛型结构体实现方法
    // 注意：为泛型结构体实现方法时，impl和结构体后面的泛型声明要保持一致
    impl<A, B> Point<A, B> {
        fn new(a: A, b: B) -> Self {
            Point { a, b }
        }
    }

    // 2.3 泛型枚举

    // 同样，可以定义泛型枚举

    enum Area<A, B, C> {
        Rectangle(A),
        Square(B),
        Circle(C),
    }

    let a1: Area<f64, u32, &str> = Area::Rectangle(42f64);
    let a2: Area<f32, u64, &str> = Area::Square(42u64);
    let a3: Area<f64, u32, &str> = Area::Circle("100 cm^2");

    // 2.4 泛型函数

    // 函数参数也可以是泛型, 当然泛型也需要在 `<>` 中先声明

    fn generics<T, B>(a: T, b: B) -> T {
        a
    }
    generics(32, "rust");
    generics("rust", 32);
```

**扩展资料**

1.[官方文档中关于泛型的介绍](https://rustwiki.org/zh-CN/book/ch10-00-generics.html)

### 1.3.4 控制流

Rust 程序在书写上并没有太复杂的结构，循环和模式匹配基本能够满足绝大多数场景需求

#### 循环

Rust 有三种循环结构 for 循环，loop 和 while

```rust
    // 1 使用for循环遍历集合
    // 注意：Rust中的for循环本质上是迭代器的语法糖，这个我们后面还会再介绍
    // 迭代器涉及到更复杂的知识点，后续会介绍

    let v = vec![1, 2, 3, 4, 5];

    for num in v {
        println!("{}", num);
    }

    let mut map = std::collections::HashMap::new();
    map.insert("a", 2);
    map.insert("b", 2);
    map.insert("c", 2);

    for kv_pair in map {
        println!("{:?}", kv_pair);
    }

    // 2 使用 loop 执行无限循环，并使用break终止

    let mut x = 0;

    loop {
        println!("{:?}", x);

        if x == 10 {
            break;
        } else {
            x = x + 1;
        }
    }

    // 3 使用 while 执行条件循环

    let mut x = 0;
    while x < 5 {
        println!("x is {}", x);
        x += 1;
    }
```

#### 模式匹配

Rust 中的模式匹配指的是结构上的匹配，最常用有 match、while let 、let 、if let

```rust
    // 1 match
    // match 是最长用的模式匹配，主要和枚举搭配使用，以匹配不同的枚举成员

    match std::fs::File::open("rust.txtr") {
        Ok(file) => println!("{:?}", file),
        Err(err) => panic!("{}", err),
    }

    // 2 if let
    // if let 可以让我们只关注我们想要的结果

    if let Ok(file) = std::fs::File::open("rust.txtr") {
        println!("{:?}", file);
    }

    // 3 while let
    // 和 if let 类似，只处理正确的结果

    while let Ok(file) = std::fs::File::open("rust.txt") {
        println!("{:?}", file);
    }

    // 4 let
    // let 本身也是一种模式匹配
    // 使用 let 匹配元组中的元素

    let tuple = (42, true, "rust");

    let (x, y, z) = tuple;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
```

### 1.3.5 注释

Rust 中的注释主要包括文档注释，多行注释和单行注释

```rust
  /// 1. 文档注释,一般写在当前文件的最顶端

  fn main() {
      /*
         2. 多行注释
         Point 结构体代表二维坐标系下的一个点，通过坐标可以求的任意一点到原点的距离
      */
      struct Point(u32, u32);

      // 3. 单行注释
      // 求某一点到原点距离的平方

      fn distance_square(p: Point) -> u32 {
          p.0 * p.0 + p.1 * p.1
      }

      let p = Point(3, 4);
      distance_square(p);
  }
```

**扩展资料**

1.[官方文档中关于注释的介绍](https://rustwiki.org/zh-CN/book/ch03-04-comments.html)

## 1.4 课后习题

1. `rustup`是什么，如何用它来管理 Rust 版本？
2. Rust 中`&str`和`String`的区别是什么，每个应该在什么时候使用？
3. Rust 中的泛型类型是什么，你可以自己写几个例子吗？
4. Rust 中使用泛型类型的一些常见数据结构有哪些？
5. Rust 中有哪三种循环结构，它们如何使用？
6. 在 Rust 中，`match`、`while let`、`let`和`if let`之间的区别是什么，每个应该在什么时候使用？
7. Rust 中有哪三种类型的注释？

# 模块二：Rust 核心知识

**前置知识：编程语言都是如何管理内存的？**

手动管理内存，代表语言：C、C++，如下是一段 C++代码

```c++
#include<iostream>

int main(){
    // 使用 new 分配内存
    int* myInt = new int;

    // 检查内存是否成功分配
    if(myInt == nullptr){
        std::cout << "Memory not allocated.\n";
        return 1;
    }

    // 在分配的内存中存储一个值
    *myInt = 10;
    std::cout << "Value of myInt: " << *myInt << "\n";

    // 使用 delete 释放内存
    delete myInt;
    myInt = nullptr; // 防止产生悬空指针

    // 检查内存是否已被释放
    if(myInt == nullptr){
        std::cout << "Memory successfully freed.\n";
    }

    return 0;
}
```

弊端：开发者心智负担重，未及时释放内存可能导致内存奔溃，未正确释放内存可能导致悬垂指针问题，好处：程序性能优越

自动管理内存（使用垃圾回收机制），代表语言：Java / GO，如下是一段 Java 代码

```java
public class Main {
     public static void main(String[] args) {
         // 创建一个新的对象
         MyClass obj = new MyClass();

         // 使用该对象
         obj.printMessage();

         // obj 现在自然离开了作用域，GC 在适当的时候会自动清理这个对象所占用的内存, GC在看不见的地方运行
         System.out.println("End of main method. The JVM will clean up the memory automatically when necessary.");
     }
 }

 class MyClass {
     public void printMessage() {
         System.out.println("Hello, World!");
     }
 }
```

弊端：程序性能低、内存开销大、世界暂停，好处是开发者心智负担小，轻松易上手

其它内存管理方案（比如 Rust 所有权机制与检查规则管理），代表语言：Rust，如下是 Rust 代码

```rust
fn main() {
    // 分配内存
    // 所有者是language,值是分配在堆上的str
    let language = String::from("Chinese");

    {
        //分配内存
        // x 也是一个所有者，它的值存放在程序的只读内存中
        let x = "x";

        // 当x离开它的作用域时，它对应的值（内存）也会被释放
    }

    // print!("x:{}", x); // 无法使用 x, 因为x已经离开了它的作用域

    println!("Language: {}", language);

    // 当main函数结束，language（所有者）离开其作用域，对应的值也就被释放了
    // 所有权的内存管理逻辑：“人在值在”（即所有者在，则其对应的值（内存）就在，所有者不在某个作用域了，那值（内存）也就被释放了，这就把管理内存的问题转变为管理所有者所有权的问题了，所有权机制也就这么来了
}
```

弊端：需要理解所有权与借用检查规则，好处：一旦上手，轻松无负担、程序性能优越

Rust 是无 GC（garbage collection）的语言，对于堆内存的管理主要通过栈来实现。具体而言就是通过借用检查和所有权机制来实现。核心规则如下

**所有权机制：**

1. 每个值都有一个所有者（owner）（唯一性，大多数情况下所有者和值一一对应）
2. 每个值在任一时刻只有一个所有者 （持续性，任何情况下值都只有一个所有者）
3. 当所有者（变量）离开作用域时，它所拥有的值将被丢弃 （内存管理，值和所有者绑定，随所有者“离开”而“释放”）

**Rust 的借用规则：**

1. 同一个作用域中，一个资源只有一个可变**借用**（&mut T），也就是说拥有可变**借用**（&mut T）后就不能再拥有不可变**借用**（&T）

2. 同一个作用域中，一个资源可以有多个不可变**借用**（&T）
3. **借用**在它离开作用域后会被释放

## 2.1 数据类型与所有权

### 2.1.1 固定类型与所有权

固定尺寸类型是指那些在编译期就可以确定大小的类型。Rust 中主要的固定尺寸类型如下：

| 类型     | 描述                             |
| -------- | -------------------------------- |
| 基本类型 | 整数、浮点数、布尔值和字符类型等 |
| 复合类型 | 数组、元组等                     |
| 指针类型 | 引用和裸指针、函数指针等         |
| ...      | ...                              |

固定大小类型：一旦声明：地址和大小不可再变

```
fn main() {
    let mut a: i32 = 32;

    // 获取 'a' 变量的大小（改变前）
    let size_before_a = std::mem::size_of_val(&a);
    println!("Size of 'a' before change: {} bytes", size_before_a); // 打印结果：Size of 'a' before change: 4 bytes

    // 打印 'a' 变量的地址（改变前）
    let address_before_a = &a as *const i32;
    println!("Address of 'a' before change: {:?}", address_before_a);

    a = 64;

    // 获取 'a' 变量的大小（改变后）
    let size_after_a = std::mem::size_of_val(&a);
    println!("Size of 'a' after change: {} bytes", size_after_a); // 打印结果：Size of 'a' after change: 4 bytes

    // 打印 'a' 变量的地址（改变后）
    let address_after_a = &a as *const i32;
    println!("Address of 'a' after change: {:?}", address_after_a);

    let mut t = ('a', 32, true, 42.1);

    // 获取 't' 变量的大小（改变前）
    let size_before_t = std::mem::size_of_val(&t);
    println!("Size of 't' before change: {} bytes", size_before_t); // 打印结果：Size of 't' before change: 24 bytes

    // 打印 't' 变量的地址（改变前）
    let address_before_t = &t as *const (_, _, _, _);
    println!("Address of 't' before change: {:?}", address_before_t);

    t = ('b', 64, false, 84.2);

    // 获取 't' 变量的大小（改变后）
    let size_after_t = std::mem::size_of_val(&t);
    println!("Size of 't' after change: {} bytes", size_after_t); // 打印结果：Size of 't' after change: 24 bytes

    // 打印 't' 变量的地址（改变后）
    let address_after_t = &t as *const (_, _, _, _);
    println!("Address of 't' after change: {:?}", address_after_t);
}
```

其它的固定大小类型实例

```
use std::mem::size_of;

fn main() {
    // 整数类型
    let int_var: i32 = 10;
    println!("Size of integer: {}", size_of::<i32>());  // 打印结果：Size of integer: 4

    // 浮点数类型
    let float_var: f64 = 10.0;
    println!("Size of float: {}", size_of::<f64>());  // 打印结果：Size of float: 8

    // 布尔类型
    let bool_var: bool = true;
    println!("Size of bool: {}", size_of::<bool>());  // 打印结果：Size of bool: 1

    // 字符类型
    let char_var: char = 'a';
    println!("Size of char: {}", size_of::<char>());  // 打印结果：Size of char: 4

    // 数组类型
    let array_var: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Size of array: {}", size_of::<[i32; 5]>());  // 打印结果：Size of array: 20

    // 元组类型
    let tuple_var: (i32, f64, bool, char) = (10, 10.0, true, 'a');
    println!("Size of tuple: {}", size_of::<(i32, f64, bool, char)>());  // 打印结果：Size of tuple: 24

    // 引用类型
    let ref_var: &i32 = &10;
    println!("Size of reference: {}", size_of::<&i32>());  // 打印结果：Size of reference: 8

    // 裸指针类型
    let raw_pointer_var: *const i32 = &10;
    println!("Size of raw pointer: {}", size_of::<*const i32>());  // 打印结果：Size of raw pointer: 8

    // 函数指针类型
    let fn_pointer_var: fn() = foo;
    println!("Size of function pointer: {}", size_of::<fn()>());  // 打印结果：Size of function pointer: 8
}

fn foo() {
    println!("This is a function.");
}
```

所有权规则在固定大小类型代码中的体现：将一个变量作为值赋值给另一个变量时，发生值拷贝行为

```

   // 1 所有权与基本类型

    // 下面的每个值都只有一个所有者

    let num1 = 42;

    let num2 = num1; // num2是一个新的所有者，它的值是 num1值的复制品，num1仍然是一个有效的所有者
    println!("{}", num1); // 42,可以通过 num1 使用值

    // 现在有两个值和对应的两个有效的所有者，num1 和 num2

    println!("num1 addr {}", num1);
    println!("num2 addr {}", num2);

    // 可以看到值的地址也是不相同（佐证num1和num2各拥有一个值）
    // 对于值42来说，它只有一个所有者，因此现在有两个42的值，并且它们的地址是不同的

    println!("num1 addr {:p}", &num1); // 0x7ff7b404dd90
    println!("num2 addr {:p}", &num2); // 0x7ff7b404dd94

    let f1 = 42.0;
    let b1 = true;

    {
        let c1 = '4'; // ‘4’ 这个值的所有者 `c1` 在离开作用域时，值会被丢弃

        let c2 = c1;

        println!("c1 addr {:p}", &c1); // 0x7ff7b404dde8
        println!("c2 addr {:p}", &c2); // 0x7ff7b404ddec
    }

    // println!("{}", c1) // 无法再使用 owner4,因为它已经被丢弃

    // 2 所有权与复合类型

    let arr1: [i32; 3] = [1, 2, 3];
    let arr2 = arr1;

    println!("arr1 addr {:p}", &arr1); // 0x7ff7b404dd90
    println!("arr2 addr {:p}", &arr2); // 0x7ff7b404dd94

    let tuple1 = (32, true, 42.0);
    let tuple2 = tuple1;

    println!("tuple1 addr {:p}", &tuple1); // 0x7ff7b404dd91
    println!("tuple2 addr {:p}", &tuple2); // 0x7ff7b404dd93

    // 3 所有权与指针类型

    // 这里所说的指针是指指向某个内存地址的变量类型，包括引用、裸指针以及函数指针

    // 3.1 引用
    let num = 21;

    let p1 = &num;
    let p2 = p1;

    println!("num {}", &num);

    println!("p1 addr {:p}", &p1); // 0x7ff7ba58c938
    println!("p2 addr {:p}", &p2); // 0x7ff7ba58c940

    // 3.1 裸指针
    // 在rust中使用 `as *const T` 可以将引用转为裸指针

    let num = 32;

    let r_p1 = &num as *const i32;
    let r_p2 = r_p1;

    println!("r_p1 addr {:p}", &r_p1); // 0x7ff7b9ecc940
    println!("r_p2 addr {:p}", &r_p2); // 0x7ff7b9ecc948

    // 3.2 函数指针
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let f_p1: fn(i32, i32) -> i32 = add;
    let f_p2 = f_p1;

    println!("f_p1 addr {:p}", &f_p1); // 0x7ff7b606f9d0
    println!("f_p2 addr {:p}", &f_p2); // 0x7ff7b606f9d8

    // 总结：对于固定大小类型而言，将一个变量赋值给另一个变量时，实际上是为新变量开辟了新的内存空间，并把值拷贝过来，也就Copy语义
    // 最终的结果是产生一个新的所有者以及对应的值，并且新变量和原变量互不影响
```

### 2.1.2 动态类型与所有权

Rust 是一门静态类型语言，这意味着所有变量在编译期必须是大小确定的，但是在实际场景中，比如字符串和切片类型的大小取决于运行时的具体情况。Rust 对这类数据的处理方法是使用它们的指针（引用），而不是数据本身，众所周知，一个类型不管多大，对应的指针（引用）大小是确定的

| 类型         | 描述                                                                                       |
| ------------ | ------------------------------------------------------------------------------------------ |
| 字符串类型   | str, 本质上是一个 u8 类型的数据序列，实际中经常使用的形式：&str 和 String                  |
| 切片类型     | [T], 它代表类型为 `T` 的元素组成的数据序列：实际中经常使用的形式： Vec<T>                  |
| trait object | trait object 的大小只有在运行时才能确定（可以先不用了解，关于 trait 的内容后面会继续讲解） |
| ...          | ...                                                                                        |

动态类型大小，声明后再次修改时，大小和地址可能会动态变化

```
fn main() {
    // 1. &str
    let mut a = "rust";
    let size_of_a = std::mem::size_of_val(a);
    let ptr_of_a = a.as_ptr();

    println!("Size of 'rust': {} bytes", size_of_a); // 打印结果：Size of 'rust': 4 bytes
    println!("Address of 'rust': {:p}", ptr_of_a); // 打印结果：Address of 'rust': 0x107e52fa0

    a = "go";
    let size_of_a = std::mem::size_of_val(a);
    let ptr_of_a = a.as_ptr();

    println!("Size of 'go': {} bytes", size_of_a); // 打印结果：Size of 'go': 2 bytes
    println!("Address of 'go': {:p}", ptr_of_a); // 打印结果：Address of 'go': 0x107e52fdb

    let ptr_of_rust = "rust".as_ptr(); // 访问初试的“rust”
    println!("Address of 'rust' after reassignment: {:p}", ptr_of_rust); // 打印结果：Address of 'rust' after reassignment: 0x107e52fa0

    // 2 String

    let mut string_data = String::from("Hello, Rust!");
    let size_of_string = string_data.len();
    let ptr_of_string = string_data.as_ptr();

    println!("Size of string data: {} bytes", size_of_string);
    println!("Address of string data: {:p}", ptr_of_string);

    string_data = String::from("Hello Rust, how are you today?");

    let size_of_string = string_data.len();
    let ptr_of_string = string_data.as_ptr();

    println!("Size of string data: {} bytes", size_of_string);
    println!("Address of string data: {:p}", ptr_of_string);

    // 3 vec

    let mut vec_data = vec![1];
    let size_of_vec = vec_data.len();
    let ptr_of_vec = vec_data.as_ptr();

    println!("Size of vector data: {} bytes", size_of_vec);
    println!("Address of vector data: {:p}", ptr_of_vec);

    vec_data.push(2);
    vec_data.push(3);
    vec_data.push(4);
    vec_data.push(5);
    vec_data.push(6);
    vec_data.push(7);
    vec_data.push(8);
    vec_data.push(9);

    let size_of_vec = vec_data.len();
    let ptr_of_vec = vec_data.as_ptr();

    println!("Size of vector data: {} bytes", size_of_vec);
    println!("Address of vector data: {:p}", ptr_of_vec);
}
```

所有权规则在动态大小类型中的体现：将一个变量作为值赋值给另一个变量时，发生所有权转移行为

```
    // 1 所有权与字符串

    // 我们在前面介绍过，字符串可以存放在程序的只读数据段中或者堆上
    // 一般情况下，字符串字面量存放在只读数据段中的，声明之后很少去修改它
    // 而需要动态变化的字符串我们会把它存放到堆上，并且通过栈内存来管理堆内存

    let ptr_owner = "Rust"; // 存放在只读数据段中
    let heap_ptr_owner = String::from("Rust"); //存放在堆上

    // 1.1 对于存放在只读数据段中的字符串字面量，它的所有权规则和其他基本类型一样,这里不再赘述

    let ptr_copy = ptr_owner;

    // 由于 ptr_owner 和 ptr_copy 的值都是指向相同值的引用，所以它们指向的内存地址是相同的
    println!("{:p}", ptr_owner); // 0x10ac12004
    println!("{:p}", ptr_copy); // 0x10ac12004

    let mut _heap_ptr_old = String::from("Rust"); //存放在堆上

    let heap_ptr_new = _heap_ptr_old;

    // println!("old owner{:?}", _heap_ptr_old); // 无法再通过 _heap_ptr_old 使用值，因为它已经把数据所有权移交给了新的所有者 heap_ptr_new
    println!("new owner{:?}", heap_ptr_new); // heap_ptr_new 可以正常访问到堆上的数据，并且它是唯一的所有者，当它离开作用域时，堆上的数据也会被丢弃

    {
        let owner_old = String::from("rust");
        let owner_new = owner_old;

        // 在此处离开作用域
    }

    // println!("{:?}", owner_new); 无法再通过 owner_new 使用值，因为它已经被丢弃

    _heap_ptr_old = String::from("Go"); // 重新赋值，注意原变量不能使用是因为转移所有权后被标注为空了，而不是立即被清除了

    // 2 所有权与slice

    // 上面的字符串str 实际上是一个特殊的 slice, 它仅代表有效的utf-8序列
    // 而切片中可以包含任何类型的元素，如其他基础类型、自定义类型等, 正如不直接使用 str一样，我们也不直接使用[T],而是使用它的指针（引用）类型，Vec<T>
    // slice中的数据也存放在堆上，Rust中slice内存管理逻辑同存放在堆上的str

    // vec 有两种创建方式：使用宏或者方法
    let str_slice = vec!["rust", "go", "cpp"];
    let u32_slice: Vec<u32> = Vec::new();

    let new_owner1 = str_slice;
    let new_owner2 = u32_slice;

    // println!("{:?}", str_slice); // 无法再通过 str_slice 使用值，因为它已经被丢弃
    // println!("{:?}", u32_slice); // 无法再通过 u32_slice 使用值，因为它已经被丢弃

    println!("{:?}", new_owner1); // 可以通过新的所有者访问到原来的值
    println!("{:?}", new_owner2); // 可以通过新的所有者访问到原来的值

    // 3 总结
    // 当数据存放在堆上时，把所有权赋值给另一个变量，意味着把堆上所有权就会转移给新的所有者，堆上的数据本身没有被复制，原来的所有者不再拥有数据，也就Move语义，这里指的是所有权的Move
    // 当数据存放在栈上时，把所有权赋值给另一个变量，意味着把栈上的数据复制了一份给新的所有者，原来的所有者仍然拥有原来的数据，也就是Copy语义，这里指值的Copy
```

**扩展资料**

1.[官方文档中关于所有权的介绍](https://rustwiki.org/zh-CN/book/ch04-01-what-is-ownership.html)

### 2.1.3 所有权共享

所有权规则更像是对资源的独占，在实际场景中，你可能希望多个角色共享访问某个动态资源。Rust 提供了两个容器类型 Rc<T>和 Arc<T>，可以让你同时让多个变量拥有动态数据的所有权

```rust
    // 1 独占访问资源

    let mut dynamic_source = String::from("content");

    let role1 = dynamic_source;
    // let role2 = dynamic_source; // 资源被 role1 所有，此时role1独占访问
    let role2 = role1; // 只有role1 把所有权移交给 role2， role2 才可以访问

    // 这样做的好处是，可以避免资源被多个变量同时访问，导致资源被修改
    // 坏处是，资源只能被一个变量访问，低效

    use std::rc::Rc;
    use std::sync::Arc;

    // 2 所有权与共享容器 Rc<T>,它适用于单线程

    // 使用共享容器包裹动态资源

    let dynamic_source = vec![1, 2];

    let container = Rc::new(dynamic_source);

    let role1 = container.clone(); // 这里clone方法其实是复制了对资源访问的所有权，而不是资源本身
    let role2 = container.clone();

    // 通过共享容器访问资源,此时资共享资源有三个所有者，可以同时访问
    println!("{:?}", container); // [1,2]
    println!("{:?}", role1); // [1,2]
    println!("{:?}", role2); // [1,2]

    // 3 所有权共享容器 Arc<T>，它适用于多线程

    let dynamic_source = String::from("rust");

    let container = Arc::new(dynamic_source);

    let role1 = container.clone(); // 这里clone方法其实是复制了对资源访问的所有权，而不是资源本身
    let role2 = container.clone();

    // 通过共享容器访问资源,此时资共享资源有三个所有者，可以同时访问
    println!("{:?}", container); // rust
    println!("{:?}", role1); // rust
    println!("{:?}", role2); // rust

    // 4 共享容器与内存管理
    // 注意：Rc<T>和Arc<T>实际上是一种引用计数，每使用clone方法一次，引用计数就会+1，当变量离开作用域时，引用计数会-1，当引用计数为0时，堆内存会被释放
    // 整个过程在编译器看来，每个变量都拥有一个Rc或者Arc。所以并不违反所有权规则
    // 这里提一点:一般情况下，Rust使用栈来管理堆内存。但是Rc和Arc是一种特别的机制，它允许不受栈内存控制的堆内存，也就是允许内存泄露。对于这种泄漏通过引用计数来管理

    // 4.1 通过栈内存来管理堆内存

    {
        let source = String::from("hello");

        let role1 = source;
        println!("{:?}", role1);
        // 丢弃

        // println!("{:?}", source); // 不能再使用source，因为source已经移交了所有权
        // 当role1离开作用域时，会立即丢弃 role1和堆上的数据
    }

    // 4.2 通过引用计数来管理堆内存

    {
        let source = String::from("hello");

        // 使用Rc包裹资源，让堆上资源生命周期更长
        let container = Rc::new(source); // 引用计数 + 1
                                         //
        let role1 = container.clone(); // 引用计数 + 1
        let role2 = container.clone(); // 引用计数 + 1

        // 当变量离开作用域时，role2，role1，container相继离开作用域时，引用计数都会-1，当引用计数为0时，堆上的数据才会被释放
    }
```

## 2.2 借用和生命周期

### 2.2.1 借用与引用以及借用检查规则

引用和借用在Rust中不同视角下的同一个概念。Rust中任何类型的变量都遵从所有权规则，但值为引用类型的变量还有遵从借用检查规则

```rust
fn main() {
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
}
```

### 2.2.2 变量生命周期

根据是否拥有数据（值），Rust 中的变量可以分为拥有数据的变量和没有数据变量，也就是说它对应的值是一个引用

当变量值为数据时，一般Rust可以根据上下文推断这个变量在哪里声明的以及在哪里离开作用域的，然后它会通过所有权规则释放变量所对应的内存，当一个变量的值是来自于另一个值的引用时，Rust会通过借用检查器来确保所有的引用都是有效的

```rust
    //1 变量的生命周期 : 从声明开始，到离开作用域结束

    {
        let x = 32; // x 的生命周期开始
        println!("{}", x);

        {
            let x_ptr = &x;  // x_ptr 的生命周期开始
            
        } // x_ptr 生命周期结束，值会被丢弃

        // println!("{}", x_ptr); // 无法再使用 x_ptr,因为它已经被丢弃

       
    }  // x 生命周期结束，值会被丢弃
```

### 2.2.3 生命周期参数

当一个变量的值是来自于另一个值的引用时，有时候编译器并不能准确推断这个引用的有效性

```
    // 可以推断引用的有效性
    // fn return_strings(x: &String) -> &String {
    //     let s = String::from("Rust");

    //     &s
    // }

    let s_p = return_string(&String::from("Rust"));

    fn return_string(x: &String) -> &String {
        x
    }

    struct Foo {
        x: i32,
        y: (i32, bool),
        z: String,
    }

    let f1 = Foo {
        x: 32,
        y: (32, true),
        z: String::from("rust"),
    };

    let f2 = Foo {
        x: 32,
        y: (32, true),
        z: String::from("rust"),
    };

    // 仍然无法编译通过，因为编译器无法推断出参数和返回值的生命周期
    // 这是因为Rust对于函数的检查只会检查签名，而不是函数里面的具体逻辑
    // 对于这个函数返回的引用，因为只从函数签名来看，返回的这个引用不知道是来自于哪里，一个可能是来自于两个参数的某一个，另一种情况是可能来自于某个内部定义的变量，这就会导致悬垂指针

    // 无法推断引用的有效性
    // fn bar(x: &Foo, y: &Foo) -> &Foo {
    //     let f3 = Foo {
    //         x: 32,
    //         y: (32, true),
    //         z: String::from("rust"),
    //     };
    //     &f3
    // }

    // let new = bar(&f1, &f2);

    // 通过生命周期参数显示指定参数的生命周期

    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用

    let s: &'static str = "I have a static lifetime.";

    // 返回变量的生命周期和至少和y一样长
    fn bar<'a, 'b>(x: &'a Foo, y: &'b Foo) -> &'b Foo {
        y
    }

    fn bar1<'b>(x: &Foo, y: &'b Foo) -> &'b Foo {
        y
    }

    // 'a:'b,表示'a不短于'b, 只能返回x
    fn bar2<'a: 'b, 'b>(x: &'a Foo, y: &'b Foo) -> &'a Foo {
        x
        // y
    }

    // x，y 都可以
    fn bar3<'a: 'b, 'b>(x: &'a Foo, y: &'b Foo) -> &'b Foo {
        // x
        y
    }

    // 调用

    let f1 = Foo {
        x: 32,
        y: (32, true),
        z: String::from("rust"),
    };
    {
        let f2 = Foo {
            x: 32,
            y: (32, true),
            z: String::from("rust"),
        };

        bar(&f1, &f2);
        bar1(&f1, &f2);
        bar2(&f1, &f2);
        bar3(&f1, &f2);
    }
```

**扩展资料**

1.[官方文档中关于借用的介绍](https://rustwiki.org/zh-CN/book/ch04-02-references-and-borrowing.html)

2.[官方文档中关于生命周期的介绍](https://rustwiki.org/zh-CN/book/ch10-03-lifetime-syntax.html)

## 2.3 trait 与 trait object

### 2.3.1 trait 

在Rust编程语言中，`trait` 是一种定义在编译器级别的接口机制，可以定义在某种数据类型上的行为。也可以将 `trait` 理解为一种类型层级的协议，协议约定了类型共同的行为，并通过方法来暴露这些协议接口

**trait的声明及种类**

```
 // 1 trait 种类

    // 1.1 空trait

    trait A {}

    // 1.2 有方法的trait

    trait B {
        fn method(&self);
        fn method2(&self);

        // ...
    }

    // 1.3 有关联类型的trait

    trait C {
        type T;

        fn method1(&self) -> Self::T;
    }

    // 1.4 有默认实现的trait

    trait D {
        // 这个方法是默认实现
        fn method1(&self) {
            println!("method1");
        }
        fn consume_method(&mut self);
    }

    // 1.5 有自由方法（函数）的trait

    trait E {
        // 这个方法是默认实现
        fn method1(&self) {
            println!("method1");
        }
        // 这个方法需要手动实现
        fn method2(&self);

        // 这个方法是默认实现
        fn method3() {
            println!("freedom method")
        }

        // 这个方法需要手动实现
        fn method4(a: &str) -> &str;
    }

    // 1.6 trait继承

    trait F: E {
        // method
    }
```

**trait的实现**

trait一般和类型结合使用，独立trait并没有太大的意义

```rust
    // 2 如何实现 trait

    // 2.1 手动实现

    struct Teacher;

    impl Teacher {
        fn method1() {
            print!("这是类型的关联方法");
        }
    }

    Teacher::method1(); // 关联方法调用

    impl A for Teacher {}

    impl B for Teacher {
        fn method(&self) {
            print!("")
        }
        fn method2(&self) {
            print!("")
        }
    }

    let mut t = Teacher;
    t.method(); // 方法通过实例调用
    t.method();

    impl C for Teacher {
        type T = Teacher;

        fn method1(&self) -> Self::T {
            let t = String::from("Teacher");

            // t
            Teacher
        }
    }

    impl D for Teacher {
        fn consume_method(&mut self) {
            // let x = self;
            // let y = self;
        }
    }

    t.consume_method();
    t.consume_method();

    impl E for Teacher {
        fn method2(&self) {}
        fn method4(a: &str) -> &str {
            "Rust"
        }
    }

    Teacher::method4("Go"); // 对trait中自由方法的调用同调用类型的关联方法

    struct Professor;

    // impl F for Professor {}

    impl F for Teacher {}

    // 2.2 使用宏实现
    // 标准库和第三方库中一些trait可以通过派生宏来实现

    #[derive(Default, Clone)]
    struct Student {
        name: String,
        age: u32,
    }

    // 调用方法

    // 可以直接调用trait提供的方法
    let s = Student::default();
    let s1 = s.clone();
```

标准库中预导入了很多 trait，可以直接在文件中使用而不用` use`导入，你可以大概看一下下列表格，消除对 trait 的陌生感

![image-20230302004216125](https://github.com/CreatorsDAO/rust-co-learn/blob/main/images/prelude_traits.png)

**扩展资料**

1. [官方文档中关于 trait 的介绍](https://rustwiki.org/zh-CN/book/ch10-01-syntax.html)

### 2.3.2 trait object 

trait既可以作为另一个trait的约束，也可以作为泛型参数的约束，体现的是它的约束能力。trait object 提供了运行时动态分发的能力

```rust
  // 1 泛型与trait bound

    trait Animal {
        fn make_sound(&self) -> &'static str;
    }

    trait Food {}

    struct Dog;

    impl Animal for Dog {
        fn make_sound(&self) -> &'static str {
            "Woof!"
        }
    }

    struct Cat;

    impl Animal for Cat {
        fn make_sound(&self) -> &'static str {
            "Meow!"
        }
    }

    struct Pig;

    impl Animal for Pig {
        fn make_sound(&self) -> &'static str {
            "Woof!"
        }
    }

    impl Food for Pig {}

    // trait 作为约束时有三种写法

    fn get_weight<T: Animal + Food>(x: T) {

        // do sth
    }

    fn get_weight1(x: impl Animal + Food) {

        // do sth
    }

    fn get_weight2<T>(x: T)
    where
        T: Animal + Food,
    {
        // do sth
    }

    let d = Dog;
    let c = Cat;
    let p = Pig;

    // get_weight(d);
    // get_weight(c);
    get_weight(p);

    // 2 trait object
    // trait 对象通过指针来创建，如 & 或 Box<T>(一种智能指针，可以把数据存放到堆上)：&dyn Trait or Box<dyn Trait>
    // Box是Rust中唯一可以把数据强制分配到堆上的类型

    // 静态分发:在编译期通过具体类型实例直接调用方法,编译期单态化

    fn animal_make_sound<T: Animal>(a: T) {
        a.make_sound();
    }
    animal_make_sound(d);
    animal_make_sound(c);

    // 动态分发：在运行时先判断类型再查找类型对应方法
    // 特别说明，使用 trait object 会带来运行时开销

    fn animal_make_sound2(animals: Vec<&dyn Animal>) {
        for animal in animals {
            animal.make_sound();
        }
    }

    let d = Dog;
    let c = Cat;

    let animals: Vec<&dyn Animal> = vec![&d, &c];

    animal_make_sound2(animals);

    // 3 trait object 安全
    // trait中方法返回值类型不为 Self
    // trait中方法没有任何泛型类型参数

    pub trait X {
        fn method(&self) -> Self;
    }

    pub trait Y {
        fn print<T: std::fmt::Display>(&self, t: T);
    }

    // fn use_trait_object(t: &dyn X) {}
    // fn use_trait_object2(t: &dyn Y) {}
```

### 2.3.3 trait 定义共有行为

trait 通过方法可以为类型定义一些通用的方法，一个是不用再给类型专门定义，代码更加简化，第二个能够更好的规定类型行为。

```
 pub struct Book {
        name: String,
        price: f64,
        inventory: u32,
        author: String,
    }

    pub struct Cosmetic {
        name: String,
        price: f64,
        inventory: u32,
    }

    pub trait Record {
        fn set_price(&mut self, price: f64);
        fn set_inventory(&mut self, inventory: u32);
    }

    impl Record for Book {
        fn set_price(&mut self, price: f64) {
            self.price = price;
        }

        fn set_inventory(&mut self, inventory: u32) {
            self.inventory = inventory;
        }
    }

    impl Record for Cosmetic {
        fn set_price(&mut self, price: f64) {
            self.price = price;
        }

        fn set_inventory(&mut self, inventory: u32) {
            self.inventory = inventory;
        }
    }

    let mut book = Book {
        name: String::from("Book A"),
        price: 29.99,
        inventory: 10,
        author: String::from("Author X"),
    };

    let mut cosmetic = Cosmetic {
        name: String::from("Lipstick"),
        price: 9.99,
        inventory: 50,
    };

    book.set_price(39.99);
    book.set_inventory(5);

    cosmetic.set_price(14.99);
    cosmetic.set_inventory(20);

    println!(
        "Book: {} - Price: {} - Inventory: {} - Author: {}",
        book.name, book.price, book.inventory, book.author
    );
    println!(
        "Cosmetic: {} - Price: {} - Inventory: {}",
        cosmetic.name, cosmetic.price, cosmetic.inventory
    );
```

### 2.3.4 trait 与所有权

我们已经深入的介绍了所有权规则：它是 Rust 实现内存管理的杀手锏之一。trait 作为 Rust 中链接类型大厦的重要环节，和类型的所有权也有很多重要的联系

```rust
    // 1 Copy trait 和 Clone trait

    // 之前我们介绍了所有的固定尺寸类型，当把一个变量赋值给另一个变量时，会发生值的复制

    // owner_one 和 owner_two 现在各自拥有一份值，数据发生了拷贝
    let owner_one = 32;
    let owner_two = owner_one;

    // 但是对于一些动态尺寸大小的类型，比如str和[T],我们在使用它们的指针 String和Vec<T>时，不会发生值的复制，而是会移交所有权

    let owner_one = String::from("hello");
    let owner_two = owner_one;

    // println!("{:?}", owner_one); // 不可通过owner_one访问数据，因为它已经移交了所有权

    // 从trait的角度来讲，就是所有固定尺寸类型都实现了 Copy 和 Clone trait，而动态尺寸类型都没有实现 Copy trait，但大多都实现了Clone trait
    // 并且编译器报错也会告诉你，哪些类型没有实现 Copy trait

    // 如果你想在堆上复制想像使用固定尺寸类型那样一样在堆上复制一份数据，你可以显式调用Clone trait中的 clone方法来实现这一点

    let v = vec![1, 2, 3, 4, 5];

    let v1 = v.clone();
    let v2 = v.clone();
    let v3 = v1.clone();

    // 新变量的地址和原变量的地址各不相同

    println!("{:p}", v.as_ptr()); // 0x7fccb3705b30
    println!("{:p}", v1.as_ptr()); // 0x7fccb3705b50
    println!("{:p}", v2.as_ptr()); // 0x7fccb3705b70
    println!("{:p}", v3.as_ptr()); // 0x7fccb3705b90

    // 2 trait实现与所有权
    // 在自定义 trait中的方法时，你可以根据需要选择要获取类型的不可变引用、可变引用或者所有权

    trait A {
        // 需要手动实现，获取所有权
        fn take_ownership(self);

        // 默认实现，获取不可变引用
        fn take_ref(&self) {
            println!("这个方法获取了类型的不可变引用")
        }

        // 默认实现，获取可变引用
        fn take_mut(&mut self) {
            println!("这个方法获取了类型的可变引用")
        }
    }

    struct X;

    impl A for X {
        fn take_ownership(self) {
            println!("这个方法获取了类型的所有权")
        }

        // 默认实现不用手动实现
    }

    let x = X;

    x.take_ownership(); // 这个方法获取了类型的所有权
                        // x.take_ref();// 不能再使用x,因为上述方法已经获取了所有权

    let mut y = X;
    y.take_ref(); // 这个方法获取了类型的不可变引用
    y.take_mut(); // 这个方法获取了类型的可变引用

    // 特别说明：所有权机制和trait本质上是Rust中两个独立的概念，即使没有trait，所有权机制也是成立的（这也是我们在介绍所有权机制时为什么没有提及trait，因为不需要）
    // 但trait系统让所有权机制更加的显式化了，更好理解，也更好使用
```

### 2.3.5 trait 与类型转换

trait 作为类型之间转换的桥梁时是用的最多的场景之一。这些类型既包括自定义类型，也包括 Rust 标准库中的类型

```rust
 // 1 类型转换trait：From和Into
    // Into trait 会自动实现

    // 1.1 From i32 to Number

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    // 为自定义类型实现From trait，注意这里Trait带了一个类型参数i32，特指将i32转换为Number

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    // 使用From trait中的from方法将i32转换为Number
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let n: Number = 32.into();

    // 1.2 From Number to i32

    // 为自定义类型实现Into trait，注意这里Trait带了一个类型参数Number，特指将Number转换为i32
    impl From<Number> for i32 {
        fn from(item: Number) -> Self {
            item.value
        }
    }

    let num = i32::from(32);
    let x = Number { value: 10 };

    // 使用Into trait中的into方法将Number转换为i32
    let num: i32 = x.into();
    println!("number is {:?}", num);

    // 与此相似的trait还有 TryFrom 和 TryInto
    // 在实际中，TryFrom 和 TryInto 用的比较多，因为它们可以处理错误，但是实现逻辑和 From 和 Into 一样

    // 2 AsRef 和 AsMut

    // 通过AsMut获取可变引用:注意这里获取结构体成员的可变引用
    impl AsMut<i32> for Number {
        fn as_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
    }

    let mut num = Number { value: 30 };

    let ref_num = num.as_mut();

    // 通过AsRef获取变量的不可变引用:注意这里获取结构体成员的不可变引用
    impl AsRef<i32> for Number {
        fn as_ref(&self) -> &i32 {
            &self.value
        }
    }

    let num = Number { value: 40 };

    let ref_num: &i32 = num.as_ref();

    // 特别说明：以上代码展示并不一定是最佳实践，只是为了介绍知识点而展示的可能性
```

如下是不同场景下经常使用的 trait

![img](https://github.com/CreatorsDAO/rust-co-learn/blob/main/images/traits_fetures.png)

## 2.4 课后习题

1.rust 中不同的类型的变量，所有权是如何体现的？

2.你可以编写一个涉及生命周期参数的代码示例吗？

3.Rust 中的 trait 怎样理解，它有哪些功能？

# 模块三：Rust 进阶知识

## 3.1 trait 进阶

trait 在 Rust 中涵盖的内容非常多，它不光为一些常见的类型定义了大量的方法，同时还为一些比较特殊的类型提供了支持

### 3.1.1 trait 与闭包

```rust
// 1. 回顾三种类型的闭包
// 前面我们介绍过，闭包有三种类型：未捕获环境变量，捕获环境变量不修改，捕获环境变量并修改

// 1.1 未捕获环境变量
let c1 = || println!("didn't catch env var");
c1();

// 1.2 捕获但不修改环境变量
let x = 10;

let c2 = || println!("catch env var but not modify, x = {}", x);

c2();

// 1.3 捕获并修改环境变量

let mut x = 10;
let mut c3 = |a: i32| {
    x = 1 + a;
    println!("catch env var and modify, x = {}", x);
};
c3(10);

// 2. 闭包实现与trait

// 在Rust中，闭包实际上是一个语法糖，它的实现在抽象概念上可以看做是一个匿名结构体，这个结构体会把环境变量捕获成为其成员，并实现Fn/FnMut/FnOnce trait
// Fn/FnMut/FnOnce中各有一个方法分别是call/call_mut/call_once，对应的语义分别是调用不可变闭包、调用可变闭包、调用消费闭包
// 并且Fn/FnMut/FnOnce trait是以次继承的，也就是说实现 Fn trait,必须实现 FnMut trait，实现 FnMut trait,必须实现 FnOnce trait

// 当声明一个闭包时，编译器会根据闭包的类型，自动推导出其实现的trait，一般情况下不需要手动实现

// 3. 闭包作为函数参数传递
// 值得注意的是，在将闭包作为参数在函数中传递时，类型的指定是通过trait来实现的

fn call_fn<F: Fn()>(f: F) {
    f();
}

fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
}

fn call_fn_once<F: FnOnce()>(f: F) {
    f();
}

// 闭包的调用 case 1
// Rust编译器会根据你如何调用推导出闭包的类型，也就是实现哪个trait

let c = || println!("closure");

call_fn_once(c); // 实现了FnOnce trait
call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
call_fn_mut(c); // 实现了FnMut trait,FnOnce trait

// 闭包的调用 case 2

let x = "10";

let c = || println!("get env var {}", x);

call_fn_once(c); // 实现了FnOnce trait
call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
call_fn_mut(c); // 实现了FnMut trait,FnOnce trait

// 闭包的调用 case 3

let mut x = String::from("10");

let mut c = || println!("get env var {x:?}", x = String::from("20"));

call_fn_once(c); // 实现了FnOnce trait
call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
call_fn_mut(c); // 实现了FnMut trait,FnOnce trait

// 4. 闭包作为函数返回

fn return_fn() -> impl Fn() {
    || println!("call_fn")
}

fn return_i32(i: i32) -> i32 {
    32
}

fn return_fn_mut() -> impl FnMut() {
    let x = 10;
    // || println!("call_fn_mut {}", x + 1) // 不能返回局部变量
    move || println!("call_fn_mut {}", x + 1) // 必须把局部变量移入闭包，才能返回（这里实际上发生了数据的复制）
}

fn return_fn_once() -> impl FnOnce() {
    let s = String::from("hello");
    // || println!("call_fn_once {:?}", s)
    move || println!("call_fn_once {:?}", s) // 必须把局部变量移入闭包，才能返回（这里实际上发生了所有权转移）
}
```

**扩展资料**

1.[官方文档中关于闭包的介绍](https://rustwiki.org/zh-CN/book/ch13-01-closures.html)

### 3.1.2 trait 与迭代器

Rust 提供了迭代器 trait,可以实现遍历功能

```rust
    // 1. for 循环与迭代器

    // 在rust中，for循环实际上的迭代器的语法糖

    // for 循环以及解糖
    let values = vec![1, 2, 3, 4, 5];
    // 使用 for 循环遍历集合中个每个元素
    for x in values {
        println!("{x}");
    }
    // for 循环解糖后等价如下：
    let v = vec![1, 2, 3, 4, 5];
    // 先将集合转为迭代器类型
    let mut v_iter = v.into_iter();
    // 在 loop 循环中使用next方法循环获取集合中下一个元素，当集合中取不到值时使用break终止 loop循环
    loop {
        match v_iter.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    // 2. 迭代器 trait IntoIterator 和 Iterator
    // IntoIterator trait 中的 into_iter方法会返回一个 实现了 Iterator trait 迭代器
    // Iterator trait 通过其 next方法来获取集合中的下一个元素

    use std::collections::HashMap;
    use std::slice::Iter;
    use std::slice::IterMut;
    use std::vec::IntoIter;


    // 如果类型实现了迭代器 trait，则可以使用迭代器中的方法，例如：

    let map = HashMap::from([("rust", 1), ("go", 2), ("python", 3)]);
    let map_iter = map.into_iter();
    let vec: Vec<(&str, i32)> = map_iter.collect();
    println!("{:?}", vec); // [("rust", 1), ("go", 2), ("python", 3)]

    // 3. 迭代器、借用和所有权
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter_mut: IterMut<i32> = v.iter_mut(); // 转为  IterMut 结构体, 可变借用
    let iter: Iter<i32> = v.iter(); // 转为 Iter 结构体， 不可变借用
    let iter_into: IntoIter<i32> = v.into_iter(); // 转为 IntoIter 结构体 ， 获取所有权

    // 4. 迭代器适配器
    let vec = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = vec
        .iter()
        .map(|&x| x * 3)
        .take(3)
        .filter(|x| *x > 6)
        .collect();
    println!("{:?}", doubled); // [9]

    // 5 迭代器与迭代器适配器特性：lazy（惰性）
    let v = vec![1, 2, 3, 4, 5];
    v.iter().for_each(|x| println!("{x}"));
    // or
    for x in &v {
        println!("{x}");
    }
```

**扩展资料**

1.[官方文档中关于迭代器的介绍](https://rustwiki.org/zh-CN/book/ch13-02-iterators.html)

### 3.1.3 trait 与智能指针

```rust
    // 在展开Rust的智能指针之前，我们先区分一下Rust中的智能指针、引用和裸指针

    // 1 指针、引用和智能指针

    // 1.1 引用

    let x = 100;
    let mut y: i64 = 200;
    #[derive(Debug)]
    struct A(i32);
    let a = A(100);

    // 使用 & 获取不变或者可变引用
    let x_pointer = &x;
    let y_pointer = &mut y;
    let a_pointer = &a;

    println!("{:?}", x_pointer); // 100 打印时会自动“解引用”到数据，而不是地址
    println!("{:p}", x_pointer); // 0x7ff7b9bae33c 如果要打印地址的话，改变占位符？为 p

    // let z = &mut y; // 可变借用不能超过1次

    y = *y_pointer + 100; // 解引用后修改

    println!("{:?}", y); //300 本条代码结束后，可变借用才释放
    println!("{:?}", a_pointer); // A(100)

    // 1.2 裸指针
    let x = 100;
    let mut y: i64 = 200;
    struct B(i32);
    let a = B(100);

    // 裸指针是使用 as *const 从引用转换而来
    let x_raw_pointer = &x as *const i32;
    let y_raw_pointer = &mut y as *const i64;
    let a_raw_pointer = &a as *const B;

    println!("{:?}", x_raw_pointer); // 0x7ff7b763a46c，裸指针打印时不会被“解引用”到数据，而是会直接会打印地址

    unsafe {
        y = *y_raw_pointer + 300; // 裸指针解引用需要使用unsafe 语法块，这里的解引用的安全的

        let z_raw_pointer = &mut y as *const i64; // 第二次生成可变裸指针，unsafe 块绕过了可变借用的次数规则，是不是感觉有点危险？

        y = *z_raw_pointer + 500; // 然后继续改变数据

        println!("{:?}", *y_raw_pointer); // 1000
    }
    println!("{:?}", a_raw_pointer); // 0x7ff7b763a47c
    println!("{:?}", y); // 1000

    // 1.3 智能指针

    // Vec 和 String 类型都是智能指针

    let vec = vec![1, 2, 3, 4];
    let s = "rust".to_string();
    let num = Box::new(100);

    let v1 = vec; // 发生了move语义，现在数据的所有者不再是vec 而是v1，数据没变，拥有者变了

    // println!("{:?}", vec); // 不能再使用 vec，因为它不再拥有数据了

    let v = [1, 2, 3, 4];
    let v = &v1; // 只是借用，v 仍然拥有数据
    println!("{:?}", v); // 所以可以使用 v

    // 2 智能指针与结构体、trait
    // Rust中的智能指针都是以结构体进行封装，然后为它实现了某些trait
    // 事实上，Rust中很多特殊的类型都是基于结构体来实现的，并且在这些结构体上实现了各种trait，这是一种通用的类型构造思路

    // 标准库中的一些智能指针的定义
    /*
    pub struct Box<T, A = Global>(_, _)
    where
        A: Allocator,
        T: ?Sized;

    pub struct String {
        vec: Vec<u8>,
    }

    pub struct Vec<T, A: Allocator = Global> {
        buf: RawVec<T, A>,
        len: usize,
    }

    pub struct Rc<T: ?Sized> {
        ptr: NonNull<RcBox<T>>,
        phantom: PhantomData<RcBox<T>>,
    }
    */

    // 那什么是智能指针？智能指针是实现了Deref trait 或 Drop trait 的结构体类型
    // Deref允许自动解引用，Drop允许自动释放资源

    // 2. 自定义智能指针

    // 2.1 实现 Drop trait

    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }

    impl Drop for User {
        fn drop(&mut self) {
            println!("User has been dropped {:?}:", "rust") // 实现细节只是做了打印
        }
    }

    // 2.2 实现 Deref trait
    use std::ops::Deref;

    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    // 2.3 Drop trait 如何起作用的？
    // 当一个值离开作用域时，它的drop方法会被自动被编译器调用，无需手动调用，强行手动调用编译器会报错

    {
        let mut user = User {
            name: "rust".to_string(),
            age: 12,
        };

        // user.drop(); //手动调用不行 因为编译器会自动调用，显式调用二者会冲突

        // 你会在终端发现打印了 “Rust”，成功验证，编译器确实调用了 drop
    }

    // 2.4 Deref trait 如何起作用的？

    {
        let m = MyBox("rust");
        let ref_my_box = *m; // 实现了 Deref trait的智能指针可以使用 * 直接解引用到内部的值
                           // 等价于下面：deref函数会自动调用
        let ref_my_box = *(m.deref());

        // String是智能指针，它实现了Deref trait，所以可以直接解引用

        fn take_ref_string(s: &str) {
            println!("{:?}", s)
        }

        // 将String解引用为str
        // 注意：String这个智能指针包裹的类型是 str，解引用后大小编译器无法确定，所以要再加&（引用）
        let s = String::from("Rust");
        // take_ref_string(s);
        take_ref_string(&s);
    }
```

Rust 中有多个智能指针，可以参考下表，这里总结了一个表，方便你阅读

![image-20230203001747611](https://github.com/CreatorsDAO/rust-co-learn/blob/main/images/smart_pointers.png)

**扩展资料**

1.[官方文档中关于智能指针的介绍](https://rustwiki.org/zh-CN/book/ch15-00-smart-pointers.html)

## 3.2 类型进阶

Rust 中对于提供了很多类型，用于处理一些特殊的场景

### 3.2.1 Box <T>

Box 可以将内存强制分配到堆上，并且它也是智能指针，可以自动解引用和管理堆内存。所以在使用的时候只需要使用它将数据分配到堆上，并不需要再考虑如何释放内存

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

### 3.2.2 可变容器

在编译期，我们需要使用 mut 显式声明变量的可变性。在运行时，Rust 提供了可变容器 Cell 和 RefCell 允许修改不可变变量（这个过程实际上是通过原生指针来完成的）

```rust
// 1.编译期：通过 mut 显式声明变量的可变性，也叫外部可变性
    use std::cell::Cell;
    let can_not_change = "rust";
    let mut can_change = "go";
    // can_not_change = "cpp"; // 不可重新赋值
    can_change = "c"; // 可以更改

    // 2 一个需要改变不可变变量的例子

    // let var1 = 0;
    // let mut var2 = 0;

    // while var2 <= 10 {
    //     if var2 == 10 {
    //         var1 = 10;
    //     }
    //     var2 += 1;
    // }

    // println!("var1: {}, var2: {}", var1, var2);

    // 3. 运行期：通过Cell和RefCell实现可变性，也叫内部可变性
    // 3.1 Cell<T> 的修改和读取
    struct Foo {
        x: u32,
        y: Cell<u32>,
        z: Cell<Vec<String>>,
    }

    let foo = Foo {
        x: 1,
        y: Cell::new(3),
        z: Cell::new(Vec::new()),
    };

    // 修改容器内的变量使用set方法
    foo.y.set(100);
    foo.z.set(vec!["rust".to_owned()]);

    // 读取容器内的变量有两种：固定大小类型可以使用 get和into_inner; 动态大小类型只能使用into_inner
    assert_eq!(100, foo.y.get());
    assert_eq!(100, foo.y.into_inner());

    // assert_eq!(vec!["rust".to_owned()], foo.z.get()); 不能使用get方法
    assert_eq!(vec!["rust".to_owned()], foo.z.into_inner());

    // 3.2 RefCell<T> 的修改和读取
    // 通过borrow_mut实现可变性
    // 主要是应用于一些动态大小类型，通过borrow获取值，有运行时开销

    use std::cell::RefCell;
    let vec = vec![1, 2, 3, 4];

    let ref_vec = RefCell::new(vec);

    println!("{:?}", ref_vec.borrow()); // 不可变借用 使用borrow
    ref_vec.borrow_mut().push(5); // 可变借用改变，使用borrow_mut
    println!("{:?}", ref_vec.borrow());
```

### 3.2.3 共享容器

共享容器 Rc<T>和 Arc<T>之前在所有权共享中介绍过。通过共享容器我们可以使多个变量拥有所有权（本质上是通过引用计数实现的），从而对资源进行操作。具体示例细节你可以参考之前的代码

### 3.2.4 特殊类型

**`PhantomData<T>`** ，它通常用于在泛型代码中标记一些类型参数，但不实际使用它们，从而向 Rust 编译器传达有关代码中类型关系的信息。它被称为 “幽灵数据”，因为它不占用任何实际内存空间，只在编译时起作用

一般它起两个作用：

1 用于在类型签名中传递类型信息，表示一种假象“拥有关系”

2 作为一个类型参数的标记，用于告诉 Rust 编译器某些重要信息，例如，当需要实现 `Drop` trait 时，但是类型不实际包含任何需要释放的资源，可以使用 `PhantomData` 来占据一个虚拟的位置，这样以确保编译器不会优化掉的 `Drop` 实现

```rust
    use std::marker::PhantomData;
    use std::ops::Deref;

    struct MyType<T> {
        data: *const T,
        _marker: PhantomData<T>,
    }

    impl<T> MyType<T> {
        fn new(t: T) -> MyType<T> {
            MyType {
                data: &t,
                _marker: PhantomData,
            }
        }
    }

    impl<T> Deref for MyType<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.data }
        }
    }

    impl<T> Drop for MyType<T> {
        fn drop(&mut self) {
            println!("Dropping MyType instance!");
        }
    }

    let resource: MyType<bool> = MyType::new(true);
    let another_resource: MyType<i32> = MyType::new(32);

    print!("{:?}", unsafe { *(resource.data) });
    print!("   {:?}", unsafe { *(another_resource.data) });

    let my_instance: MyType<i32> = MyType::new(33);
    // 执行到这里时，my_instance 将会离开作用域并被销毁，调用我们自定义的 drop 方法。
```

**`Pin<T>`**: 通常用于解决 Rust 引用类型的安全性问题，尤其是与异步编程和内存管理相关的问题。`Pin<T>` 类型可以确保被引用的值不会被移动或释放

一般情况下：Rust 会确保引用总是有效的，但是也有例外情况：

1. 当异步代码在运行时可能会移动或释放被引用的值时，比如 Future 或 async 闭包（这个我们后面再讲）
2. 当使用 `unsafe` 代码时，可能会通过裸指针将引用类型转换为可变引用类型，从而破坏编译器对引用类型的保护

```rust
// 2 特殊类型：Pin<T>

  use std::marker::PhantomPinned;
    use std::pin::Pin;

    // 定义一个自引用的结构体。因为它含有指向自身的指针，所以它在内存中不能被移动。
    struct SelfReferential {
        i: i32,
        p: *const i32,       // 裸指针，将会指向上述的 i
        _pin: PhantomPinned, // 也是一个零大小的标记类型，阻止 Rust 自动为我们的类型实现 Unpin trait
    }

    // 注意此时 p 是一个空指针，我们还没有为它分配指向的地址
    let mut test = SelfReferential {
        i: 123,
        p: std::ptr::null(),
        _pin: PhantomPinned,
    };

    // 使用 Pin 包装我们的结构体实例。这样就能保证 test 的内存地址不会在其生命周期中改变。
    // 注意：这里使用了 unsafe，因为我们需要保证在 test 被包装为 Pin 后，其地址不会被改变
    let mut test = unsafe { Pin::new_unchecked(&mut test) };

    // 创建一个裸指针，指向 test 的 i 字段。注意我们使用了 test 的引用版本以保证安全。
    let self_ptr: *const i32 = &test.as_ref().get_ref().i;

    // 将裸指针存储到 test 的 p 字段。注意我们使用了 unsafe，因为我们正在直接修改内存。

    unsafe {
        let mut_ref = Pin::as_mut(&mut test);
        mut_ref.get_unchecked_mut().p = self_ptr;
    }

    // 打印 test 的 p 字段所指向的内容。注意我们使用了 unsafe，因为我们正在解引用裸指针。
    let val = unsafe { *(test.as_ref().get_ref().p) };
    println!("val: {}", val); // 输出 "val: 123"
```

## 3.4 课后习题

1. 闭包的本质是什么
2. rust 中的迭代器是如何实现的，有哪些主要的方法，你能编写几个使用案例吗
3. 智能指针“智能”在哪里？Rust 中常见的智能指针有哪些？
4. Rust 中有哪些容器类型，适用于哪些场景？
5. 你能从标准库中发现其他比较特殊的类型吗？

# 模块四：Rust 项目基础

## 4.1 错误处理

Rust 整体的错误处理机制有一个层级，随着错误的`严重程度`可以选择不同的处理方案（其实每一种错误都挺严重的）

1. 类型系统保证函数契约（Rust 严格的类型系统已经帮我们消除了这部分的错误，如果类型不正确，是不会通过编译的）
2. Option<T>消除空指针失败 (处理有值或者无值的情况)
3. Result<T,E> 传播错误 （处理成功或者失败的情况，失败时可以抛出错误）
4. 断言用于防御
5. Panic 恐慌

```rust
 // 1 类型系统保证函数契约
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // sum(1u32, 2u32) 违反函数契约

    // 2 使用Option处理有值或无值的情况
    // 当某个值可能为无值时，应该使用Option<T>来包裹，以正确处理无值的情况
    fn log(val: f64) -> Option<f64> {
        match val.log2() {
            x if x.is_normal() => Some(x), // 有值情况
            _ => None,                     // 无值情况
        }
    }

    // 当一个值为Option<T>时，经常使用map和and_then等方法来链式处理

    fn double(val: f64) -> f64 {
        val * 2.
    }

    fn square(val: f64) -> f64 {
        val.powi(2 as i32)
    }

    fn inverse(val: f64) -> f64 {
        val * -1.
    }

    fn sqrt(val: f64) -> Option<f64> {
        match val.sqrt() {
            x if x.is_normal() => Some(x),
            _ => None,
        }
    }

    let number = 20.;
    let result = Option::from(number)
        .map(inverse)
        .map(double)
        .map(inverse)
        .and_then(log)
        .map(square)
        .and_then(sqrt);
    match result {
        Some(x) => println!("x was {:?}", x),
        None => println!("this failed"),
    }

    // 3 Result 用于处理成功或失败的情况

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error;

    fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    let file_path = "example.txt";
    match read_file_contents(file_path) {
        Ok(contents) => println!("File contents: {}", contents), // 成功情况
        Err(error) => println!("Error reading file: {}", error), // 失败情况，抛出错误
    }

    // 4 断言
    // 当你确定某个值一定会出现某种情况时，可以使用断言来终止程序

    fn extend_vec(v: &mut Vec<i32>, i: i32) {
        assert!(v.len() == 3); // 断言
        v.push(i)
    }

    let mut vec = vec![1, 2, 3];
    extend_vec(&mut vec, 4);

    assert_eq!(4, vec[3]); // 断言

    // 5 恐慌
    // 当你确定某个值一定不会出现某种情况时，可以使用恐慌来终止程序
    //

    fn factorial(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    let result = factorial(10);
    println!("Result: {}", result);
    if result < 1000 {
        panic!("Result too large!"); // 使用panic!恐慌
    }

    // 总结：使用Option和Result来处理值或者错误，使用恐慌和断言来终止程序
```

一个使用第三方库自定义错误的小例子，使用thiserror，请先使用`cargo add thiserror` 安装依赖

```
// 扩展:使用第三方库thiserror来自定义错误

    use std::io;
    use thiserror::Error;

   

    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("data store disconnected")]
        Disconnect(#[from] io::Error),
        #[error("the data for key `{0}` is not available")]
        Redaction(String),
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        InvalidHeader { expected: String, found: String },
        #[error("unknown data store error")]
        Unknown,
    }

    fn read_data(key: &str) -> Result<(), DataStoreError> {
        if key == "invalid_key" {
            return Err(DataStoreError::Redaction(format!(
                "The data for key `{}` is not available",
                key
            )));
        }

        // 读取数据的逻辑...

        Ok(())
    }

    match read_data("valid_key") {
        Ok(()) => println!("read success"),
        Err(err) => println!("error: {:?}", err),
    }

```

## 4.2 项目管理

Rust 工程管理非常友好，可以轻松管理复杂庞大的工程项目

### 4.2.1 crate

在 Rust 中，有两种类型的 crate，也叫 package（类似于其他语言的`包`）: binary application package 和 library package .前者主要应用程序入口，后者更多的是为前者提供各种各样的功能支持

可以使用`Cargo`来创建

```bash
 cargo new c1 --lib // 创建lib package
 cargo new c2 --bin // 创建binary package，默认创建binary package
```

两种 crate 的区别并不大，但 binary application package 可以直接使用 cargo run 运行，而 library package 需要配置才可以。你可参考下面链接了解如何配置

[如何在 lib crate 中运行程序](https://zhuanlan.zhihu.com/p/614506900)

### 4.2.2 工作空间

工作空间用于组织多个 crate，本文档的代码组织结构就是使用 workspace 组织了多个 lib crate

```bash
[workspace]
members = ['module-one','module-two','module-three','module-four','module-five','module-six']
```

你可参考下面链接了解如何为项目配置工作空间

[如何配置 workspace](https://zhuanlan.zhihu.com/p/614506900)

## 4.3 测试

测试对于任何语言来说都非常重要，在 Rust 中，你可以非常轻松的构建测试。测试需要放在测试模块中，你可以在代码编写完成后直接在当前文件中编写测试，也可以新建一个测试文档专门编写

### 4.3.1 单元测试

单元测试主要对局部模块内的代码进行测试。测试代码放在测试模块中

```rust
use std::fs::File;
use std::io::Error;

fn read_file(path: &str) -> Result<File, Error> {
    // 2.1 读取文件
    let file = File::open(path);

    // 2.2 判断文件是否存在
    match file {
        Ok(file) => Ok(file),
        Err(error) => Err(error),
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// 测试模块

#[cfg(test)]
mod tests {
    use super::*;

    // 1 使用 assert! 宏断言结果是抖为 true

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller)); // 可以直接断言，也可以带上提示信息
        assert!(
            larger.can_hold(&smaller),
            "larger is {:?}, smaller is {:?}",
            larger,
            smaller
        );
    }

    // 2 使用 assert_eq! 宏断言两个值相等

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // 3 使用 assert_ne! 宏断言两个值不相等
    #[test]
    fn it_adds_two() {
        assert_ne!(3, add_two(2));
    }

    // 4 使用 should_panic 宏断言函数会 panic

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 5 使用Result<T, E>类型的断言

    #[test]
    fn read_file_should_works() -> Result<(), String> {
        match read_file("rust.txt") {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("file did not exit")),
        }
    }
}
```

### 4.3.2 文档测试

文档测试也是单元测试，只不过不把测试代码写在测试模块中而是写在文档备注中

````rust
/// ```
/// fn add(a: i32, b: i32) -> i32 {
/// a + b
/// }
/// let result = add(2, 3);
/// assert_eq!(result, 5);

/// ```
fn add() {}
````

本文档的所有代码都以文档测试形式编写，你可以直接运行

### 4.3.3 集成测试

集成测试对于整个 lib crate 来说是外部的，目的在于测试各个模块是否能够一起正确的工作，创建集成测试需要创建于 src 同级的 tests 目录执行

在 lib.rs 下创建一个模块，并导出定义的函数

```rust
pub use my_add::*;
mod my_add {

    pub fn add() -> i32 {
        2 + 2
    }
}
```

新建 src 同级目录 tests，并创建文件 add_test.rs,并在其中先引入函数，再编写测试

```rust
use module_four::add;

#[test]
fn it_adds_two() {
    assert_eq!(4, add());
}
```

然后在项目下运行`cargo test`就可以测试了

## 4.4 课后习题

1. 你认为的错误处理逻辑应该是怎样的？
2. 以一个工作空间的形式创建一个项目，并在其下创建多个模块，并把它们关联起来，使之能够正常编译运行
3. 尝试编写单元测试和集成测试

# 模块五：并发编程和异步编程

## 5.1 并发编程

### 5.1.1 原理介绍

在实际的业务场景中，我们可能会并发或者异步处理各种各样的请求，甚至是二者相结合，以提高请求处理效率。在此之前，我们先区别两个概念：并发和并行：并发和并行都是对“多任务”处理的描述，其中并发是轮流处理，而并行是同时处理

现代个人计算机通常拥有多核心，通过将任务分成多个队列并交给不同核心处理并行执行，可以极大提高处理效率和速度

在操作系统层面，多线程管理任务队列，每个线程管理一个任务队列，并可根据空闲程度进行任务调度。程序只与操作系统线程交互，而不关心 CPU 核心数量，当线程将任务分配给 CPU 核心执行时，若只有一个核心，则只能同时处理一个任务。此时微观上 CPU 快速轮换处理不同的任务，带来了宏观上的同时运行假象。若有 N 个核心时，就能够实现同时处理 N 个任务

当操作系统的线程为 M,CPU 核心数量为 N 时，在 M 个线程中的任务会被分配给 N 个 CPU 核心处理，此时就实现了 M:N 处理模型。此时并发和并行同时发生

操作系统为编程语言提供了创建线程的 API，此时程序内创建的线程数量会和该程序占用操作系统中的线程数量相等，即 1:1 模型。Rust 就是这种模型，当你在程序中创建了一个线程时，这意味着你创建了一个操作系统线程

有些编程语言实现了自己的线程模型（例如绿色线程和协程），程序内的 M 个线程会映射到 N 个操作系统线程上运行，这被称为 M:N 线程模型，其中 M 和 N 没有特定的限制关系。Go 语言是一个典型的例子。其他语言则使用 Actor 模型，基于消息传递实现并发，例如 Erlang 语言

### 5.1.2 并发实战

**在程序中创建多个线程**

尽管并发编程涉及计算机的物理基础和一些操作系统知识，比较费解。但是在创建上非常简单，你只需要借助编程语言提供好的接口直接创建即可

```rust
// main函数是主线程
fn main() {
    // 创建线程

    use std::thread;

    let mut threads = vec![];

    // 创建5个线程，它们和main函数的线程是并行的，并且执行结束的时间不一定
    for i in 0..5 {
        // let x = move || {
        //     println!("Hello from thread {}", i);
        // };

        // let handle = thread::spawn(x);

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

**在多个线程间共享资源**

并发编程除了在原理理解上比较难之外，另一个难点是异步间共享数据。一般情况下，线程间共享数据主要有两种方式，共享内存和消息传递，其中共享内存还可以分为无锁共享和有锁共享。在 Rust 中，你可以通过通道（channel）在线程间发送数据，还可以通过锁来独占访问数据，也可以通过直接操作原子类型，实现无锁共享，下面是几个例子

```rust
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

## 5.2 异步编程

我们再区别一下同步和异步，从调用者得视角看同步指调用直到获得结果时才返回，而异步指在未获得结果时先返回。

1. 同步 IO 模型下，程序执行时，数据从外部到程序内部的整个过程可以分为两段：数据准备和数据拷贝阶段（从操作系统内核缓冲区复制到进程缓冲区）。程序在使用数据时需要通过系统调用来请求数据

数据准备阶段：可以是阻塞的（阻塞线程，直到数据准备好）和非阻塞的（立即返回错误，然后通过轮询数据，直到数据准备就绪）

数据拷贝阶段：把数据从内核缓冲区拷贝至应用程序缓冲区（用户态缓冲区），同步 I/O 下永远阻塞

2. 异步 I/O 可以把数据的准备和复制过程看作是一个操作，均由操作系统完成

### 5.2.1 异步原理

Rust 中的异步编程通过事件循环或异步运行时来实现，它避免了线程上下文切换的开销和内存占用。异步任务在等待 I/O 操作完成时可以挂起，而不需要占用线程。当 I/O 操作完成时，异步任务可以继续执行，从而减少了线程切换的开销。因此，异步编程可以更高效地利用系统资源，提高应用程序的性能和可靠性

另外，注意 Rust 中的异步 IO 编程模型包含了操作系统的同步 IO 和异步 IO

### 5.2.2 异步运行时

Rust 只提供了零成本的异步编程抽象而不内置运行时，这意味着你可以根据实际业务需求替换运行时，如 tokio，async-std，no_std 和 smol 等

### **5.2.3 async 和 await**

Rust 中提供了一个 Future trait，并在 future 的基础上提供 async/await 语法糖，它的本质是一个状态机，也叫无栈协程

零成本抽象：async/await 语法糖在编译期就会展开成为固定类型

这里的 async/await 语法很形象，比较好理解，早期 Rust 中的异步写法是一串串链式调用，一个主要的问题是会产生很多内嵌代码

，不好处理。而使用 async/await 语法允许你像写同步代码一样写一步代码

异步任务可以看作是一种绿色线程，在线程内执行，它们的区别在用户态，没有线程上下文切换开销

### 5.2.4 异步实战

尽管 Rust 异步编程原理理解比较困难，但是在实际使用中它非常简单，下面是一个案例

Cargo.toml

```
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

```rust
use reqwest::{blocking::Client, Error};
use serde::Deserialize;
use std::vec::Vec;
use tokio;

#[derive(Deserialize, Debug)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

async fn fetch_post_async(url: &str) -> Result<Post, Error> {
    let response = reqwest::get(url).await?;
    let response = response.error_for_status()?;
    let post: Post = response.json().await?;
    Ok(post)
}

fn fetch_post_sync(url: &str) -> Result<Post, Error> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let response = response.error_for_status()?;
    let post: Post = response.json()?;
    Ok(post)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];

    let urls_clone = urls.clone();

    println!("同步请求：");

    let sync_start = std::time::Instant::now();
    let sync_thread = std::thread::spawn(move || {
        for url in &urls_clone {
            match fetch_post_sync(url) {
                Ok(post) => println!("Post信息: {:?}", post),
                Err(err) => eprintln!("请求失败: {:?}", err),
            }
        }
    });
    sync_thread.join().unwrap();
    let sync_duration = sync_start.elapsed().as_millis();
    println!("同步请求总耗时：{} ms", sync_duration);

    println!("异步请求：");
    let async_start = std::time::Instant::now();
    let mut tasks = Vec::new();
    for url in urls {
        let task = tokio::spawn(async move {
            match fetch_post_async(&url).await {
                Ok(post) => println!("Post信息: {:?}", post),
                Err(err) => eprintln!("请求失败: {:?}", err),
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }
    let async_duration = async_start.elapsed().as_millis();
    println!("异步请求总耗时：{} ms", async_duration);

    let ratio = sync_duration as f64 / async_duration as f64;
    println!("同步请求耗时是异步请求耗时的 {:.2} 倍", ratio);

    Ok(())
}
```

## 5.3 课后习题

1. 如何理解 Rust 中的并发编程和异步编程？
2. 尝试编写一个并发程序
3. 使用社区中你喜欢的异步运行时编写一个小程序

# 模块六：Rust 内容扩展（选学）

## 6.1 宏编程

### 6.1.1 宏介绍

宏（macro）是一种编程技术，允许程序员编写可在编译时或预处理时展开的代码片段。许多高级编程语言支持宏或类似的功能。Rust 支持两种类型的宏：声明式宏（Declarative Macros）和过程宏（Procedural Macros）

### 6.1.2 声明宏

声明式宏类似于 C 和 C++ 中的预处理器宏，但更强大和安全。声明式宏使用 `macro_rules!` 关键字定义，并通过模式匹配和代码生成来工作。它们可以用来减少代码重复、创建简洁的 DSL（领域特定语言）等

如下是一个声明宏的声明和使用案例

```rust
// 导入 HashMap，用于存储键值对。
use std::collections::HashMap;
// 导入 lazy_static 宏，用于创建静态变量。
use lazy_static::lazy_static;

// 定义一个名为 `create_map` 的宏，它接受一系列键值对，并在展开时创建一个包含这些键值对的 HashMap。
// 宏使用了匹配表达式 (`$key:expr => $value:expr`) 来捕获键值对，并使用重复模式 (`$()*`) 来插入每个键值对到 HashMap。
macro_rules! create_map {
    // 接受一系列键值对，每个键值对以 `$key:expr => $value:expr` 的形式给出。
    // 最后一个键值对后面可以有一个可选的逗号。
    ( $($key:expr => $value:expr),* $(,)? ) => {{
        // 创建一个新的 HashMap。
        let mut map = HashMap::new();
        // 使用重复模式，对每一个键值对执行以下代码。
        $(
            // 将每个键值对插入到 HashMap 中。
            map.insert($key, $value);
        )*
        // 返回填充好的 HashMap。
        map
    }};
}

fn main() {
    // 使用 lazy_static 宏创建一个名为 `FRUITS` 的静态 HashMap。
    // 这里我们使用 `create_map!` 宏来初始化 HashMap。
    lazy_static! {
        static ref FRUITS: HashMap<&'static str, u32> = create_map! {
            "apple" => 1,
            "banana" => 2,
            "orange" => 3,
            "peach" => 4,
        };
    }

    // 打印 FRUITS 的内容。注意，我们需要使用 *FRUITS 来解引用 FRUITS。
    println!("{:?}", *FRUITS);
}
```

声明宏的一大特征是 Token 的匹配，Token 有多种，如下，相当于宏的关键字，在实际代码中，尤其涉及到重复模式时，可以考虑用声明宏来消除样板代码

```bash
$name:ident：标识符（identifier），如变量名、函数名等。例如：$func_name:ident。
$e:expr：表达式（expression），如 1 + 2、my_var 等。例如：$my_expression:expr。
$t:ty：类型（type），如 i32、String、Vec<T> 等。例如：$my_type:ty。
$p:pat：模式（pattern），如用于匹配的字面量、变量、通配符等。例如：$my_pattern:pat。
$s:stmt：语句（statement），如赋值语句、函数调用等。例如：$my_statement:stmt。
$b:block：代码块（block），由一对大括号包围的一系列语句。例如：$my_block:block。
$m:meta：元数据（metadata），如属性（attribute）中的元数据。例如：$my_metadata:meta。
$i:item：项（item），如函数、结构体、枚举等顶级定义。例如：$my_item:item。
$v:vis：可见性（visibility），如 pub 关键字。例如：$my_visibility:vis。
$l:lifetime：生命周期（lifetime），如生命周期参数 'a。例如：$my_lifetime:lifetime。
$tt:tt：单个语法树（token tree），可以匹配任何单个 token。例如：$my_token_tree:tt。
```

### 6.1.3 过程宏

过程宏可以分为三种类型：自定义派生（custom derive）、属性宏（attribute macros）和函数宏（function-like macros）。它们的原理如下：

1. 自定义派生（Custom Derive）：

自定义派生允许你为自定义数据类型自动实现某个 trait。当你在数据类型上使用 `#[derive(MyTrait)]` 时，编译器会调用实现了 `MyTrait` 自定义派生的过程宏。该过程宏将接收输入类型的定义，然后生成该类型所需的 trait 实现。

自定义派生宏需要使用 `#[proc_macro_derive]` 属性进行标记，它们的输入是 `syn::DeriveInput` 类型，输出是 `proc_macro::TokenStream` 类型。

声明宏一般定义在项目的某个文件中，但是派生宏一般都是定义在一个独立的包中，然后引入使用

```
cargo new derive-macro --lib # 创建一个独立的package
```

```bash
# 在Cargo.toml中写入必备的依赖
[dependencies]
syn = "1.0"
quote = "1.0"
proc-macro2 = "1.0"

[lib]
proc-macro = true
```

```rust
// 在lib.rs中编写宏代码

// 导入所需库
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// 使用 proc_macro_derive 属性标记自定义派生宏，并将宏命名为 simple_debug_derive。
#[proc_macro_derive(SimpleDebug)]
pub fn simple_debug_derive(input: TokenStream) -> TokenStream {
    // 将输入的 TokenStream 解析为 DeriveInput 结构体。
    let input_ast = parse_macro_input!(input as DeriveInput);
    // 从 DeriveInput 结构体中提取类型的名称。
    let name = &input_ast.ident;

    // 使用 quote 宏构造实现 SimpleDebug 的代码。
    let expanded = quote! {
        // 为指定类型实现 std::fmt::Debug trait。
        impl std::fmt::Debug for #name {
            // 实现 fmt 方法。
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // 使用 write! 宏将类型名称写入 Formatter。
                write!(f, "Instance of {}", stringify!(#name))
            }
        }
    };

    // 将生成的代码转换为 TokenStream 并返回。
    expanded.into()
}
```

```bash
# 在其它包中引如定义好的包
[dependencies]
derive-macro = { path = "../derive-macro" }
```

```rust
// 正常使用即可
use derive_macro::SimpleDebug;

// 通过 #[derive(SimpleDebug)] 语法应用自定义派生宏
#[derive(SimpleDebug)]
struct TestStruct;

fn main() {
    let test_instance = TestStruct;
    // 调用 Debug trait 的实现
    println!("{:?}", test_instance);
}
```

2. 属性宏（Attribute Macros）：

属性宏类似于编译器注解，可以应用于项（如函数、结构体、模块等）。属性宏接收被注解的项的定义，然后可以根据需要修改或扩展这些项的行为。属性宏使用 `#[proc_macro_attribute]` 属性进行标记，输入为 `proc_macro::TokenStream`（表示属性参数）和 `syn::Item`（表示被注解的项），输出也是 `proc_macro::TokenStream` 类型。

3. 函数宏（Function-like Macros）：

函数宏看起来像函数调用，但在编译时展开。这类宏可以用于生成代码、实现编译时计算等。函数宏使用 `#[proc_macro]` 属性进行标记，输入和输出都是 `proc_macro::TokenStream` 类型。

为了编写过程宏，通常需要使用 `proc_macro`、`syn` 和 `quote` 这三个库。`proc_macro` 是 Rust 标准库的一部分，用于处理过程宏的输入和输出；`syn` 库用于解析 Rust 代码；`quote` 库用于生成 Rust 代码

## 6.2 Unsafe Rust

### 6.2.1 Unsafe Rust 介绍

Rust 的设计初衷是提供内存安全性和线程安全性，因此其编译器会在编译时检查很多潜在的内存和线程安全问题。然而，在某些情况下，编程者需要执行编译器默认不允许的操作，如直接操作内存或调用不安全的外部函数。在这种情况下，Unsafe Rust 提供了一种途径，让程序员能够明确地执行这些操作

要使用 Unsafe Rust，需要在代码块中使用 `unsafe` 关键字。在这个 `unsafe` 代码块内，你可以执行以下操作：

1. 解引用裸指针（raw pointers）：裸指针是一种没有 Rust 编译器检查的指针，类似于 C 语言中的指针。裸指针分为两类：不可变的 `*const T` 和可变的 `*mut T`。
2. 调用不安全的函数或方法：这些函数或方法需要在其签名中使用 `unsafe` 关键字。通常，不安全的函数或方法执行了编译器无法验证的操作，如调用底层系统 API 或直接修改内存。
3. 访问或修改可变静态变量：Rust 中的静态变量具有固定的内存地址，它们在整个程序执行期间都是有效的。可变静态变量可能导致数据竞争，因此访问或修改它们需要在 `unsafe` 代码块中进行。
4. 实现不安全的 trait：这些 trait 需要在其定义中使用 `unsafe` 关键字。实现不安全的 trait 通常表示它们需要满足某些编译器无法验证的约束。

使用 Unsafe Rust 时需要特别小心，因为它可能导致内存安全问题和未定义行为。在编写 Unsafe Rust 代码时，程序员需要确保代码符合 Rust 的安全性约束。尽量将不安全的代码封装在安全的抽象中，以便在程序的其他部分使用安全的接口

### 6.2.2 Unsafe Rust 编程

下面是 Unsafe Rust 中每种情况的代码示例：

1. 解引用裸指针

```rust

fn main() {
    let x = 10;
    let y = &x as *const i32;
    let z = 0x12345678 as *const i32; // 假设这是一个无效的内存地址

    unsafe {
        println!("Value of x: {}", *y); // 输出 "Value of x: 10"
        // println!("Value at address 0x12345678: {}", *z); // 不安全！可能导致未定义行为
    }
}
```

2. 调用不安全的函数或方法：

```rust

unsafe fn unsafe_function() {
    // 执行不安全的操作，如直接操作内存或调用底层系统 API
}

fn main() {
    unsafe {
        unsafe_function(); // 调用不安全函数
    }
}
```

3. 访问或修改可变静态变量：

```rust

static mut COUNTER: i32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    increment_counter();
    unsafe {
        println!("Counter value: {}", COUNTER); // 输出 "Counter value: 1"
    }
}
```

4. 实现不安全的 trait：

```rust

unsafe trait UnsafeTrait {
    unsafe fn unsafe_method(&self);
}

struct MyStruct {
    value: i32,
}

unsafe impl UnsafeTrait for MyStruct {
    unsafe fn unsafe_method(&self) {
        let mut raw_ptr = self as *const Self as *mut Self;
        (*raw_ptr).value = 42;
    }
}

fn main() {
    let my_struct = MyStruct { value: 10 };

    unsafe {
        my_struct.unsafe_method();
    }
}
```

## 6.3 课后习题

1. 尝试编写一个声明宏和一个过程宏，体验它们的异同
2. 结合你过去的编码经验，你是怎么理解 Rust 中的不安全问题的，安全问题有被扩大吗？
