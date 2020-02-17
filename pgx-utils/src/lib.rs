use proc_macro2::TokenTree;
use std::collections::HashSet;
use syn::export::TokenStream2;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ExternArgs {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    Error(String),
}

pub fn parse_extern_attributes(attr: TokenStream2) -> HashSet<ExternArgs> {
    let mut args = HashSet::<ExternArgs>::new();
    let mut itr = attr.into_iter();
    while let Some(t) = itr.next() {
        match t {
            TokenTree::Group(g) => {
                for arg in parse_extern_attributes(g.stream()).into_iter() {
                    args.insert(arg);
                }
            }
            TokenTree::Ident(i) => {
                let name = i.to_string();
                match name.as_str() {
                    "immutable" => args.insert(ExternArgs::Immutable),
                    "strict" => args.insert(ExternArgs::Strict),
                    "stable" => args.insert(ExternArgs::Stable),
                    "volatile" => args.insert(ExternArgs::Volatile),
                    "raw" => args.insert(ExternArgs::Raw),
                    "no_guard" => args.insert(ExternArgs::NoGuard),
                    "parallel_safe" => args.insert(ExternArgs::ParallelSafe),
                    "parallel_unsafe" => args.insert(ExternArgs::ParallelUnsafe),
                    "parallel_restricted" => args.insert(ExternArgs::ParallelRestricted),
                    "error" => {
                        let _punc = itr.next().unwrap();
                        let literal = itr.next().unwrap();
                        let message = literal.to_string();
                        let message = unescape::unescape(&message).expect("failed to unescape");

                        // trim leading/trailing quotes around the literal
                        let message = message[1..message.len() - 1].to_string();
                        args.insert(ExternArgs::Error(message.to_string()))
                    }
                    _ => false,
                };
            }
            TokenTree::Punct(_) => {}
            TokenTree::Literal(_) => {}
        }
    }
    args
}

#[cfg(test)]
mod tests {
    use crate::{parse_extern_attributes, ExternArgs};
    use std::str::FromStr;
    use syn::export::TokenStream2;

    #[test]
    fn parse_args() {
        let s = "error = \"syntax error at or near \\\"THIS\\\"\"";
        let ts = TokenStream2::from_str(s).unwrap();

        let args = parse_extern_attributes(ts);
        assert!(args.contains(&ExternArgs::Error(
            "syntax error at or near \"THIS\"".to_string()
        )));
    }
}
