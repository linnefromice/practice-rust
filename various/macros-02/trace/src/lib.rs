use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, Stmt};
use quote::quote;

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let ident = &ast.sig.ident;

    let enter = quote! {
        println!("Enter: {}", stringify!(#ident));
    };
    let enter: TokenStream = enter.into();
    let enter = parse_macro_input!(enter as Stmt);

    let mut body = quote! {};
    for s in &ast.block.stmts {
        body = quote! {
            #body
            #s
        };
    }
    let body = quote! {
        let body = || { #body };
    };
    let body: TokenStream = body.into();
    let body = parse_macro_input!(body as Stmt);
    
    let exit = quote! {
        {
            let ret = body();
            println!("Exit: {}", stringify!(#ident) );
            ret
        }
    };
    let exit: TokenStream = exit.into();
    let exit = parse_macro_input!(exit as Stmt);
    
    ast.block.stmts.clear();
    ast.block.stmts.push(enter);
    ast.block.stmts.push(body);
    ast.block.stmts.push(exit);

    let gen = quote!(
        #ast
    ); // TokenStream in proc-macro2
    
    gen.into()
}
