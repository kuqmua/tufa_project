pub fn check_for_none(
    fields_named: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    id_field: &syn::Field,
    proc_macro_name_upper_camel_case_ident_stringified: &std::string::String,
    dot_space: &str,
    try_ident_response_variants_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    should_exclude_primary_key: bool
) -> proc_macro2::TokenStream {
    let (none_elements, match_elements) = {
        let fields_named_handle = match should_exclude_primary_key {
            true => fields_named.iter().filter(|field|*field != id_field).collect::<Vec<&syn::Field>>(),
            false => fields_named.iter().collect::<Vec<&syn::Field>>()
        };
        let fields_named_handle_len = fields_named_handle.len();
        fields_named_handle.iter().enumerate().fold(
            (
                std::string::String::default(),
                std::string::String::default()
            ), |mut acc, (index, field)| {
                let field_ident = field.ident.clone()
                    .unwrap_or_else(|| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                    });
                let possible_dot_space = match (index + 1) == fields_named_handle_len {
                    true => "",
                    false => dot_space,
                };
                acc.0.push_str(&format!("None{possible_dot_space}"));
                acc.1.push_str(&format!("&parameters.payload.{field_ident}{possible_dot_space}"));//todo parameters reuse naming
                acc
            })
    };
    let none_elements_token_stream = none_elements.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {none_elements} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let match_elements_token_stream = match_elements.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {match_elements} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
    let response_variant_token_stream = quote::quote!{
        NoPayloadFields {
            no_payload_fields: std::string::String::from("no payload fields"), 
            code_occurence: crate::code_occurence_tufa_common!()
        }
    };
    quote::quote!{
        if let (#none_elements_token_stream) = (#match_elements_token_stream) {
            return #try_ident_response_variants_upper_camel_case_token_stream::#response_variant_token_stream;
        }
    }
}