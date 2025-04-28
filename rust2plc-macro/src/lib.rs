use proc_macro::{TokenStream, TokenTree};
use rust2plc::langs::PLCLang;
use syn::{parse_macro_input, ItemFn};

fn parse_attribute_args(
    mut tokens: impl Iterator<Item = TokenTree>,
) -> Option<(PLCLang, Option<String>, Option<String>, Option<String>)> {
    if let Some(TokenTree::Ident(ident)) = tokens.next() {
        let dialect: PLCLang = ident.to_string().as_str().into();
        let mut dsc = None;
        let mut ver = None;
        let mut namespace = None;

        let mut tokens = tokens.peekable();

        while let Some(token) = tokens.next() {
            if let TokenTree::Ident(ident) = token {
                match ident.to_string().as_str() {
                    "description" => {
                        if let Some(TokenTree::Punct(punct)) = tokens.peek() {
                            if punct.as_char() == '=' {
                                tokens.next();
                                if let Some(TokenTree::Literal(lit)) = tokens.next() {
                                    dsc = Some(lit.to_string().trim_matches('"').to_string());
                                }
                            }
                        }
                    }
                    "namespace" => {
                        if let Some(TokenTree::Punct(punct)) = tokens.peek() {
                            if punct.as_char() == '=' {
                                tokens.next();
                                if let Some(TokenTree::Literal(lit)) = tokens.next() {
                                    namespace = Some(lit.to_string().trim_matches('"').to_string());
                                }
                            }
                        }
                    }
                    "version" => {
                        if let Some(TokenTree::Punct(punct)) = tokens.peek() {
                            if punct.as_char() == '=' {
                                tokens.next();
                                if let Some(TokenTree::Literal(lit)) = tokens.next() {
                                    ver = Some(lit.to_string().trim_matches('"').to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Some((dialect, dsc, namespace, ver))
    } else {
        None
    }
}

#[proc_macro_attribute]
pub fn plc_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Some((dialect, description, namespace, version)) = parse_attribute_args(attr.into_iter())
    {
        // Use the parsed values here
        eprintln!(
            "Dialect: {:?}, Desc: {:?}, NS: {:?}, Ver: {:?}",
            dialect, description, namespace, version
        );
        let cloned_item = item.clone();
        let func = parse_macro_input!(cloned_item as ItemFn);
        let func_name = func.sig.ident.to_string();
        eprintln!("Function name: {}", func_name);
    }

    item
}
