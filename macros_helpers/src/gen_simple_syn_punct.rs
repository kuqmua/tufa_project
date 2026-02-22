use syn::{PathArguments, PathSegment, punctuated::Punctuated, token::PathSep};
#[must_use]
pub fn gen_simple_syn_punct(v: &[&str]) -> Punctuated<PathSegment, PathSep> {
    let v_len = v.len();
    if v_len >= 1 {
        let mut acc = Punctuated::<PathSegment, PathSep>::new();
        for el in v.iter().rev().skip(1).rev() {
            acc.push_value(PathSegment {
                ident: proc_macro2::Ident::new(el, proc_macro2::Span::call_site()),
                arguments: PathArguments::None,
            });
            acc.push_punct(PathSep {
                spans: [
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ],
            });
        }
        if let Some(v0) = v.last() {
            acc.push_value(PathSegment {
                ident: proc_macro2::Ident::new(v0, proc_macro2::Span::call_site()),
                arguments: PathArguments::None,
            });
        }
        acc
    } else {
        panic!("f68497cc");
    }
}
#[must_use]
pub fn string_syn_punct() -> Punctuated<PathSegment, PathSep> {
    gen_simple_syn_punct(&["std", "string", "String"])
}
