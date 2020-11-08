extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, parse_macro_input, token, Field, Ident, ImplItem, ItemImpl, Result, Token, Type,
};

fn generate_cb_pattern_match(item: &ImplItem) -> Option<proc_macro2::TokenStream> {
    match item {
        ImplItem::Method(method_item) => {
            let method_name = method_item.sig.ident.clone();
            Some(quote! {
                stringify!(#method_name) => Self::#method_name,
            })
        }
        _ => None,
    }
}

#[proc_macro_attribute]
pub fn cursive_callbacks(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cb_impl = parse_macro_input!(item as ItemImpl);

    let struct_name = match *cb_impl.self_ty.clone() {
        Type::Path(tp) => tp.path.get_ident().cloned(),
        _ => None,
    };

    let map_inserts = cb_impl
        .items
        .iter()
        .map(|item| generate_cb_pattern_match(item))
        .filter(|x| x.is_some());

    if let Some(name) = struct_name {
        let expanded = quote! {
            #cb_impl
            impl CallbackRegistry for #name {
                fn new() -> Self {
                    Self {}
                }

                fn get(&self, callback_name: &str) -> fn(&Self, &mut Cursive) {
                    match callback_name {
                        #(#map_inserts)*
                        _ => panic!("No callback named {}", callback_name),
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("Not implementing a struct.");
    }
}
