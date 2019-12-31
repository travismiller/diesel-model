extern crate proc_macro;

#[macro_use]
extern crate quote;

use crate::quote::ToTokens;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ExprPath, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let schema = parse_macro_input!(attr as ExprPath);

    let input_struct = parse_macro_input!(item as ItemStruct);

    let struct_name = &input_struct.ident;

    let model_impl = if let Fields::Named(fields) = &input_struct.fields {
        let type_fields = fields
            .named
            .iter()
            .map(|f| format_ident!("{}", f.ident.as_ref().unwrap()));
        let const_fields = type_fields.clone();

        quote! {
            impl diesel_model::Model for #struct_name {
                type AllColumns = (#(#schema::#type_fields),*);
                type All = diesel::dsl::Select<#schema::table, Self::AllColumns>;
                const ALL_COLUMNS: Self::AllColumns = (#(#schema::#const_fields),*);
            }
        }
    } else {
        panic!("Only named fields are allowed");
    };

    let mut output_struct = input_struct.to_token_stream();
    model_impl.to_tokens(&mut output_struct);

    output_struct.into()
}
