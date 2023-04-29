// 导入所需库
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
// 使用 proc_macro_attribute 属性标记属性宏，并将宏命名为 show_function_name。
#[proc_macro_attribute]
pub fn show_function_name(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 将输入的 TokenStream 解析为 ItemFn 结构体。
    let input_ast = parse_macro_input!(item as ItemFn);
    // 从 ItemFn 结构体中提取函数的名称。
    let name = &input_ast.sig.ident;

    // 使用 quote 宏构造修改后的函数代码。
    let func = quote! {
        // 重新定义函数。
        fn #name() {
            // 在函数开始时打印函数名称。
            println!("Function name: {}", stringify!(#name));
        }
    };

    // 将生成的代码转换为 TokenStream 并返回。
    func.into()
}
