pub fn gen_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{
        // impl crate::generate_postgresql_query_part::PostgresqlJsonType for #ident {
        //     #to_create_token_stream
        //     #try_generate_postgresql_query_part_to_create_token_stream
        //     #bind_value_to_postgresql_query_part_to_create_token_stream
        //     #field_reader_token_stream
        //     #options_to_read_token_stream
        //     #generate_postgresql_query_part_to_read_token_stream
        //     #option_to_update_token_stream
        //     #option_to_update_try_generate_postgresql_query_part_error_named_token_stream
        //     #try_generate_postgresql_query_part_to_update_token_stream
        //     #bind_value_to_postgresql_query_part_to_update_token_stream
        // }
    }
}