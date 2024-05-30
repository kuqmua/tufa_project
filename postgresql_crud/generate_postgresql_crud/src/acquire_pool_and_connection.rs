pub(crate) fn acquire_pool_and_connection(
    pg_connection_token_stream: &naming_conventions::PgConnectionSnakeCase,
    // proc_macro_name_upper_camel_case_ident_stringified: &str,
    try_operation_route_logic_error_named_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    try_operation_route_logic_response_variants_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    postgresql_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    eprintln_error_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    // let error_snake_case_token_stream = <naming_constants::Error as naming_constants::Naming>::snake_case_token_stream();
    let value_snake_case = naming_constants::ValueSnakeCase;
    let error_snake_case = naming_constants::ErrorSnakeCase;
    let from_snake_case = naming_constants::FromSnakeCase;
    let into_response_snake_case = naming_conventions::IntoResponseSnakeCase;
    quote::quote! {
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
            Ok(#value_snake_case) => #value_snake_case,
            Err(#error_snake_case) => {
                let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_initialization_token_stream;
                #eprintln_error_token_stream;//todo reuse it as token_stream

                let mut res = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                *res.status_mut() = axum::http::StatusCode::CREATED;
                // *res.headers_mut() = axum::http::HeaderMap::new();
                return res;
            }
        };
        let #pg_connection_token_stream = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(#value_snake_case) => #value_snake_case,
            Err(#error_snake_case) => {
                let #error_snake_case = #try_operation_route_logic_error_named_upper_camel_case_token_stream::#postgresql_syn_variant_initialization_token_stream;
                #eprintln_error_token_stream;

                let mut res = axum::response::IntoResponse::#into_response_snake_case(axum::Json(#try_operation_route_logic_response_variants_upper_camel_case_token_stream::#from_snake_case(#error_snake_case)));
                *res.status_mut() = axum::http::StatusCode::CREATED;
                // *res.headers_mut() = axum::http::HeaderMap::new();
                return res;
            }
        };
    }
}
