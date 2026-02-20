use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
fn const_space_ts(ts: &dyn ToTokens) -> Ts2 {
    quote! {const #ts}
}
fn pub_space_ts(ts: &dyn ToTokens) -> Ts2 {
    quote! {pub #ts}
}
fn impl_ident_ts(ident_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        impl #ident_ts {
            #ts
        }
    }
}
pub fn gen_new_ts(attr_ts: &dyn ToTokens, parameters_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        #attr_ts
        fn new(#parameters_ts) -> Self {
            #ts
        }
    }
}
pub fn gen_const_new_ts(
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts_5986cf7b = const_space_ts(&gen_new_ts(&Ts2::new(), parameters_ts, ts));
    quote! {
        #attr_ts
        #ts_5986cf7b
    }
}
pub fn gen_pub_new_ts(
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts_73940779 = pub_space_ts(&gen_new_ts(&Ts2::new(), parameters_ts, ts));
    quote! {
        #attr_ts
        #ts_73940779
    }
}
pub fn gen_pub_const_new_ts(
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts_5dc3668f = pub_space_ts(&gen_const_new_ts(&Ts2::new(), parameters_ts, ts));
    quote! {
        #attr_ts
        #ts_5dc3668f
    }
}
pub fn gen_impl_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_new_ts(attr_ts, parameters_ts, ts))
}
pub fn gen_impl_const_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_new_ts(attr_ts, parameters_ts, ts))
}
pub fn gen_impl_pub_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_pub_new_ts(attr_ts, parameters_ts, ts))
}
pub fn gen_impl_pub_const_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_pub_const_new_ts(attr_ts, parameters_ts, ts))
}
pub fn gen_try_new_ts(
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn try_new(#parameters_ts) -> Result<Self, #err_type_ts> {
            #ts
        }
    }
}
pub fn gen_const_try_new_ts(
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    const_space_ts(&gen_try_new_ts(parameters_ts, err_type_ts, ts))
}
pub fn gen_pub_try_new_ts(
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    pub_space_ts(&gen_try_new_ts(parameters_ts, err_type_ts, ts))
}
pub fn gen_pub_const_try_new_ts(
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    pub_space_ts(&gen_const_try_new_ts(parameters_ts, err_type_ts, ts))
}
pub fn gen_impl_try_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_try_new_ts(parameters_ts, err_type_ts, ts))
}
pub fn gen_impl_const_try_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(
        ident_ts,
        &gen_const_try_new_ts(parameters_ts, err_type_ts, ts),
    )
}
pub fn gen_impl_pub_try_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(
        ident_ts,
        &gen_pub_try_new_ts(parameters_ts, err_type_ts, ts),
    )
}
pub fn gen_impl_pub_const_try_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    parameters_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(
        ident_ts,
        &gen_pub_const_try_new_ts(parameters_ts, err_type_ts, ts),
    )
}
