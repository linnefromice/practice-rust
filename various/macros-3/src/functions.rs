use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub struct GenerateSomethingArgs {
    func_name: syn::LitStr,
    request_args: syn::Type,
    response: Option<syn::Type>,
}
impl syn::parse::Parse for GenerateSomethingArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let func_name: syn::LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let request_args: syn::Type = input.parse()?;
        let response = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(GenerateSomethingArgs {
            func_name,
            request_args,
            response,
        })
    }
}
pub fn generate_something(input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input as GenerateSomethingArgs);
    generate_something_internal(args).into()
}

pub fn generate_something_internal(args: GenerateSomethingArgs) -> proc_macro2::TokenStream {
    let func_name = format_ident!("{}", args.func_name.value());
    let request_args = args.request_args;
    let response = args.response;
    quote! {
        fn #func_name(request: #request_args) -> #response {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;
    use rust_format::{Formatter, RustFmt};

    use super::*;

    #[test]
    fn test_snapshot_generate_something() {
        let input = quote! {"something", String, String};
        let args: syn::Result<GenerateSomethingArgs> = syn::parse2(input);
        let generated = generate_something_internal(args.unwrap());
        let formatted = RustFmt::default()
            .format_str(generated.to_string())
            .expect("rustfmt failed");
        assert_snapshot!("snapshot__generate_something", formatted);
    }
}
