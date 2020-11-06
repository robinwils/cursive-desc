extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, token, Field, Ident, Result, Token};

struct CallbackStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for CallbackStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(CallbackStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}

#[proc_macro_derive(Callback, attributes(cbmap))]
pub fn callback_derive(input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as CallbackStruct);
    let name = input_ast.ident;
    let mut cbmacro_ident: Option<syn::Ident> = None;
    let empty_ident = syn::Ident::new("aa", Span::call_site());

    for field in input_ast.fields {
        if let Some(attr) = field
            .attrs
            .iter()
            .find(|a| a.path.get_ident().unwrap_or(&empty_ident) == "cbmap")
        {
            cbmacro_ident = attr.path.get_ident().cloned();
        }
    }

    if let Some(cbmap_ident) = cbmacro_ident {
        let expanded = quote! {
            impl CallbackMap for #name {
                fn init_callback_map(self: &mut Self) {

                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("struct does not have any field with the cbmacro attribute");
    }
}

#[proc_macro_attribute]
pub fn cursive_callback(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr2 = proc_macro2::TokenStream::from(attr);
    let item2 = proc_macro2::TokenStream::from(item);

    let expanded = quote! {};

    TokenStream::from(expanded)
}
