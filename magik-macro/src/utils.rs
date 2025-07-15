use std::vec;

use quote::{quote, quote_spanned};
use syn::{Ident, spanned::Spanned};

use crate::is_block_returning_value;

pub fn read_template_file(path: &str) -> String {
    let basedir = std::env::current_dir().expect("Failed to get current directory");
    let full_path = basedir.join(path);

    if !full_path.exists() {
        panic!("Template file does not exist at: {}", path);
    }

    std::fs::read_to_string(full_path)
        .unwrap_or_else(|_| panic!("Failed to read template file at: {}", path))
}

pub fn parse_template(input: &str) -> Vec<magik::TemplateData> {
    let mut parser = magik::Parser::new(input);
    let mut result = vec![];

    while let Some(data) = parser.next() {
        result.push(data);
    }

    result
}

pub fn compile_template(
    tmp: &Vec<magik::TemplateData>,
    struct_name: Ident,
) -> proc_macro2::TokenStream {
    let mut quotes = vec![];

    let capacity = tmp.len();

    quotes.push(quote! {
        let mut magik__result = Vec::with_capacity(#capacity);
        // let mut result = String::new();
    });

    for data in tmp {
        match data {
            magik::TemplateData::String(html) => {
                quotes.push(quote! {
                   magik__result.push(#html.to_string());
                });
            }
            magik::TemplateData::Code(code) => {
                let code: syn::Block = match syn::parse_str(code) {
                    Ok(expr) => expr,
                    Err(err) => {
                        return syn::Error::new_spanned(
                            code,
                            format!("Error parsing code: {}", err),
                        )
                        .to_compile_error()
                        .into();
                    }
                };

                if code.stmts.is_empty() {
                    continue; // Skip empty code blocks
                }

                // call a function to check if block returns a value
                if is_block_returning_value(&code) {
                    quotes.push(quote_spanned! {
                        code.span() => magik__result.push(magik__render_and_validate(#code));
                    });
                } else {
                    code.stmts.iter().for_each(|stmt| {
                        quotes.push(quote_spanned! {
                            stmt.span() => #stmt
                        });
                    });
                }
            }
        }
    }

    quote! {
        mod __hidden {
            use magik::Choosable;
            use super::#struct_name;

            #[inline(always)]
            fn magik__render_and_validate<T: magik::Renderable>(value: T) -> String {
                value.render()
            }

            pub fn magik__render(props: &#struct_name) -> String {
                #(#quotes)*
                magik__result.concat()
            }
        }
    }
}
