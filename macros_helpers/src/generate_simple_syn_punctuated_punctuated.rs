#[must_use]
pub fn generate_simple_syn_punctuated_punctuated(
    parts_vec: &[&str],
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
    let parts_vec_len = parts_vec.len();
    if parts_vec_len >= 1 {
        let mut handle =
            syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
        for el_bef7ca16 in parts_vec.iter().rev().skip(1).rev() {
            handle.push_value(syn::PathSegment {
                ident: proc_macro2::Ident::new(el_bef7ca16, proc_macro2::Span::call_site()),
                arguments: syn::PathArguments::None,
            });
            handle.push_punct(syn::token::PathSep {
                spans: [
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ],
            });
        }
        if let Some(value) = parts_vec.last() {
            handle.push_value(syn::PathSegment {
                ident: proc_macro2::Ident::new(value, proc_macro2::Span::call_site()),
                arguments: syn::PathArguments::None,
            });
        }
        handle
    } else {
        panic!("f68497cc-bd58-497e-ae49-9ee22e07b45e");
    }
}

#[must_use]
pub fn std_string_string_syn_punctuated_punctuated()
-> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
    generate_simple_syn_punctuated_punctuated(&["std", "string", "String"])
}
