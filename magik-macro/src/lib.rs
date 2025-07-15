extern crate proc_macro;

mod check_return;
mod utils;

use check_return::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse::Parse, parse_macro_input};

use crate::utils::{compile_template, parse_template, read_template_file};

#[proc_macro_attribute]
pub fn template_str(attr: TokenStream, item: TokenStream) -> TokenStream {
    let source = parse_macro_input!(attr as syn::LitStr);
    let input = source.value();

    let item = parse_macro_input!(item as ItemStruct);

    let tmp = compile_template(&parse_template(&input), item.ident.clone());

    implement_renderable(&item, &tmp)
}

#[proc_macro_attribute]
pub fn template(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as Attributes);
    let path = input.path;

    let item = parse_macro_input!(item as ItemStruct);

    let code = compile_template(
        &parse_template(read_template_file(&path).as_str()),
        item.ident.clone(),
    );

    implement_renderable(&item, &code)
}

fn implement_renderable(item: &ItemStruct, code: &proc_macro2::TokenStream) -> TokenStream {
    let name = &item.ident;

    quote! {
        #item

        impl magik::Renderable for #name {
            fn render(self) -> String {
                #code
                __hidden::magik__render(&self)
            }
        }
    }
    .into()
}

struct Attributes {
    path: String,
}

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(val) = input.parse::<syn::LitStr>() {
            return Ok(Attributes { path: val.value() });
        }

        let key: syn::Ident = input.parse()?;
        let _ = input.parse::<syn::Token![=]>()?;
        let path: syn::LitStr = input.parse()?;

        if key != "path" {
            return Err(syn::Error::new_spanned(key, "Expected 'path' attribute"));
        }

        let path_str = path.value();

        if path_str.is_empty() {
            return Err(syn::Error::new_spanned(path, "Path cannot be empty"));
        }

        Ok(Attributes { path: path_str })
    }
}
