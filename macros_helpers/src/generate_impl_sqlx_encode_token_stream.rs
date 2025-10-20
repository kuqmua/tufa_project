pub fn generate_impl_sqlx_encode_token_stream(
    for_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #for_type_token_stream {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#content_token_stream, buf)
            }
        }
    }
}
