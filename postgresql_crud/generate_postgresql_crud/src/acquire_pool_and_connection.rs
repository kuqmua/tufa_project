pub(crate) fn acquire_pool_and_connection(
    pg_connection_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    try_operation_route_logic_error_named_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    try_operation_route_logic_response_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    try_operation_route_logic_response_variants_upper_camel_case_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let error_value_snake_case_token_stream = proc_macro_common::error_value_snake_case_token_stream();
    let field_code_occurence_new_05d9ba85_7a76_412e_8243_8bdd48e9ffac_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    let field_code_occurence_new_2a54f69c_8962_4acf_9c7f_807aa2ac61f3_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
        file!(),
        line!(),
        column!(),
        &proc_macro_name_upper_camel_case_ident_stringified,
    );
    quote::quote! {
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
            Ok(value) => value,
            Err(error) => {
                let error = #try_operation_route_logic_error_named_upper_camel_case_token_stream::Postgresql {
                    postgresql: error,
                    #field_code_occurence_new_05d9ba85_7a76_412e_8243_8bdd48e9ffac_token_stream
                };
                eprintln!("{error}");
                return #try_operation_route_logic_response_upper_camel_case_token_stream {
                    status_code: axum::http::StatusCode::CREATED,//todo
                    body: #try_operation_route_logic_response_variants_upper_camel_case_token_stream::from(error),
                };
            }
        };
        let #pg_connection_token_stream = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error) => {
                let error = #try_operation_route_logic_error_named_upper_camel_case_token_stream::Postgresql {
                    postgresql: error,
                    #field_code_occurence_new_2a54f69c_8962_4acf_9c7f_807aa2ac61f3_token_stream
                };
                eprintln!("{error}");
                return #try_operation_route_logic_response_upper_camel_case_token_stream {
                    status_code: axum::http::StatusCode::CREATED,//todo
                    body: #try_operation_route_logic_response_variants_upper_camel_case_token_stream::from(error),
                };
            }
        };
    }
}
