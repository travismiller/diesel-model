extern crate proc_macro;
#[macro_use]
extern crate quote;

use crate::quote::ToTokens;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ExprAssign, ExprPath, ItemStruct, Token};

fn wrap_in_dummy_mod(item: impl quote::ToTokens) -> impl quote::ToTokens {
    quote! {
        #[allow(unused_imports)]
        const _: () = {
            #item
        };
    }
}

struct ModelArgsInput {
    backend: ExprPath,
    schema: ExprPath,
}

impl ModelArgsInput {
    pub fn new(input: ParseStream) -> syn::Result<Self> {
        let fields = input.parse_terminated(ExprAssign::parse)?;

        Ok(Self {
            backend: Self::expr_path_for("backend", &fields)?,
            schema: Self::expr_path_for("schema", &fields)?,
        })
    }

    fn expr_path_for(
        ident: &str,
        fields: &Punctuated<ExprAssign, Token!(,)>,
    ) -> syn::Result<ExprPath> {
        for field in fields {
            let left = field.left.as_ref();
            let right = field.right.as_ref();

            match (left, right) {
                (Expr::Path(left), Expr::Path(right)) => {
                    if ident == left.to_token_stream().to_string() {
                        return Ok((*right).clone());
                    }
                }
                _ => panic!(
                    "Invalid model argument: {} = {}",
                    left.to_token_stream().to_string(),
                    right.to_token_stream().to_string(),
                ),
            };
        }

        panic!("Model argument not found: {}", ident);
    }
}

impl std::fmt::Debug for ModelArgsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModelArgsInput(db: {}, schema: {})",
            self.backend.to_token_stream().to_string(),
            self.schema.to_token_stream().to_string(),
        )
    }
}

impl Parse for ModelArgsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Self::new(input)
    }
}

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_attr = parse_macro_input!(attr as ModelArgsInput);
    let backend = &input_attr.backend;
    let schema = &input_attr.schema;

    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &input_struct.ident;

    let model_impl = wrap_in_dummy_mod(quote! {
        use diesel::associations::HasTable;
        use diesel::dsl::Select;
        use diesel::QueryDsl;
        use diesel::Table;

        impl diesel_model::Model for #struct_name {
            type All = Select<#schema::table, Self::AllColumns>;
            type AllColumns = <#schema::table as Table>::AllColumns;
            type Backend = #backend;

            fn all() -> Self::All {
                #schema::table.select(#schema::all_columns)
            }
        }
    });

    let mut output_struct = TokenStream::new().into();
    input_struct.to_tokens(&mut output_struct);
    model_impl.to_tokens(&mut output_struct);

    output_struct.into()
}
