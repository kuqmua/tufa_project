pub fn generate_postgresql_json_type_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    postgresql_json_type_ident_to_create_token_stream: &dyn quote::ToTokens,
    try_generate_postgresql_query_part_to_create_token_stream: &dyn quote::ToTokens,
    bind_value_to_postgresql_query_part_to_create_token_stream: &dyn quote::ToTokens,
    ident_postgresql_json_type_field_reader: &dyn quote::ToTokens,
    ident_postgresql_json_type_options_to_read: &dyn quote::ToTokens,
    generate_postgresql_query_part_to_read_token_stream: &dyn quote::ToTokens,
    ident_postgresql_json_type_option_to_update: &dyn quote::ToTokens,
    ident_postgresql_json_type_option_to_update_try_generate_postgresql_query_part_error_named: &dyn quote::ToTokens,
    try_generate_postgresql_query_part_to_update_token_stream: &dyn quote::ToTokens,
    bind_value_to_postgresql_query_part_to_update_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_json_type_to_create_upper_camel_case = naming_conventions::PostgresqlJsonTypeToCreateUpperCamelCase;
    let postgresql_json_type_to_create_snake_case = naming_conventions::PostgresqlJsonTypeToCreateSnakeCase;
    let postgresql_json_type_field_reader_upper_camel_case = naming_conventions::PostgresqlJsonTypeFieldReaderUpperCamelCase;
    let postgresql_json_type_field_reader_snake_case = naming_conventions::PostgresqlJsonTypeFieldReaderSnakeCase;
    let postgresql_json_type_options_to_read_upper_camel_case = naming_conventions::PostgresqlJsonTypeOptionsToReadUpperCamelCase;
    let postgresql_json_type_option_to_update_upper_camel_case = naming_conventions::PostgresqlJsonTypeOptionToUpdateUpperCamelCase;
    let postgresql_json_type_option_to_update_snake_case = naming_conventions::PostgresqlJsonTypeOptionToUpdateSnakeCase;
    let postgresql_json_type_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = naming_conventions::PostgresqlJsonTypeOptionToUpdateTryGeneratePostgresqlQueryPartErrorNamedUpperCamelCase;
    let increment_snake_case = naming_conventions::IncrementSnakeCase;
    let postgresql_json_type_upper_camel_case = naming_conventions::PostgresqlJsonTypeUpperCamelCase;
    let query_snake_case = naming_conventions::QuerySnakeCase;
    let field_ident_snake_case = naming_conventions::FieldIdentSnakeCase;
    let column_name_and_maybe_field_getter_snake_case = naming_conventions::ColumnNameAndMaybeFieldGetterSnakeCase;
    let column_name_and_maybe_field_getter_for_error_message_snake_case = naming_conventions::ColumnNameAndMaybeFieldGetterForErrorMessageSnakeCase;
    let jsonb_set_accumulator_snake_case = naming_conventions::JsonbSetAccumulatorSnakeCase;
    let jsonb_set_target_snake_case = naming_conventions::JsonbSetTargetSnakeCase;
    let jsonb_set_path_snake_case = naming_conventions::JsonbSetPathSnakeCase;
    let postgresql_json_type_try_generate_postgresql_query_part_to_create_error_named_upper_camel_case = naming_conventions::PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamedUpperCamelCase;
    let try_generate_postgresql_query_part_to_create_snake_case = naming_conventions::TryGeneratePostgresqlQueryPartToCreateSnakeCase;
    let bind_value_to_postgresql_query_part_to_create_snake_case = naming_conventions::BindValueToPostgresqlQueryPartToCreateSnakeCase;
    let generate_postgresql_query_part_to_read_snake_case = naming_conventions::GeneratePostgresqlQueryPartToReadSnakeCase;
    let try_generate_postgresql_query_part_to_update_snake_case = naming_conventions::TryGeneratePostgresqlQueryPartToUpdateSnakeCase;
    let bind_value_to_postgresql_query_part_to_update_snake_case = naming_conventions::BindValueToPostgresqlQueryPartToUpdateSnakeCase;
    let reference_std_primitive_str_token_stream = quote::quote!{&std::primitive::str};
    let reference_mut_std_primitive_u64_token_stream = quote::quote!{&mut std::primitive::u64};
    let mut_query_sqlx_query_postgres_arguments_token_stream = quote::quote!{mut #query_snake_case: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_postgres_arguments_token_stream = quote::quote!{sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let std_string_string_token_stream = token_patterns::StdStringString;
    //todo maybe reexport sqlx?
    quote::quote!{
        impl #path_token_stream #postgresql_json_type_upper_camel_case for #ident {
            type #postgresql_json_type_to_create_upper_camel_case<'a> = #postgresql_json_type_ident_to_create_token_stream;
            fn #try_generate_postgresql_query_part_to_create_snake_case(
                #postgresql_json_type_to_create_snake_case: &Self::#postgresql_json_type_to_create_upper_camel_case<'_>,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream
            ) -> Result<#std_string_string_token_stream, #path_token_stream #postgresql_json_type_try_generate_postgresql_query_part_to_create_error_named_upper_camel_case> {
                #try_generate_postgresql_query_part_to_create_token_stream
            }
            fn #bind_value_to_postgresql_query_part_to_create_snake_case<'a>(
                #postgresql_json_type_to_create_snake_case: Self::#postgresql_json_type_to_create_upper_camel_case<'a>,
                #mut_query_sqlx_query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #bind_value_to_postgresql_query_part_to_create_token_stream
            }
            type #postgresql_json_type_field_reader_upper_camel_case<'a> = #ident_postgresql_json_type_field_reader;
            type #postgresql_json_type_options_to_read_upper_camel_case<'a> = #ident_postgresql_json_type_options_to_read;
            fn #generate_postgresql_query_part_to_read_snake_case(
                #postgresql_json_type_field_reader_snake_case: &Self::#postgresql_json_type_field_reader_upper_camel_case<'_>,
                #field_ident_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_snake_case: #reference_std_primitive_str_token_stream,
                #column_name_and_maybe_field_getter_for_error_message_snake_case: #reference_std_primitive_str_token_stream
            ) -> #std_string_string_token_stream {
                #generate_postgresql_query_part_to_read_token_stream
            }
            type #postgresql_json_type_option_to_update_upper_camel_case<'a> = #ident_postgresql_json_type_option_to_update;
            type #postgresql_json_type_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case = #ident_postgresql_json_type_option_to_update_try_generate_postgresql_query_part_error_named;
            fn #try_generate_postgresql_query_part_to_update_snake_case(
                #postgresql_json_type_option_to_update_snake_case: &Self::#postgresql_json_type_option_to_update_upper_camel_case<'_>,
                #jsonb_set_accumulator_snake_case: #reference_std_primitive_str_token_stream,
                #jsonb_set_target_snake_case: #reference_std_primitive_str_token_stream,
                #jsonb_set_path_snake_case: #reference_std_primitive_str_token_stream,
                #increment_snake_case: #reference_mut_std_primitive_u64_token_stream,
            ) -> Result<#std_string_string_token_stream, Self::#postgresql_json_type_option_to_update_try_generate_postgresql_query_part_error_named_upper_camel_case> {
                #try_generate_postgresql_query_part_to_update_token_stream
            }
            fn #bind_value_to_postgresql_query_part_to_update_snake_case<'a>(
                #postgresql_json_type_option_to_update_snake_case: Self::#postgresql_json_type_option_to_update_upper_camel_case<'_>,
                #mut_query_sqlx_query_postgres_arguments_token_stream
            ) -> #query_postgres_arguments_token_stream {
                #bind_value_to_postgresql_query_part_to_update_token_stream
            }
        }
    }
}