pub fn generate_postgresql_json_type_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    ident_to_create_token_stream: &dyn quote::ToTokens,
    try_generate_postgresql_query_part_to_create_token_stream: &dyn quote::ToTokens,
    bind_value_to_postgresql_query_part_to_create_token_stream: &dyn quote::ToTokens,
    ident_field_reader: &dyn quote::ToTokens,
    ident_options_to_read: &dyn quote::ToTokens,
    generate_postgresql_query_part_to_read_token_stream: &dyn quote::ToTokens,
    ident_option_to_update: &dyn quote::ToTokens,
    ident_option_to_update_try_generate_postgresql_query_part_error_named: &dyn quote::ToTokens,
    try_generate_postgresql_query_part_to_update_token_stream: &dyn quote::ToTokens,
    bind_value_to_postgresql_query_part_to_update_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    //todo maybe reexport sqlx?
    quote::quote!{
        impl #path_token_stream PostgresqlJsonType for #ident {
            type ToCreate<'a> = #ident_to_create_token_stream;
            fn try_generate_postgresql_query_part_to_create(
                to_create: &Self::ToCreate<'_>,
                increment: &mut std::primitive::u64
            ) -> Result<std::string::String, #path_token_stream PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed> {
                #try_generate_postgresql_query_part_to_create_token_stream
            }
            fn bind_value_to_postgresql_query_part_to_create<'a>(
                to_create: Self::ToCreate<'a>,
                mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #bind_value_to_postgresql_query_part_to_create_token_stream
            }
            type FieldReader<'a> = #ident_field_reader;
            type OptionsToRead<'a> = #ident_options_to_read;
            fn generate_postgresql_query_part_to_read(
                field_reader: &Self::FieldReader<'_>,
                field_ident: &std::primitive::str,
                column_name_and_maybe_field_getter: &std::primitive::str,
                column_name_and_maybe_field_getter_for_error_message: &std::primitive::str
            ) -> std::string::String {
                #generate_postgresql_query_part_to_read_token_stream
            }
            type OptionToUpdate<'a> = #ident_option_to_update;
            type OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed = #ident_option_to_update_try_generate_postgresql_query_part_error_named;
            fn try_generate_postgresql_query_part_to_update(
                option_to_update: &Self::OptionToUpdate<'_>,
                jsonb_set_accumulator: &std::primitive::str,
                jsonb_set_target: &std::primitive::str,
                jsonb_set_path: &std::primitive::str,
                increment: &mut std::primitive::u64,
            ) -> Result<std::string::String, Self::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed> {
                #try_generate_postgresql_query_part_to_update_token_stream
            }
            fn bind_value_to_postgresql_query_part_to_update<'a>(
                option_to_update: Self::OptionToUpdate<'_>,
                mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
                #bind_value_to_postgresql_query_part_to_update_token_stream
            }
        }
    }
}