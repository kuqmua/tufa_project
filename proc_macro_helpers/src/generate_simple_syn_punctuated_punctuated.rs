pub fn generate_simple_syn_punctuated_punctuated(
    parts_vec: &[&str],
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
    let parts_vec_len = parts_vec.len();
    match parts_vec_len >= 1 {
        true => {
            let mut handle = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
            for element in parts_vec.iter().rev().skip(1).rev() {
                handle.push_value(
                    syn::PathSegment {
                        ident: proc_macro2::Ident::new(element, proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    }
                );
                handle.push_punct(syn::token::Colon2{
                    spans: [proc_macro2::Span::call_site(),proc_macro2::Span::call_site()],
                });
            }
            if let Some(value) = parts_vec.last() {
                handle.push_value(
                    syn::PathSegment {
                        ident: proc_macro2::Ident::new(value, proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    }
                );
            }
            handle
        },
        false => panic!("{proc_macro_name_upper_camel_case_ident_stringified} generate_simple_syn_punctuated_punctuated parts_vec_len.len() > 1 == false for {parts_vec:?}")
    }
}

pub fn std_string_string_syn_punctuated_punctuated(
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> {
    crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
        &["std", "string", "String"],
        proc_macro_name_upper_camel_case_ident_stringified,
    )
}
