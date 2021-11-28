mod error;

use error::{DeriveError, DeriveResult};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AlipayParam)]
pub fn derive_alipay_param(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    impl_map_macro(&input).unwrap()
}

fn impl_map_macro(input: &syn::DeriveInput) -> DeriveResult<TokenStream> {
    let data_struct = match &input.data {
        Data::Struct(data) => data,
        Data::Enum(_) => Err(DeriveError::new("invalid type: Enum"))?,
        Data::Union(_) => Err(DeriveError::new("invalid type: Union"))?,
    };

    let fields_named = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named,
        Fields::Unnamed(_) => Err(DeriveError::new("invalid type: Unnamed"))?,
        Fields::Unit => Err(DeriveError::new("invalid type: Unit"))?,
    };

    let to_field_value_token_streams: Vec<proc_macro2::TokenStream> = fields_named.named.iter().enumerate().map(|(i, field)| {
        let field_name = match &field.ident {
            Some(ident) => syn::Member::Named(ident.clone()),
            None => syn::Member::Unnamed(i.into()),
        };

        return quote! {
            result.insert(stringify!(#field_name).to_string(), alipay::param::Converter::to_field_value(&self.#field_name));
        };
    }).collect();

    let to_primitive_token_streams: Vec<proc_macro2::TokenStream> = fields_named.named.iter().enumerate().map(|(i, field)| {
        let field_name = match &field.ident {
            Some(ident) => syn::Member::Named(ident.clone()),
            None => syn::Member::Unnamed(i.into()),
        };
        let ty = &field.ty;

        return quote! {
            let mut #field_name: Option<#ty> = None;
            if let Some(value) = __optional_map__.get_mut(stringify!(#field_name)) {
                if let Some(value) = std::mem::replace(value, None) {
                    #field_name = Some(alipay::param::Converter::to_primitive(value)?);
                }else {
                    return Err(alipay::error::AlipayError::ConvertError(format!("invalid type: {}", stringify!(#ty).to_owned())));
                }
            }
            let #field_name = #field_name.ok_or(alipay::error::AlipayError::ConvertError(format!("invalid type: {}", stringify!(#ty).to_owned())))?;
        };
    }).collect();

    let to_struct_token_streams: Vec<proc_macro2::TokenStream> = fields_named
        .named
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let field_name = match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            };
            return quote! {
                #field_name,
            };
        })
        .collect();

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics alipay::param::AlipayParam for #struct_name #ty_generics #where_clause {
            fn to_map(&self) -> std::collections::HashMap<String, alipay::param::FieldValue> {
                let mut result: std::collections::HashMap<String, alipay::param::FieldValue> = std::collections::HashMap::new();
                #(#to_field_value_token_streams)*
                result
            }
            
            fn from_map(__map__: std::collections::HashMap<String, alipay::param::FieldValue>) -> alipay::error::AlipayResult<Self> {
                let mut __optional_map__ = std::collections::HashMap::with_capacity(__map__.len());
                for (key, val) in __map__ {
                    __optional_map__.insert(key, Some(val));
                }
                #(#to_primitive_token_streams)*
                Ok(#struct_name { #(#to_struct_token_streams)* })
            }
        }
    }.into())
}
