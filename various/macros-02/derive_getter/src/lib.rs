use proc_macro::TokenStream;

#[proc_macro_derive(Getter)]
pub fn getter_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
