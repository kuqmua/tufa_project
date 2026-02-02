fn const_space_content_token_stream(
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {const #content_token_stream}
}
fn pub_space_content_token_stream(
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {pub #content_token_stream}
}
fn impl_ident_content_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #ident_token_stream {
            #content_token_stream
        }
    }
}

pub fn generate_new_token_stream(
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        #attribute_token_stream
        fn new(#parameters_token_stream) -> Self {
            #content_token_stream
        }
    }
}
pub fn generate_const_new_token_stream(
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let content_token_stream_5986cf7b =
        const_space_content_token_stream(&generate_new_token_stream(
            &proc_macro2::TokenStream::new(),
            parameters_token_stream,
            content_token_stream,
        ));
    quote::quote! {
        #attribute_token_stream
        #content_token_stream_5986cf7b
    }
}
pub fn generate_pub_new_token_stream(
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let content_token_stream_73940779 = pub_space_content_token_stream(&generate_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        content_token_stream,
    ));
    quote::quote! {
        #attribute_token_stream
        #content_token_stream_73940779
    }
}
pub fn generate_pub_const_new_token_stream(
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let content_token_stream_5dc3668f =
        pub_space_content_token_stream(&generate_const_new_token_stream(
            &proc_macro2::TokenStream::new(),
            parameters_token_stream,
            content_token_stream,
        ));
    quote::quote! {
        #attribute_token_stream
        #content_token_stream_5dc3668f
    }
}
pub fn generate_impl_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_new_token_stream(
            attribute_token_stream,
            parameters_token_stream,
            content_token_stream,
        ),
    )
}
pub fn generate_impl_const_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_new_token_stream(
            attribute_token_stream,
            parameters_token_stream,
            content_token_stream,
        ),
    )
}
pub fn generate_impl_pub_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_pub_new_token_stream(
            attribute_token_stream,
            parameters_token_stream,
            content_token_stream,
        ),
    )
}
pub fn generate_impl_pub_const_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    attribute_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_pub_const_new_token_stream(
            attribute_token_stream,
            parameters_token_stream,
            content_token_stream,
        ),
    )
}

pub fn generate_try_new_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        fn try_new(#parameters_token_stream) -> Result<Self, #err_type_token_stream> {
            #content_token_stream
        }
    }
}
pub fn generate_const_try_new_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    const_space_content_token_stream(&generate_try_new_token_stream(
        parameters_token_stream,
        err_type_token_stream,
        content_token_stream,
    ))
}
pub fn generate_pub_try_new_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    pub_space_content_token_stream(&generate_try_new_token_stream(
        parameters_token_stream,
        err_type_token_stream,
        content_token_stream,
    ))
}
pub fn generate_pub_const_try_new_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    pub_space_content_token_stream(&generate_const_try_new_token_stream(
        parameters_token_stream,
        err_type_token_stream,
        content_token_stream,
    ))
}
pub fn generate_impl_try_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_try_new_token_stream(
            parameters_token_stream,
            err_type_token_stream,
            content_token_stream,
        ),
    )
}
pub fn generate_impl_const_try_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_const_try_new_token_stream(
            parameters_token_stream,
            err_type_token_stream,
            content_token_stream,
        ),
    )
}
pub fn generate_impl_pub_try_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_pub_try_new_token_stream(
            parameters_token_stream,
            err_type_token_stream,
            content_token_stream,
        ),
    )
}
pub fn generate_impl_pub_const_try_new_for_ident_token_stream(
    ident_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    content_token_stream: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_token_stream(
        ident_token_stream,
        &generate_pub_const_try_new_token_stream(
            parameters_token_stream,
            err_type_token_stream,
            content_token_stream,
        ),
    )
}
