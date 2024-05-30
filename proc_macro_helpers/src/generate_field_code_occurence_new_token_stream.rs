pub fn generate_field_code_occurence_new_token_stream(
    file: &'static str,
    line: std::primitive::u32,
    column: std::primitive::u32,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let code_occurence_new_token_stream = {
        let file_token_stream = proc_macro_common::generate_quotes::token_stream(
            file,
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        let line_token_stream = {
            let line_stringified = line.to_string();
            line_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {line_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        let column_token_stream = {
            let column_stringified = column.to_string();
            column_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {column_stringified} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
        };
        quote::quote! {
            error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from(#file_token_stream),
                    line: #line_token_stream,
                    column: #column_token_stream,
                })
            )
        }
    };
    let code_occurence_snake_case_token_stream = naming_conventions::CodeOccurenceSnakeCase;
    quote::quote! {
        #code_occurence_snake_case_token_stream: #code_occurence_new_token_stream
    }
}
