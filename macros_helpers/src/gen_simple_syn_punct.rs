use syn::{PathArguments, PathSegment, punctuated::Punctuated, token::PathSep};
#[must_use]
pub fn gen_simple_syn_punct(parts_vec: &[&str]) -> Punctuated<PathSegment, PathSep> {
    let parts_vec_len = parts_vec.len();
    if parts_vec_len >= 1 {
        let mut handle = Punctuated::<PathSegment, PathSep>::new();
        for el_bef7ca16 in parts_vec.iter().rev().skip(1).rev() {
            handle.push_value(PathSegment {
                ident: proc_macro2::Ident::new(el_bef7ca16, proc_macro2::Span::call_site()),
                arguments: PathArguments::None,
            });
            handle.push_punct(PathSep {
                spans: [
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ],
            });
        }
        if let Some(value) = parts_vec.last() {
            handle.push_value(PathSegment {
                ident: proc_macro2::Ident::new(value, proc_macro2::Span::call_site()),
                arguments: PathArguments::None,
            });
        }
        handle
    } else {
        panic!("f68497cc");
    }
}
#[must_use]
pub fn string_syn_punct() -> Punctuated<PathSegment, PathSep> {
    gen_simple_syn_punct(&["std", "string", "String"])
}
