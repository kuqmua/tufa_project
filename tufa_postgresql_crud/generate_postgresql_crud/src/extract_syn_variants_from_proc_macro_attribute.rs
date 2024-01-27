pub fn extract_syn_variants_from_proc_macro_attribute(
    ast: &syn::DeriveInput,
    proc_macro_attribute_name: &str,
    proc_macro_name_snake_case: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &std::string::String
) -> std::vec::Vec::<(
        syn::Ident,
        proc_macro2::TokenStream,
        std::vec::Vec::<syn::Variant>
)> {
    let additional_http_status_codes_error_variant_path = format!("{proc_macro_name_snake_case}::{proc_macro_attribute_name}");
    let additional_http_status_codes_error_variants_attribute = proc_macro_helpers::get_macro_attribute::get_macro_attribute(
        &ast.attrs,
        &additional_http_status_codes_error_variant_path,
        proc_macro_name_upper_camel_case_ident_stringified
    );
    match additional_http_status_codes_error_variants_attribute.path.segments.len() == 2 {
        true => {
            let first_ident = &additional_http_status_codes_error_variants_attribute.path.segments.first().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} additional_http_status_codes_error_variants_attribute.path.segments.get(0) is None")
            }).ident;
            let second_ident = &additional_http_status_codes_error_variants_attribute.path.segments.last().unwrap_or_else(|| {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} additional_http_status_codes_error_variants_attribute.path.segments.get(0) is None")
            }).ident;
            let possible_additional_http_status_codes_error_variants_attribute_path = format!("{first_ident}::{second_ident}");
            if let false = additional_http_status_codes_error_variant_path == possible_additional_http_status_codes_error_variants_attribute_path {
                panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} {possible_additional_http_status_codes_error_variants_attribute_path} is not {additional_http_status_codes_error_variant_path}")
            }
        },
        false => panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} no {additional_http_status_codes_error_variant_path} path")
    }
    let mut additional_http_status_codes_error_variants_attribute_tokens_stringified = additional_http_status_codes_error_variants_attribute.tokens.to_string();
    let additional_http_status_codes_error_variants_attribute_tokens_stringified_len = additional_http_status_codes_error_variants_attribute_tokens_stringified.len();
    let additional_http_status_codes_error_variants_attribute_tokens_without_brackets_stringified = &additional_http_status_codes_error_variants_attribute_tokens_stringified[1..(additional_http_status_codes_error_variants_attribute_tokens_stringified_len - 1)];//todo maybe check
    additional_http_status_codes_error_variants_attribute_tokens_without_brackets_stringified.split(";").collect::<Vec<&str>>()
        .iter().fold(std::vec::Vec::<(
            syn::Ident,
            proc_macro2::TokenStream,
            std::vec::Vec::<syn::Variant>
        )>::new(), |mut acc, element| {
            let element_token_stream: proc_macro::TokenStream = element.parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} {element} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
                .into();
            let element_token_stream_stringified = element_token_stream.to_string();
            match element_token_stream_stringified.is_empty() {
                true => acc,
                false => {
                    let element_derive_input: syn::DeriveInput = syn::parse(element_token_stream).unwrap_or_else(|e| {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} parse additional_http_status_codes_error_variants_attribute_tokens failed {e}");
                    });
                    let option_path_sttribute = element_derive_input.attrs.iter().find(|element|{
                        let element_path = &element.path;
                        let element_path_token_stream = quote::quote!{#element_path};
                        let element_path_stringified = element_path_token_stream.to_string();
                        element_path_stringified == "path"
                    });
                    let path_attribute = match option_path_sttribute {
                        Some(value) => value,
                        None => panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} no path attribute"),
                    };
                    let path_to_additional_variant_enum_stringified = &path_attribute.tokens.to_string();
                    let path_to_additional_variant_enum_without_brackets_stringified = &path_to_additional_variant_enum_stringified[1..(path_to_additional_variant_enum_stringified.len()- 1)];//todo maybe check
                    let path_to_additional_variant_enum_without_brackets_token_stream = path_to_additional_variant_enum_without_brackets_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {path_to_additional_variant_enum_without_brackets_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE));
                    let element_ident = element_derive_input.ident;//todo check if error type even exists (with empty functions)
                    let data_enum = if let syn::Data::Enum(data_enum) = element_derive_input.data {
                        data_enum
                    } else {
                        panic!("{proc_macro_name_upper_camel_case_ident_stringified} {additional_http_status_codes_error_variant_path} does not work on enums!");
                    };
                    acc.push((
                        element_ident,
                        path_to_additional_variant_enum_without_brackets_token_stream,
                        data_enum.variants.into_iter().collect::<std::vec::Vec<syn::Variant>>()
                    ));
                    acc
                }
            }
        })
}

pub fn extract_syn_variants_from_method_proc_macro_attribute(
    ast: &syn::DeriveInput,
    operation_name_snake_case_stringified: &str,
    additional_http_status_codes_error_variants_snake_case_stringified: &str,
    proc_macro_name_snake_case: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &std::string::String
) -> std::vec::Vec::<(
    syn::Ident,
    proc_macro2::TokenStream,
    std::vec::Vec::<syn::Variant>
)> {
    crate::extract_syn_variants_from_proc_macro_attribute::extract_syn_variants_from_proc_macro_attribute(
        &ast,
        &format!("{operation_name_snake_case_stringified}_{additional_http_status_codes_error_variants_snake_case_stringified}"),
        &proc_macro_name_snake_case,
        &proc_macro_name_upper_camel_case_ident_stringified
    )
}