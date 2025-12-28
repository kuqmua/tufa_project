pub fn generate_field_code_occurence_new_token_stream(
    file: &'static str,
    line: u32,
    column: u32,
) -> proc_macro2::TokenStream {
    let code_occurence_new_token_stream = {
        let file_token_stream = generate_quotes::double_quotes_token_stream(&file);
        let line_token_stream = {
            let line_stringified = line.to_string();
            line_stringified
                .parse::<proc_macro2::TokenStream>()
                .expect("1d6812d7-2296-4d38-b3ea-bff1e625eaf5")
        };
        let column_token_stream = {
            let column_stringified = column.to_string();
            column_stringified
                .parse::<proc_macro2::TokenStream>()
                .expect("105a4e62-7574-4b1e-bd5f-eed440d72e89")
        };
        quote::quote! {
            error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from(#file_token_stream),
                    line: #line_token_stream,
                    column: #column_token_stream,
                })
            )
        }
    };
    let code_occurence_snake_case_token_stream = naming::CodeOccurenceSnakeCase;
    quote::quote! {
        #code_occurence_snake_case_token_stream: #code_occurence_new_token_stream
    }
}
