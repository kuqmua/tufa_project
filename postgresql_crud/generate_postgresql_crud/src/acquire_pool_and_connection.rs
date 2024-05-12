pub(crate) fn acquire_pool_and_connection(
    from_log_and_return_error_token_stream: &proc_macro2::TokenStream,
    pg_connection_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let error_value_snake_case_token_stream = proc_macro_common::error_value_snake_case_token_stream();
    quote::quote! {
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {//todo find out difference between acquire and try_acquire
            Ok(value) => value,
            Err(error) => {
                #from_log_and_return_error_token_stream
            }
        };
        let #pg_connection_token_stream = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error) => {
                #from_log_and_return_error_token_stream
            }
        };
    }
}
