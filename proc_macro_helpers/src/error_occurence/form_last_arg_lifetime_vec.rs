pub fn form_last_arg_lifetime_vec(
    segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
    proc_macro_name_ident_stringified: &str,
) -> Vec<crate::error_occurence::lifetime::Lifetime> {
    if let Some(path_segment) = segments.last() {
        match &path_segment.arguments {
            syn::PathArguments::None => Vec::new(),
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => {
                angle_bracketed_generic_argument.args.iter().map(|generic_argument|{
                    match generic_argument {
                        syn::GenericArgument::Lifetime(lfmt) => crate::error_occurence::lifetime::Lifetime::Specified(lfmt.ident.to_string()),
                        syn::GenericArgument::Type(_) => crate::error_occurence::lifetime::Lifetime::NotSpecified,
                        _ => panic!(
                            "{proc_macro_name_ident_stringified} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] {} syn::GenericArgument::Lifetime and {}",
                            naming_constants::SUPPORTS_ONLY_STRINGIFIED,
                            naming_constants::SYN_GENERIC_ARGUMENT_TYPE_STRINGIFIED
                        )
                    }
                })
                .collect()
            },
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        }
    } else {
        panic!(
            "{proc_macro_name_ident_stringified} type_path.path.segments.last() {}",
            naming_constants::IS_NONE_STRINGIFIED
        );
    }
}
