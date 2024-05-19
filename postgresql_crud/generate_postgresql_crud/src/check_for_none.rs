pub(crate) fn check_for_none(
    fields_named: &[&syn::Field],//&syn::punctuated::Punctuated<syn::Field, syn::token::Comma>
    id_field: &syn::Field,
    proc_macro_name_upper_camel_case_ident_stringified: &std::string::String,
    dot_space: &str,
    try_ident_response_variants_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    should_exclude_primary_key: bool,
    no_payload_fields_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let (none_elements, match_elements) = {
        let fields_named_handle = if should_exclude_primary_key {
            fields_named
                .iter()
                .filter(|field| **field != id_field)
                .collect::<Vec<&&syn::Field>>()
        }
        else {
            fields_named.iter().collect::<Vec<&&syn::Field>>()
        };
        let fields_named_handle_len = fields_named_handle.len();
        fields_named_handle.iter().enumerate().fold(
            (
                std::string::String::default(),
                std::string::String::default(),
            ),
            |mut acc, (index, field)| {
                let field_ident = field.ident.as_ref().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} {}",
                        naming_constants::FIELD_IDENT_IS_NONE
                    )
                });
                let possible_dot_space = if index.checked_add(1).unwrap() == fields_named_handle_len {
                    ""
                }
                else {
                    dot_space
                };
                acc.0.push_str(&format!("None{possible_dot_space}"));
                acc.1.push_str(&format!(
                    "&parameters.payload.{field_ident}{possible_dot_space}"
                )); //todo parameters reuse naming
                acc
            },
        )
    };
    let none_elements_token_stream = none_elements.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {none_elements} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let match_elements_token_stream = match_elements.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {match_elements} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    quote::quote! {
        if let (#none_elements_token_stream) = (#match_elements_token_stream) {
            return #try_ident_response_variants_upper_camel_case_token_stream::#no_payload_fields_syn_variant_initialization_token_stream;
        }
    }
}
