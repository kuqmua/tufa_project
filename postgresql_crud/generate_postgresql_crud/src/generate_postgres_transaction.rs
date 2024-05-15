#[allow(clippy::too_many_arguments)]
pub(crate) fn generate_postgres_transaction(
    expected_updated_primary_keys_token_stream: &proc_macro2::TokenStream,
    query_string_name_token_stream: &proc_macro2::TokenStream,
    query_string_token_stream: &proc_macro2::TokenStream,
    binded_query_token_stream: &proc_macro2::TokenStream,
    acquire_pool_and_connection_token_stream: &proc_macro2::TokenStream,
    pg_connection_token_stream: &proc_macro2::TokenStream,
    binded_query_name_token_stream: &proc_macro2::TokenStream,
    use_futures_try_stream_ext_token_stream: &proc_macro2::TokenStream,
    primary_key_try_from_sqlx_row_name_token_stream: &proc_macro2::TokenStream,
    from_log_and_return_error_token_stream: &proc_macro2::TokenStream,
    rollback_error_name_token_stream: &proc_macro2::TokenStream,
    non_existing_primary_keys_name_token_stream: &proc_macro2::TokenStream,
    rollback_token_stream: &proc_macro2::TokenStream,
    response_variants_token_stream: &proc_macro2::TokenStream,
    desirable_token_stream: &proc_macro2::TokenStream,
    try_ident_upper_camel_case_token_stream: &proc_macro2::TokenStream,
    error_log_call_token_stream: &proc_macro2::TokenStream,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
    primary_key_syn_field_with_additional_info: &crate::SynFieldWithAdditionalInfo<'_>,
    query_and_rollback_failed_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
    primary_key_from_row_and_failed_rollback_syn_variant_initialization_token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let error_snake_case_token_stream = <naming_constants::Error as naming_constants::Naming>::snake_case_token_stream();
    let sqlx_acquire_token_stream = proc_macro_common::sqlx_acquire_token_stream();
    let begin_token_stream =
        <naming_constants::Begin as naming_constants::Naming>::snake_case_token_stream();
    let expected_updated_primary_keys_name_token_stream =
        quote::quote! {expected_updated_primary_keys};
    let primary_key_vec_name_token_stream = quote::quote! {primary_key_vec};
    let commit_token_stream =
        <naming_constants::Commit as naming_constants::Naming>::snake_case_token_stream();
    let postgres_transaction_token_stream = quote::quote! {postgres_transaction};
    let non_existing_primary_keys_variant_initialization_token_stream = {
        let field_code_occurence_new_4853d33a_b7e0_45df_8024_98ba66d26973_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            NonExistingPrimaryKeys {
                #non_existing_primary_keys_name_token_stream,
                #field_code_occurence_new_4853d33a_b7e0_45df_8024_98ba66d26973_token_stream,
            }
        }
    };
    let non_existing_primary_keys_and_failed_rollback_variant_initialization_token_stream = {
        let field_code_occurence_new_5e07939c_0aa6_4f48_9f1f_5d3866c651ab_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            NonExistingPrimaryKeysAndFailedRollback {
                #non_existing_primary_keys_name_token_stream,
                #rollback_error_name_token_stream: #error_snake_case_token_stream,
                #field_code_occurence_new_5e07939c_0aa6_4f48_9f1f_5d3866c651ab_token_stream,
            }
        }
    };
    let commit_failed_variant_initialization_token_stream = {
        let field_code_occurence_new_52fad21a_c2cd_40f2_85af_dfec05be9d22_token_stream = proc_macro_helpers::generate_field_code_occurence_new_token_stream::generate_field_code_occurence_new_token_stream(
            file!(),
            line!(),
            column!(),
            proc_macro_name_upper_camel_case_ident_stringified,
        );
        quote::quote! {
            CommitFailed {
                commit_failed: #error_snake_case_token_stream,
                #field_code_occurence_new_52fad21a_c2cd_40f2_85af_dfec05be9d22_token_stream,
            }
        }
    };
    let primary_key_inner_type_with_serialize_deserialize_token_stream = &primary_key_syn_field_with_additional_info.inner_type_with_serialize_deserialize_token_stream;
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
            use #sqlx_acquire_token_stream;
            #pg_connection_token_stream.#begin_token_stream()
        }
        .await
        {
            Ok(value) => value,
            Err(#error_snake_case_token_stream) => {
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
                        Err(#error_snake_case_token_stream) => {
                            option_error = Some(#error_snake_case_token_stream);
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
                        let #error_snake_case_token_stream = #try_ident_upper_camel_case_token_stream::#query_and_rollback_failed_syn_variant_initialization_token_stream;
                        #error_log_call_token_stream
                        return #response_variants_token_stream::from(#error_snake_case_token_stream);
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
                    Err(#error_snake_case_token_stream) => match #postgres_transaction_token_stream.#rollback_token_stream().await {
                        Ok(_) => {
                            #from_log_and_return_error_token_stream;
                        }
                        Err(#rollback_error_name_token_stream) => {
                            let #error_snake_case_token_stream = #try_ident_upper_camel_case_token_stream::#primary_key_from_row_and_failed_rollback_syn_variant_initialization_token_stream;
                            #error_log_call_token_stream
                            return #response_variants_token_stream::from(#error_snake_case_token_stream);
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
                        let #error_snake_case_token_stream = #try_ident_upper_camel_case_token_stream::#non_existing_primary_keys_variant_initialization_token_stream;
                        #error_log_call_token_stream
                        return #response_variants_token_stream::from(#error_snake_case_token_stream);
                    }
                    Err(#error_snake_case_token_stream) => {
                        let #error_snake_case_token_stream = #try_ident_upper_camel_case_token_stream::#non_existing_primary_keys_and_failed_rollback_variant_initialization_token_stream;
                        #error_log_call_token_stream
                        return #response_variants_token_stream::from(#error_snake_case_token_stream);
                    }
                }
            }
        }
        match #postgres_transaction_token_stream.#commit_token_stream().await {
            Ok(_) => #response_variants_token_stream::#desirable_token_stream(#primary_key_vec_name_token_stream.into_iter().map(
                |element|#primary_key_inner_type_with_serialize_deserialize_token_stream::from(element)
            ).collect()),
            Err(#error_snake_case_token_stream) => {
                let #error_snake_case_token_stream = #try_ident_upper_camel_case_token_stream::#commit_failed_variant_initialization_token_stream;
                #error_log_call_token_stream
                #response_variants_token_stream::from(#error_snake_case_token_stream)
            }
        }
    }
}
