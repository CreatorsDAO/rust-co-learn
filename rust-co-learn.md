# 文档说明

本文档的行文逻辑：先介绍一些基础知识，然后再层层递进。 原则是尽量不在前面内容中引入比较高阶的知识点。

但是为了照顾到部分同学觉得前期太简单，所以列出了扩展资料

# 模块一：初识Rust

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

### 1.3.1 变量和可变性

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

两个重要的集合Vec和HashMap，这里的集合指的是它们都聚集了多个同类型的元素

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

Rust是无GC（garbage collection）的语言，对于堆内存的管理主要通过栈来实现。具体而言就是通过借用检查和所有权机制来实现。核心规则如下

**所有权规则：**

1. 每个值都有一个所有者（owner）
2. 每个值在任一时刻只有一个所有者
3. 当所有者（变量）离开作用域时，它所拥有的值将被丢弃

**Rust 的借用规则：**

1. 同一个作用域中，一个资源只有一个可变**借用**（&mut T），但拥有可变**借用**（&mut T）后就不能有不可变**借用**（&T）

2. 同一个作用域中，一个资源可以有多个不可变**借用**（&T），但拥有不可变**借用**（&T）后就不能有可变**借用**（&mut T）
3. **借用**在离开作用域后释放。

## 2.1 所有权机制

### 2.1.1 固定尺寸类型

固定尺寸类型是指那些在编译期就可以确定大小的类型。Rust中主要的固定尺寸类型如下：

| 类型     | 描述                             |
| -------- | -------------------------------- |
| 基本类型 | 整数、浮点数、布尔值和字符类型等 |
| 复合类型 | 数组、元组等                     |
| 指针类型 | 引用和裸指针、函数指针等         |
| ...      | ...                              |

```
// 1 所有权与基本类型

    // 下面的每个值都只有一个所有者

    let owner1 = 42;

    let owner11 = owner1; // owner是一个新的所有者，它的值是 owner1值的复制品，owner1仍然是一个有效的所有者
    println!("{}", owner1); // 42,可以通过 owner1 使用值

    // 现在有两个值和对应的两个有效的所有者，owner1 和 owner11

    println!("owner1 addr {}", owner1);
    println!("owner11 addr {}", owner11);

    // 可以看到值的地址也是不相同（佐证owner11和owner1各拥有一个值）
    // 对于值42来说，它只有一个所有者，因此现在有两个42的值，并且它们的地址是不同的

    println!("owner1 addr {:p}", &owner1); // 0x7ff7b404dd90
    println!("owner11 addr {:p}", &owner11); // 0x7ff7b404dd94

    let owner2 = 42.0;
    let owner3 = true;

    {
        let owner4 = '4'; // ‘4’ 这个值的所有者 `owner4` 在离开作用域时，值会被丢弃
    }

    // println!("{}", owner4) // 无法再使用 owner4,因为它已经被丢弃

    // 2 所有权与复合类型

    let arr_owner: [i32; 3] = [1, 2, 3];
    let tuple_owner = (32, true, 42.0);

    // 3 所有权与指针类型

    // 这里所说的指针是指指向某个内存地址的变量类型，包括引用、裸指针以及函数指针

    // 3.1 字符串的引用

    let ptr_owner = "rust";
    let num = 42;

    // 注意: ptr_owner 是字符串引用的所有者，而不是字符串的所有者，这里的`值`就是引用本身

    let ptr_copy = ptr_owner; // 此处所有者 ptr_copy 的值是 ptr_owner 的值的复制品，ptr_owner 仍然是一个有效的所有者

    // 由于 ptr_owner 和 ptr_copy 的值都是指向相同值的引用，所以它们指向的内存地址是相同的
    println!("{:p}", ptr_owner); // 0x10ac12004
    println!("{:p}", ptr_copy); // 0x10ac12004

    // 3.2 基本类型的裸指针
    // 在rust中使用 `as *const T` 可以将引用转为裸指针

    let ptr_owner2 = &num as *const i32;

    // 3.3 函数指针

    fn func() -> i32 {
        0
    }
    let func_ptr = func;
```

### 2.1.2 动态尺寸类型

Rust是一门静态类型语言，这意味着所有变量在编译期必须是大小确定的，但是在实际场景中，比如字符串和切片类型的大小取决于运行时的具体情况。Rust对这类数据的处理方法是使用它们的指针（引用），而不是数据本身，众所周知，一个类型不管多大，对应的指针（引用）大小是确定的

| 类型         | 描述                                                         |
| ------------ | ------------------------------------------------------------ |
| 字符串类型   | str, 本质上是一个u8类型的数据序列，实际中经常使用的形式：&str 和 String |
| 切片类型     | [T], 它代表类型为 `T` 的元素组成的数据序列：实际中经常使用的形式： Vec<T> |
| trait object | trait object 的大小只有在运行时才能确定（可以先不用了解，关于trait的内容后面会继续讲解） |
| ...          | ...                                                          |

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

    // 1.2 对于存放在堆上的字符串，如果把它的所有者赋值给另一个变量，意味着把堆上所有权就会转移给新的所有者
    let heap_ptr_old = String::from("Rust"); //存放在堆上

    let heap_ptr_new = heap_ptr_old;

    // println!("old owner{:?}", heap_ptr_old); // 无法再通过 heap_ptr_old 使用值，因为它已经把数据所有权移交给了新的所有者 heap_ptr_new
    println!("old owner{:?}", heap_ptr_new); // heap_ptr_new 可以正常访问到堆上的数据，并且它是唯一的所有者，当它离开作用域时，堆上的数据也会被丢弃

    {
        let owner_old = String::from("rust");
        let owner_new = owner_old;

        // 在此处离开作用域
    }

    // println!("{:?}", owner_new); 无法再通过 owner_new 使用值，因为它已经被丢弃

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
    // 当数据存放在堆上时，把所有权赋值给另一个变量，意味着把堆上所有权就会转移给新的所有者，堆上的数据本身没有被复制，原来的所有者不再拥有数据
    // 当数据存放在栈上时，把所有权赋值给另一个变量，意味着把栈上的数据复制了一份给新的所有者，原来的所有者仍然拥有原来的数据
```

### 2.1.3 所有权共享

所有权规则更像是对资源的独占，在实际场景中，你可能希望多个角色共享访问某个动态资源。Rust提供了两个容器类型Rc<T>和Arc<T>，可以让你同时让多个变量拥有动态数据的所有权

```
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

        // 使用Rc包裹资源，让内存泄漏
        let container = Rc::new(source); // 引用计数 + 1
                                         //
        let role1 = container.clone(); // 引用计数 + 1
        let role2 = container.clone(); // 引用计数 + 1

        // 当变量离开作用域时，role2，role1，container相继离开作用域时，引用计数都会-1，当引用计数为0时，堆上的数据才会被释放
    }
```

## 2.2 借用和生命周期

根据是否拥有数据的所有权，Rust中的变量可以分为拥有所有权的变量和没有所有权的变量

而拥有所有权的变量我们只需要搞明白所有权规则就行了，所以有所有权的变量生命周期并没有难点

但对与没有所有权的变量也就是引用（借用），比较麻烦，但也主要是在参数传递的过程中

```
 // 1 有所有权的变量和没有所有权的变量

    // 有所有权

    let have_ownership = String::from("rust");

    // 没有所有权
    let have_no_ownership = "rust"; // 字符串切片的引用

    // 有所有权
    let num = 42;

    // 没有所有权
    let num_ptr = &num;

    // 2 变量的生命周期 （不管是有所有权的变量还是没所有权的变量）: 从声明开始，到离开作用域结束

    {
        let x = 32;
        println!("{}", x);

        {
            // 引用的生命周期
            let x_ptr = &x;

            // x_ptr 在离开作用域时，生命周期结束，值会被丢弃
        }

        // println!("{}", x_ptr); // 无法再使用 x_ptr,因为它已经被丢弃

        // x 在离开作用域时，生命周期结束，值会被丢弃
    }

    // println!("{}", x); // 无法再使用 x,因为它已经被丢弃

    // 3 使用泛型生命周期参数显式标注参数生命周期

    // 声明一个函数，参数是引用类型，返回值也是引用类型
    // 它无法编译通过，因为编译器无法推断出参数和返回值的生命周期
    // 事实上，i32这种非常轻量的类型，我们直接传值就可以了，不需要传引用

    // fn foo(x: &i32, &y: &i32) -> &i32 {
    //     println!("{}", x);
    // }

    // 如果是比较大的类型，比如结构体，我们就需要传引用了

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

    // fn bar(x: &Foo, y: &Foo) -> &Foo {
    //     x
    // }

    // 但是Rust到底在担心什么情况呢，我们来看看下面的代码

    {
        // 假设下面的函数可以编译通过

        // fn bar(x: &Foo, y: &Foo) -> &Foo {
        //     let f3 = &Foo {
        //         x: 32,
        //         y: (32, true),
        //         z: String::from("rust"),
        //     };

        //     f3
        // }

        // 定义两个生命周期不同的变量

        let mut f1 = &Foo {
            x: 32,
            y: (32, true),
            z: String::from("rust"),
        };

        {
            let f2 = &Foo {
                x: 32,
                y: (32, true),
                z: String::from("rust"),
            };

            // 调用函数，传入两个引用
            // 前面假设函数可以通过，会返回函数内部变量的引用

            // let f4 = bar(f1, f2);
            // 将返回结果赋值给f4,显然是不合理的，因f3在函数结束时会被丢弃，f4就会指向一个无效的内存地址
        }

        // 现在使用生命周期参数，来标注参数和返回值的生命周期
        // 注意Rust中的生命周期参数是以单引号开头的小写字母，也是一种泛型，但通常使用 'a，‘b，‘c这样的字母
        // 生命周期参数也像泛型一样需要先声明才能使用
        // 使用了生命周期参数后，编译器通过了，注意这里我们只是告诉编译器返回值的生命周期是y的生命周期，其他的什么都没做
        // 这实际上就告诉编译器我们没有返回局部变量，所以不会有悬垂指针的问题

        fn bar<'a, 'b>(x: &'a Foo, y: &'b Foo) -> &'b Foo {
            y
        }

        // 如果不确定返回哪个参数的引用，可以使用下面的写法,`'b: 'a` 表示'b 的生命周期要不短于'a的生命周期

        fn far<'a, 'b: 'a>(x: &'a Foo, y: &'b Foo) -> &'a Foo {
            if x.x > y.x {
                x
            } else {
                y
            }
        }

        {
            let f3 = &Foo {
                x: 32,
                y: (32, true),
                z: String::from("rust"),
            };

            let f3 = bar(f1, f3);

            f1 = f3;

            println!("{}", f1.x);

            // 调用far
            // 我们在声明函数的时候，要求‘b不短于’a,但是f3的生命周期比f1的生命周期短，但依然会成功执行，这是为什么？
            let f4 = far(f1, f3);
            let f5 = far(f3, f1);

            // 借用检查器
            // 实际上，当我们标注了生命周期以后，Rust编译器会进行计算，而不是简单的检查签名中参数生命周期和参数声明时的生命周期是否一致
            // 生命周期计算过程：Rust先会取所有参数的周期，记录代码最后的覆盖位置，假设记为x，然后对两个参数的生命周期求交集，记录最早结束位置，假设记为y
            // x < = y,编译器通过检查，x > y 编译器会报错
        }
    }
```

## 2.3 trait

### 2.3.1 trait 概况简介

Rust 中的 trait 是一种定义行为的方式，它类似于其他语言中的接口或抽象类。一个 trait 定义了一组方法的签名，这些方法可以在其他类型中实现，并允许这些类型表现出特定的行为

Rust中的trait一方面约定类型的共同行为，但另一方面也经常以是否实现了某个trait作为对类型的约束

Rust中的trait非常强大，它几乎和所有类型相关，你可以通过标准库中的大量定义好的trait来学习类型有哪些方法（可以执行哪些行为），同时，也可以自定义triat，粘合不同的类型，构建自己的项目

```
 // 1 trait类型

    // 1.1 空trait

    trait A {}

    // 1.2 有方法的trait

    trait B {
        fn method1(&self);
        fn method2(&self);

        // ...
    }

    // 1.3 有关联类型的trait

    trait C {
        type Type;

        fn method1(&self) -> Self::Type;
    }

    // 1.4 有默认实现的trait

    trait D {
        // 这个方法是默认实现
        fn method1(&self) {
            println!("method1");
        }
        fn method2(&self);
    }

    // 2 如何实现 trait

    // 2.1 手动实现

    struct Book;

    trait Read {
        fn read(&self);
    }

    // 使用impl语法
    impl Read for Book {
        fn read(&self) {
            println!("read book");
        }
    }

    // 注意和为类型实现方法做区别

    impl Book {
        fn read(&self) {
            println!("read book");
        }
    }

    // 2.2 使用宏实现
    // 标准库和第三方库中一些trait可以通过派生宏来实现

    #[derive(Default, Clone)]
    struct Student {
        name: String,
        age: u32,
    }

    // 可以直接调用trait提供的方法
    let s = Student::default();
    let s1 = s.clone();

    // 3 trait约束

    // 3.1 trait继承，如下要求类型必须先实现 Clone和Default trait才能是实现 S trait
    trait S: Clone + Default {
        fn get_age(&self) -> u32;
    }

    impl S for Student {
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    // trait 作为函数参数的约束：只有实现了S trait的泛型才能作为下列函数的参数

    fn person_age<T: S>(s: T) -> u32 {
        s.get_age()
    }

    struct Teacher {
        name: String,
        age: u32,
    }

    let t = Teacher {
        name: "teacher".to_string(),
        age: 30,
    };

    // person_age(t); // t没有实现S trait，所以不能作为参数
    person_age(s); // 可以调用
```

标准库中预导入了很多trait，可以直接在文件中使用而不用` use`导入，你可以大概看一下下列表格，消除对 trait的陌生感

![image-20230302004216125](/Users/qinjianquan/Career/rust/image/4.3.png)

如下是不同场景下经常使用的trait

![img](https://pic2.zhimg.com/80/v2-3d9eb5c90181e8e59e1bb0d062107a39_1440w.webp)

### 2.3.2 Trait与基础类型

### 2.3.2 Trait与进阶类型

### 2.3.3 Trait 对象

## 2.4 项目练习

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



