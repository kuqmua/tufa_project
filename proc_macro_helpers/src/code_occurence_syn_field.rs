pub fn code_occurence_syn_field(
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> syn::Field {
    syn::Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(
            syn::Ident::new("code_occurence", proc_macro2::Span::call_site())
        ),
        colon_token: Some(
            syn::token::Colon {
                spans: [proc_macro2::Span::call_site()],
            },
        ),
        ty: syn::Type::Path(
            syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: crate::generate_simple_syn_punctuated_punctuated::generate_simple_syn_punctuated_punctuated(
                        &["error_occurence_lib","code_occurence","CodeOccurence"],
                        proc_macro_name_upper_camel_case_ident_stringified
                    ),
                },
            },
        ),
    }
}
