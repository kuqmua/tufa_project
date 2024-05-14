pub(crate) fn acquire_pool_and_connection(
    pg_connection_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    try_operation_route_logic_error_named_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    try_operation_route_logic_response_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    try_operation_route_logic_response_variants_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    postgresql_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let error_snake_case_token_stream = <naming_constants::Error as naming_constants::Naming>::snake_case_token_stream();
    quote::quote! {
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
            Ok(value) => value,
            Err(#error_snake_case_token_stream) => {
                let #error_snake_case_token_stream = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_initialization_token_stream;
                eprintln!("{error}");//todo reuse it as token_stream
                return #try_operation_route_logic_response_upper_camel_case_token_stream {
                    status_code: axum::http::StatusCode::CREATED,//todo
                    body: #try_operation_route_logic_response_variants_upper_camel_case_token_stream::from(#error_snake_case_token_stream),
                };
            }
        };
        let #pg_connection_token_stream = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(#error_snake_case_token_stream) => {
                let #error_snake_case_token_stream = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_initialization_token_stream;
                eprintln!("{error}");
                return #try_operation_route_logic_response_upper_camel_case_token_stream {
                    status_code: axum::http::StatusCode::CREATED,//todo
                    body: #try_operation_route_logic_response_variants_upper_camel_case_token_stream::from(#error_snake_case_token_stream),
                };
            }
        };
    }
}
