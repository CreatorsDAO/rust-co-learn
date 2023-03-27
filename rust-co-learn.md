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

### 2.1 常量、变量、可变性和不变性

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

扩展资料

1. [官方文档关于变量遮蔽的介绍](https://rustwiki.org/zh-CN/book/ch03-01-variables-and-mutability.html)
2. [通过可变容器让变量获得可变性](https://zhuanlan.zhihu.com/p/611487702)

### 2.2 基础数据类型

Rust是强类型语言，每个值都有确切的类型

#### 2.2.1 标量类型

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

参考资料

1. [Rust官方文档关于基础类型的详细介绍](https://rustwiki.org/zh-CN/book/ch03-02-data-types.html)

#### 2.2.2 复合类型

```
 // 1 元组
    // Rust中的元组可以将各种类型组合起来
    let types = (42, "Rust", true);

    // 可以像访问数组元素一样通过下标索引访问
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

更多参考资料

1. [官方文档中关于复合类型的介绍](https://rustwiki.org/zh-CN/book/ch03-02-data-types.html)
2. [关于Rust中类型与表达式的详细介绍，此课程为付费课程，但是强烈推荐](https://time.geekbang.org/course/detail/100060601-289993)

### 2.3 进阶数据类型

#### 2.3.1 字符串

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

更多参考资料

1. [官方文档中关于字符串的更多解释](https://rustwiki.org/zh-CN/book/ch08-02-strings.html)
2. [一些字符串练习的小例子](https://github.com/rust-lang-cn/rustlings-cn/tree/main/exercises/strings)

#### 2.3.2 引用



#### 2.3.3 Slice

#### 2.3.4 结构体

#### 2.3.5 枚举

#### 2.3.6 函数

#### 2.3.7 闭包

#### 2.3.8 泛型

### 2.4 注释

### 2.5 控制流

2.5.1 模式匹配

## 3 练习

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

## 1 迭代器与闭包

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



