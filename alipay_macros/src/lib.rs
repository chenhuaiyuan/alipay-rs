mod error;
use error::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AlipayParams)]
pub fn derive_alipay_param(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    impl_map_macro(&input).unwrap()
}

fn impl_map_macro(input: &syn::DeriveInput) -> Result<TokenStream, Error> {
    let data_struct = match &input.data {
        Data::Struct(data) => data,
        Data::Enum(_) => return Err(Error::new("Must be struct type")),
        Data::Union(_) => return Err(Error::new("Must be struct type")),
    };
    let fields_named = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named,
        Fields::Unnamed(_) => return Err(Error::new("Struct must have field")),
        Fields::Unit => return Err(Error::new("Struct type cannot have punctuation marks")),
    };
    let to_field_value_token_streams: Vec<proc_macro2::TokenStream> = fields_named
        .named
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let field_name = match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            };

            quote! {
                result.insert(stringify!(#field_name).to_string(), self.#field_name.to_alipay_value());
            }
        })
        .collect();

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics alipay_params::AlipayParams for #struct_name #ty_generics #where_clause {
            fn to_alipay_value(self) -> alipay_params::AlipayValue {
                let mut result: std::collections::HashMap<String, alipay_params::AlipayValue> = std::collections::HashMap::new();
                #(#to_field_value_token_streams)*
                alipay_params::AlipayValue::from(result)
            }
        }
    }.into())
}
