extern crate proc_macro;

mod check_return;
mod utils;

use check_return::*;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse::Parse, parse_macro_input};

use crate::utils::{compile_template, parse_template, read_template_file};

#[proc_macro_attribute]
pub fn template(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as Attributes);

    let source = if let Some(path) = &input.path {
        read_template_file(path)
    } else if let Some(source) = &input.source {
        source.clone()
    } else {
        panic!("Either 'path' or 'source' attribute must be provided")
    };

    let item = parse_macro_input!(item as ItemStruct);

    let code = compile_template(
        &parse_template(source.as_str()),
        &item,
        input.context.as_deref(),
    );

    implement_renderable(&item, &code)
}

fn implement_renderable(item: &ItemStruct, code: &proc_macro2::TokenStream) -> TokenStream {
    let name = &item.ident;
    let generics = &item.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {

        #item

        impl #impl_generics magik::Renderable for #name #ty_generics #where_clause {
            fn render(&self) -> String {
                #code
                __hidden::magik__render(self)
            }
        }

        impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use magik::Renderable;
                write!(f, "{}", self.render())
            }
        }
    }
    .into()
}

struct Attributes {
    path: Option<String>,
    source: Option<String>,
    context: Option<String>,
}

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(val) = input.parse::<syn::LitStr>() {
            return Ok(Attributes {
                path: None,
                source: Some(val.value()),
                context: None,
            });
        }

        let mut path = None;
        let mut source = None;
        let mut context = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let value: syn::LitStr = input.parse()?;

            match key.to_string().as_str() {
                "path" => {
                    if source.is_some() {
                        return Err(syn::Error::new_spanned(
                            key,
                            "Cannot specify both 'path' and 'source'",
                        ));
                    }
                    path = Some(value.value());
                }
                "source" => {
                    if path.is_some() {
                        return Err(syn::Error::new_spanned(
                            key,
                            "Cannot specify both 'path' and 'source'",
                        ));
                    }
                    source = Some(value.value());
                }
                "context" => {
                    context = Some(value.value());
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "Expected 'path', 'source', or 'context' attribute",
                    ));
                }
            }

            if input.peek(syn::Token![,]) {
                let _ = input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Attributes {
            path,
            source,
            context,
        })
    }
}
