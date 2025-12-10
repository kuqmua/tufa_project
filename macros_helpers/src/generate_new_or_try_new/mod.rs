mod generate_new;
pub use generate_new::{
    generate_new_token_stream,
    generate_const_new_token_stream,
    generate_pub_new_token_stream,
    generate_pub_const_new_token_stream,
    generate_impl_new_for_ident_token_stream,
    generate_impl_const_new_for_ident_token_stream,
    generate_impl_pub_new_for_ident_token_stream,
    generate_impl_pub_const_new_for_ident_token_stream
};
mod generate_try_new;
pub use generate_try_new::{
    generate_try_new_token_stream,
    generate_const_try_new_token_stream,
    generate_pub_try_new_token_stream,
    generate_pub_const_try_new_token_stream,
    generate_impl_try_new_for_ident_token_stream,
    generate_impl_const_try_new_for_ident_token_stream,
    generate_impl_pub_try_new_for_ident_token_stream,
    generate_impl_pub_const_try_new_for_ident_token_stream
};

fn const_space_content_token_stream(content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {const #content_token_stream}
}
fn pub_space_content_token_stream(content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {pub #content_token_stream}
}
fn impl_ident_content_token_stream(ident_token_stream: &dyn quote::ToTokens, content_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #ident_token_stream {
            #content_token_stream
        }
    }
}