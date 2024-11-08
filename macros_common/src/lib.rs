pub mod constants;
pub mod generate_quotes;
pub mod naming_conventions;
pub mod panic_location;

pub fn eprintln_error_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {eprintln!("{error}");}
}
