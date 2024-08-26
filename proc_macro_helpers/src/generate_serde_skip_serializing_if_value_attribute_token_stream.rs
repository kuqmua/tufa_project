pub fn generate_serde_skip_serializing_if_value_attribute_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{#[serde(skip_serializing_if = "Option::is_none")]}
}