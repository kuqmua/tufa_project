#[proc_macro]
pub fn generate_struct_or_enum_derive_token_stream_builder(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generated = quote::quote! {};
    // macros_helpers::write_token_stream_into_file(
    //     "GenerateStructOrEnumDeriveTokenStreamBuilder",
    //     &generated,
    //     &macros_helpers::FormatWithRustfmt::True
    // );
    generated.into()
}
