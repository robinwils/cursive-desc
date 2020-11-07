extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, parse_macro_input, token, Field, Ident, ImplItem, ItemImpl, Result, Token, Type,
};

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
        let expanded = quote! {};

        TokenStream::from(expanded)
    } else {
        panic!("struct does not have any field with the cbmacro attribute");
    }
}

fn generate_map_insert(item: &ImplItem) -> Option<proc_macro2::TokenStream> {
    match item {
        ImplItem::Method(method_item) => {
            let method_name = method_item.sig.ident.clone();
            Some(quote! {
                map.insert(stringify!(#method_name).to_string(), Self::#method_name);
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

    let map_inserts = cb_impl.items.iter().map(|item| generate_map_insert(item));

    if let Some(name) = struct_name {
        let expanded = quote! {
            #cb_impl
            impl CallbackMap for #name {
                fn new() -> Self {
                    Self {
                        callbacks: {
                            let mut map: HashMap<String, fn(&mut Cursive)> = ::std::collections::HashMap::new();
                            #(#map_inserts)*
                            map
                        }
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("Not implementing a struct.");
    }
}
