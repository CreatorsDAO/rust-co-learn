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
