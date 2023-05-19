use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ext::IdentExt};
use quote::quote;

#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput); // ItemStruct?
    match generate_getter(input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_getter(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(&input.ident, "Must be struct type"))
        },
    };

    // 生成する method の TokenStream を格納する Vec
    let mut get_fields = Vec::new();
    for field in &struct_data.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let method_name: proc_macro2::TokenStream = format!("get_{}", ident.unraw().to_string())
            .parse()
            .unwrap();

        get_fields.push(quote! {
            pub fn #method_name(&self) -> #ty {
                self.#ident
            }
        });
    };

    let struct_name = &input.ident;
    let (impl_generics, _, where_clause) = &input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics #struct_name #where_clause {
            #(#get_fields)*
        }
    };

    Ok(expanded.into())
}
