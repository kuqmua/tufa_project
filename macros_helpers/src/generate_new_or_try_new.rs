fn const_space_content_ts(content_ts: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {const #content_ts}
}
fn pub_space_content_ts(content_ts: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {pub #content_ts}
}
fn impl_ident_content_ts(
    ident_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #ident_ts {
            #content_ts
        }
    }
}

pub fn generate_new_ts(
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        #attribute_ts
        fn new(#parameters_ts) -> Self {
            #content_ts
        }
    }
}
pub fn generate_const_new_ts(
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let content_ts_5986cf7b = const_space_content_ts(&generate_new_ts(
        &proc_macro2::TokenStream::new(),
        parameters_ts,
        content_ts,
    ));
    quote::quote! {
        #attribute_ts
        #content_ts_5986cf7b
    }
}
pub fn generate_pub_new_ts(
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let content_ts_73940779 = pub_space_content_ts(&generate_new_ts(
        &proc_macro2::TokenStream::new(),
        parameters_ts,
        content_ts,
    ));
    quote::quote! {
        #attribute_ts
        #content_ts_73940779
    }
}
pub fn generate_pub_const_new_ts(
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let content_ts_5dc3668f = pub_space_content_ts(&generate_const_new_ts(
        &proc_macro2::TokenStream::new(),
        parameters_ts,
        content_ts,
    ));
    quote::quote! {
        #attribute_ts
        #content_ts_5dc3668f
    }
}
pub fn generate_impl_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_new_ts(attribute_ts, parameters_ts, content_ts),
    )
}
pub fn generate_impl_const_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_new_ts(attribute_ts, parameters_ts, content_ts),
    )
}
pub fn generate_impl_pub_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_pub_new_ts(attribute_ts, parameters_ts, content_ts),
    )
}
pub fn generate_impl_pub_const_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    attribute_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_pub_const_new_ts(attribute_ts, parameters_ts, content_ts),
    )
}

pub fn generate_try_new_ts(
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        fn try_new(#parameters_ts) -> Result<Self, #err_type_ts> {
            #content_ts
        }
    }
}
pub fn generate_const_try_new_ts(
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    const_space_content_ts(&generate_try_new_ts(parameters_ts, err_type_ts, content_ts))
}
pub fn generate_pub_try_new_ts(
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    pub_space_content_ts(&generate_try_new_ts(parameters_ts, err_type_ts, content_ts))
}
pub fn generate_pub_const_try_new_ts(
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    pub_space_content_ts(&generate_const_try_new_ts(
        parameters_ts,
        err_type_ts,
        content_ts,
    ))
}
pub fn generate_impl_try_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_try_new_ts(parameters_ts, err_type_ts, content_ts),
    )
}
pub fn generate_impl_const_try_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_const_try_new_ts(parameters_ts, err_type_ts, content_ts),
    )
}
pub fn generate_impl_pub_try_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_pub_try_new_ts(parameters_ts, err_type_ts, content_ts),
    )
}
pub fn generate_impl_pub_const_try_new_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    parameters_ts: &dyn quote::ToTokens,
    err_type_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    impl_ident_content_ts(
        ident_ts,
        &generate_pub_const_try_new_ts(parameters_ts, err_type_ts, content_ts),
    )
}
