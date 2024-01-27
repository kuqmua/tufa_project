#[allow(clippy::too_many_arguments)]
pub fn generate_postgres_transaction(
    expected_updated_primary_keys_token_stream: &proc_macro2::TokenStream,
    query_string_name_token_stream: &proc_macro2::TokenStream,
    query_string_token_stream: &proc_macro2::TokenStream,
    binded_query_token_stream: &proc_macro2::TokenStream,
    acquire_pool_and_connection_token_stream: &proc_macro2::TokenStream,
    use_sqlx_acquire_token_stream: &proc_macro2::TokenStream,
    pg_connection_token_stream: &proc_macro2::TokenStream,
    begin_token_stream: &proc_macro2::TokenStream,
    binded_query_name_token_stream: &proc_macro2::TokenStream,
    use_futures_try_stream_ext_token_stream: &proc_macro2::TokenStream,
    query_and_rollback_failed_token_stream: &proc_macro2::TokenStream,
    primary_key_try_from_sqlx_row_name_token_stream: &proc_macro2::TokenStream,
    from_log_and_return_error_token_stream: &proc_macro2::TokenStream,
    rollback_error_name_token_stream: &proc_macro2::TokenStream,
    primary_key_from_row_and_failed_rollback_token_stream: &proc_macro2::TokenStream,
    non_existing_primary_keys_name_token_stream: &proc_macro2::TokenStream,
    expected_updated_primary_keys_name_token_stream: &proc_macro2::TokenStream,
    primary_key_vec_name_token_stream: &proc_macro2::TokenStream,
    rollback_token_stream: &proc_macro2::TokenStream,
    non_existing_primary_keys_token_stream: &proc_macro2::TokenStream,
    non_existing_primary_keys_and_failed_rollback_token_stream: &proc_macro2::TokenStream,
    postgres_transaction_token_stream: &proc_macro2::TokenStream,
    commit_token_stream: &proc_macro2::TokenStream,
    response_variants_token_stream: &proc_macro2::TokenStream,
    desirable_token_stream: &proc_macro2::TokenStream,
    try_ident_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    commit_failed_token_stream: &proc_macro2::TokenStream,
    error_log_call_token_stream: &proc_macro2::TokenStream,
    crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        let #expected_updated_primary_keys_name_token_stream = {
            #expected_updated_primary_keys_token_stream
        };
        let #binded_query_name_token_stream = {
            let #query_string_name_token_stream = {
                #query_string_token_stream
            };
            println!("{}", #query_string_name_token_stream);
            #binded_query_token_stream
        };
        #acquire_pool_and_connection_token_stream
        let mut #postgres_transaction_token_stream = match {
            #use_sqlx_acquire_token_stream;
            #pg_connection_token_stream.#begin_token_stream()
        }
        .await
        {
            Ok(value) => value,
            Err(e) => {
                #from_log_and_return_error_token_stream
            }
        };
        let results_vec = {
            let mut results_vec = std::vec::Vec::with_capacity(#expected_updated_primary_keys_name_token_stream.len());
            let mut option_error: Option<sqlx::Error> = None;
            {
                let mut rows = #binded_query_name_token_stream.fetch(#postgres_transaction_token_stream.as_mut());
                while let (Some(Some(row)), None) = (
                    match {
                        #use_futures_try_stream_ext_token_stream;
                        rows.try_next()
                    }
                    .await
                    {
                        Ok(value) => Some(value),
                        Err(e) => {
                            option_error = Some(e);
                            None
                        }
                    },
                    &option_error,
                ) {
                    results_vec.push(row);
                }
            }
            if let Some(e) = option_error {
                match #postgres_transaction_token_stream.#rollback_token_stream().await {
                    Ok(_) => {
                        #from_log_and_return_error_token_stream;
                    }
                    Err(#rollback_error_name_token_stream) => {
                        //todo  BIG QUESTION - WHAT TO DO IF ROLLBACK FAILED? INFINITE LOOP TRYING TO ROLLBACK?
                        let error = #try_ident_upper_camel_case_token_stream::#query_and_rollback_failed_token_stream;
                        #error_log_call_token_stream
                        return #response_variants_token_stream::from(error);
                    }
                }
            }
            results_vec
        };
        let #primary_key_vec_name_token_stream = {
            let mut #primary_key_vec_name_token_stream = std::vec::Vec::with_capacity(#expected_updated_primary_keys_name_token_stream.len());
            for element in results_vec {
                match #primary_key_try_from_sqlx_row_name_token_stream(&element) {
                    Ok(primary_key) => {
                        #primary_key_vec_name_token_stream.push(primary_key);
                    }
                    Err(e) => match #postgres_transaction_token_stream.#rollback_token_stream().await {
                        Ok(_) => {
                            #from_log_and_return_error_token_stream;
                        }
                        Err(#rollback_error_name_token_stream) => {
                            let error = #try_ident_upper_camel_case_token_stream::#primary_key_from_row_and_failed_rollback_token_stream;
                            #error_log_call_token_stream
                            return #response_variants_token_stream::from(error);
                        }
                    },
                }
            }
            #primary_key_vec_name_token_stream
        };
        {
            let #non_existing_primary_keys_name_token_stream = {
                let len = #expected_updated_primary_keys_name_token_stream.len();
                #expected_updated_primary_keys_name_token_stream.into_iter().fold(std::vec::Vec::with_capacity(len), |mut acc, element| {
                    if let false = #primary_key_vec_name_token_stream.contains(&element) {
                        acc.push(element);
                    }
                    acc
                })
            };
            if let false = #non_existing_primary_keys_name_token_stream.is_empty() {
                match #postgres_transaction_token_stream.#rollback_token_stream().await {
                    Ok(_) => {
                        let error = #try_ident_upper_camel_case_token_stream::#non_existing_primary_keys_token_stream;
                        #error_log_call_token_stream
                        return #response_variants_token_stream::from(error);
                    }
                    Err(e) => {
                        let error = #try_ident_upper_camel_case_token_stream::#non_existing_primary_keys_and_failed_rollback_token_stream;
                        #error_log_call_token_stream
                        return #response_variants_token_stream::from(error);
                    }
                }
            }
        }
        match #postgres_transaction_token_stream.#commit_token_stream().await {
            Ok(_) => #response_variants_token_stream::#desirable_token_stream(#primary_key_vec_name_token_stream.into_iter().map(
                |element|#crate_server_postgres_uuid_wrapper_possible_uuid_wrapper_token_stream::from(element)
            ).collect()),
            Err(e) => {
                let error = #try_ident_upper_camel_case_token_stream::#commit_failed_token_stream;
                #error_log_call_token_stream
                #response_variants_token_stream::from(error)
            }
        }
    }
}
