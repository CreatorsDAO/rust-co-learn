# 文档说明

本文档的行文逻辑：先介绍一些基础知识，然后再层层递进。 原则是尽量不在前面内容中引入比较高阶的知识点。

但是为了照顾到部分同学觉得前期太简单，所以列出了扩展资料

# 模块一：了解Rust

## 1 .1 安装Rust

#### 1.1.1 安装Rust

`rustup`是一个管理Rust版本以及相关工具的命令行工具，你可以通过它来安装Rust开发环境

[在 Linux 或 macOS 上安装 `rustup`](https://rustwiki.org/zh-CN/book/ch01-01-installation.html#在-linux-或-macos-上安装-rustup)

[在 Windows 上安装 `rustup`](https://rustwiki.org/zh-CN/book/ch01-01-installation.html#在-windows-上安装-rustup)

#### 1.1.2 更新和卸载

```
rustup update # 更新
```

```
rustup self uninstall # 卸载
```

#### 1.1.3 rustc

rustc 是Rust的编译器，如下是一些使用案例：

**查看Rust版本**

```
rustc --version # 查看已安装的Rust的版本
```

**编译Rust代码**

使用rustc来直接编译代码为二进制程序，然后运行，例如：

```
mkdir module-one # 随便创建一个文件夹
cd module-one 
touch main.rs # 随便创建一个.rs结尾的文件
```

rust-co-learn/module-one/main.rs

```
// 写一个简单的main函数
fn main() {
    println!("Hello Rust")
}
```

**编译和运行**

```
rustc main.rs
ls
main    main.rs # `main`为编译后的可执行程序
./main # 使用 `./filename` 直接运行程序
Hello Rust # 输出结果
```

**扩展资料**

1. [官方文档关于rustc的介绍](https://rustwiki.org/zh-CN/book/ch01-02-hello-world.html)

## 1.2 使用Cargo

`Cargo`是 Rust 的构建系统和包管理器,非常强大，类似于python的包管理器pip。可以用它来创建、编译和运行Rust项目。以下是一个例子：

```
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

1. [官方文档中使用Cargo构建项目的详细介绍](https://rustwiki.org/zh-CN/book/ch01-03-hello-cargo.html)
2. [Cargo Book：Cargo使用大全](https://doc.rust-lang.org/cargo/)
3. [Rust中的crate与项目管理](https://zhuanlan.zhihu.com/p/614506900)

## 1.3 Rust基础知识

### 1.3.1 常量和变量、可变性和不变性

```
 // 1 常量
    // 使用 const 声明; 常量名称使用大写字母; 显式标注类型

    const RUST: &str = "rust";
    const WEIGHT: u64 = 100;

    println!("{},{}",RUST,WEIGHT);

    // 2 变量
    // 使用let 声明,大多数情况下，编译器可以根据上下文推断变量类型

    let name = "rust";
    let age: u32 = 13;

    println!("{},{}",name,age);

    // 3 不变性
    // Rust中变量默认不可变，若需修改变量，需要使用mut关键字声明变量具有可变性

    let _language = "go";
    // _language = "rust"; 无法修改

    // 4 可变性
    // 通过 mut 声明变量

    let mut language = "go";
    language = "rust";

    println!("{}", language);
```

**扩展资料**

1. [官方文档关于变量遮蔽的介绍](https://rustwiki.org/zh-CN/book/ch03-01-variables-and-mutability.html)
2. [通过可变容器让变量获得可变性](https://zhuanlan.zhihu.com/p/611487702)

### 1.3.2 基础数据类型

Rust是强类型语言，每个值都有确切的类型

#### 1.3.3.1 标量类型

```
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
```

**扩展资料**

1. [Rust官方文档关于基础类型的详细介绍](https://rustwiki.org/zh-CN/book/ch03-02-data-types.html)

#### 1.3.3.2 复合类型

Rust中的复合类型主要有元组和数组

```
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
2. [关于Rust中类型与表达式的详细介绍，此课程为付费课程，但是强烈推荐](https://time.geekbang.org/course/detail/100060601-289993)

### 1.3.3 进阶数据类型

#### 1.3.3.1 字符串

Rust中的字符串比较复杂，有多种形式，适用于不同的场景。核心是需要掌握&str和String

Rust在编译代码时需要在编译期就能够确定类型的大小，而字符串str本身是动态大小的，因而日常中我们更多使用的是字符串的引用&str和String

```
   // 1 &str
    // 字符串字面量实际上存放在程序的只读数据段中，在程序运行时会被加载到内存中读取
    let s = "Hello Rust";
    let mut s1 = "Hello Go";

    s1 = "Hello Rust";
    println!("{}", s1);

    // 2 String
    // String 通过指针指向存放在堆上的字符串

    let s2 = String::from("Hello Rust");

    // String 有三个word：ptr、len、cap,可以直接通过方法访问

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

#### 1.3.3.2 引用

Rust中的引用类型是一等公民，从可变性上可以分为可变引用和不可变引用

```
// 1 不可变借用
    let num = 42;
    let immutable_s = &num;

    // 2 不可变借用
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

1.[Rust中引用和指针的区别](https://zhuanlan.zhihu.com/p/614970269)

2.[官方文档中对引用的更多介绍](https://rustwiki.org/zh-CN/book/ch04-02-references-and-borrowing.html)

#### 1.3.3.3 集合

两个重要的集合Vec和HashMap

```
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
    // 有三个word

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

#### 1.3.3.4 结构体

```
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

    // 为结构体实现方法
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

    // 方法调用
    let r = Rust::new();

    Rust::print();

    // 访问结构体成员
    println!("{:?}", r.0)
```

**扩展资料**

1. [官方文档对于结构体的介绍](https://rustwiki.org/zh-CN/book/ch05-00-structs.html)

#### 1.3.3.5 枚举

```
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

#### 1.3.3.6 函数

```
 // 1 函数定义

    // 没有参数和返回值
    fn foo() {
        println!("foo")
    }

    // 有参数和返回值

    fn bar(s: &str) -> String {
        String::from(s)
    }

    // 参数类型必须显式声明，比如引用或者可变性

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

#### 1.3.3.7 闭包

```
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

#### 1.3.3.8 泛型

Rust语言支持泛型编程，在实际操作中会大量涉及到泛型。泛型提供了抽象能力，让代码复用性更强。泛型一般和其它数据结构结合使用

```
    // 1 泛型参数的表示

    // 泛型参数一般用大写字母`T`表示,多个泛型参数可以使用多个大写字母

    // 可以把泛型当作自定义类型，必须先声明才能使用

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

Rust程序在书写上并没有太复杂的结构，循环和模式匹配基本能够满足绝大多数场景需求

#### 1.3.4.1 循环

Rust有三种循环结构for循环，loop和while

```
 // 1 使用for循环遍历集合
    // 注意：Rust中的for循环本质上是迭代器的语法糖
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

#### 1.3.4.2 模式匹配

Rust中的模式匹配指的是结构上的匹配，最常用有match、while let 、let 、if let 

```
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

Rust中的注释主要包括文档注释，多行注释和单行注释

```
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

## 1.4 练习

# 模块二：Rust核心知识

## 1 所有权机制

所有权与字符串

所有权与动态类型

所有权共享

## 2 借用和生命周期



## 3 trait与泛型

标准库中的主要trait

泛型、自定义数据结构和trait

## 4 项目练习

# 模块三：Rust进阶知识

## 1 迭代器

## 2 智能指针

## 3 项目练习

# 模块四：Rust项目基础

## 1 错误处理

## 2 项目管理

### 2.1 crate

### 2.2 工作空间

## 3 测试

### 3.1 单元测试

### 3.2 文档测试

### 3.3 集成测试

## 4 项目练习

# 模块五：异步编程和无畏并发

## 1 并发和异步编程基础

## 2 Rust并发和异步编程

## 3 项目实战

## 4 项目练习

# 模块六：Rust内容扩展（选学）

## 1 宏编程

## 2 Unsafe Rust

## 3 阅读材料



