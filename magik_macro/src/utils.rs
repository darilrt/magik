use std::vec;

use quote::{quote, quote_spanned};
use syn::{ItemStruct, Stmt, parse_quote_spanned, spanned::Spanned};

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
    let parser = magik::Parser::new(input);

    parser
        .collect::<Result<Vec<magik::TemplateData>, String>>()
        .unwrap_or_else(|err| panic!("Template parsing error: {}", err))
}

pub fn compile_template(
    tmp: &Vec<magik::TemplateData>,
    struct_item: &ItemStruct,
) -> proc_macro2::TokenStream {
    let mut quotes = vec![];

    let capacity = tmp.len();

    quotes.push(quote! {
        use std::borrow::Cow;
        let mut magik__result = Vec::with_capacity(#capacity);
    });

    for data in tmp {
        match data {
            magik::TemplateData::String(html) => {
                quotes.push(quote! {
                   magik__result.push(Cow::Borrowed(#html));
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
                        .to_compile_error();
                    }
                };

                if code.stmts.is_empty() {
                    continue; // Skip empty code blocks
                }

                // call a function to check if block returns a value
                if is_block_returning_value(&code) {
                    let mut stmts = code.stmts.clone();
                    let last_stmt = stmts.pop().unwrap();

                    let new_last = match last_stmt {
                        Stmt::Expr(expr, None) => Stmt::Expr(
                            syn::Expr::Call(parse_quote_spanned! {expr.span() =>
                                magik__render_and_validate(&#expr)
                            }),
                            None,
                        ),
                        other => other,
                    };

                    let new_block = syn::Block {
                        brace_token: code.brace_token,
                        stmts: {
                            let mut stmts2 = stmts;
                            stmts2.push(new_last);
                            stmts2
                        },
                    };

                    quotes.push(quote_spanned! {
                        code.span() => magik__result.push(Cow::Owned(
                            #new_block
                        ));
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

    let struct_name = &struct_item.ident;
    let generics = &struct_item.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        mod __hidden {
            use magik::Choosable;
            use super::#struct_name;

            #[inline(always)]
            fn magik__render_and_validate<'a, T: magik::Renderable>(value: &'a T) -> String {
                value.render()
            }

            pub fn magik__render #impl_generics(props: &#struct_name #ty_generics) -> String #where_clause {
                #(#quotes)*
                magik__result.concat()
            }
        }
    }
}
