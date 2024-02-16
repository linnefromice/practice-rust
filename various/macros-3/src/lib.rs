use proc_macro::TokenStream;

mod functions;

#[proc_macro]
pub fn generate_something(input: TokenStream) -> TokenStream {
    functions::generate_something(input)
}
