use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
fn with_attr_ts(attr_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        #attr_ts
        #ts
    }
}
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
pub fn gen_new_ts(attr_ts: &dyn ToTokens, prms_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        #attr_ts
        fn new(#prms_ts) -> Self {
            #ts
        }
    }
}
pub fn gen_const_new_ts(attr_ts: &dyn ToTokens, prms_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    let ts_5986cf7b = const_space_ts(&gen_new_ts(&Ts2::new(), prms_ts, ts));
    with_attr_ts(attr_ts, &ts_5986cf7b)
}
pub fn gen_pub_new_ts(attr_ts: &dyn ToTokens, prms_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    let ts_73940779 = pub_space_ts(&gen_new_ts(&Ts2::new(), prms_ts, ts));
    with_attr_ts(attr_ts, &ts_73940779)
}
pub fn gen_pub_const_new_ts(
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts_5dc3668f = pub_space_ts(&gen_const_new_ts(&Ts2::new(), prms_ts, ts));
    with_attr_ts(attr_ts, &ts_5dc3668f)
}
pub fn gen_impl_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_new_ts(attr_ts, prms_ts, ts))
}
pub fn gen_impl_const_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_const_new_ts(attr_ts, prms_ts, ts))
}
pub fn gen_impl_pub_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_pub_new_ts(attr_ts, prms_ts, ts))
}
pub fn gen_impl_pub_const_new_for_ident_ts(
    ident_ts: &dyn ToTokens,
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_pub_const_new_ts(attr_ts, prms_ts, ts))
}
pub fn gen_try_new_ts(
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        #attr_ts
        fn try_new(#prms_ts) -> Result<Self, #err_type_ts> {
            #ts
        }
    }
}
pub fn gen_const_try_new_ts(
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts0 = const_space_ts(&gen_try_new_ts(&Ts2::new(), prms_ts, err_type_ts, ts));
    with_attr_ts(attr_ts, &ts0)
}
pub fn gen_pub_try_new_ts(
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts0 = pub_space_ts(&gen_try_new_ts(&Ts2::new(), prms_ts, err_type_ts, ts));
    with_attr_ts(attr_ts, &ts0)
}
pub fn gen_pub_const_try_new_ts(
    attr_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let ts0 = pub_space_ts(&gen_const_try_new_ts(&Ts2::new(), prms_ts, err_type_ts, ts));
    with_attr_ts(attr_ts, &ts0)
}
pub fn gen_impl_try_new_for_ident_ts(
    attr_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(ident_ts, &gen_try_new_ts(attr_ts, prms_ts, err_type_ts, ts))
}
pub fn gen_impl_const_try_new_for_ident_ts(
    attr_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(
        ident_ts,
        &gen_const_try_new_ts(attr_ts, prms_ts, err_type_ts, ts),
    )
}
pub fn gen_impl_pub_try_new_for_ident_ts(
    attr_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(
        ident_ts,
        &gen_pub_try_new_ts(attr_ts, prms_ts, err_type_ts, ts),
    )
}
pub fn gen_impl_pub_const_try_new_for_ident_ts(
    attr_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    prms_ts: &dyn ToTokens,
    err_type_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    impl_ident_ts(
        ident_ts,
        &gen_pub_const_try_new_ts(attr_ts, prms_ts, err_type_ts, ts),
    )
}
#[cfg(test)]
mod tests {
    use super::{
        gen_impl_const_new_for_ident_ts, gen_impl_new_for_ident_ts,
        gen_impl_pub_const_new_for_ident_ts,
    };
    use proc_macro2::TokenStream as Ts2;
    fn cmpct(v: &str) -> String {
        v.split_whitespace().collect::<String>()
    }
    fn empty_ts() -> Ts2 {
        Ts2::new()
    }
    #[test]
    fn gen_impl_new_for_ident_ts_generates_non_const_new() {
        let ident: Ts2 = "Cfg".parse().expect("48495be4");
        let prms: Ts2 = "v:usize".parse().expect("db75b4fb");
        let body: Ts2 = "Self{v}".parse().expect("7ad6dd07");
        let ts = gen_impl_new_for_ident_ts(&ident, &empty_ts(), &prms, &body);
        assert_eq!(
            cmpct(&ts.to_string()),
            cmpct("impl Cfg { fn new(v:usize) -> Self { Self{v} } }")
        );
    }
    #[test]
    fn gen_impl_const_new_for_ident_ts_generates_const_new() {
        let ident: Ts2 = "Cfg".parse().expect("7795af9b");
        let prms: Ts2 = "v:usize".parse().expect("28ccdfc4");
        let body: Ts2 = "Self{v}".parse().expect("46fb1c80");
        let ts = gen_impl_const_new_for_ident_ts(&ident, &empty_ts(), &prms, &body);
        assert_eq!(
            cmpct(&ts.to_string()),
            cmpct("impl Cfg { const fn new(v:usize) -> Self { Self{v} } }")
        );
    }
    #[test]
    fn gen_impl_pub_const_new_for_ident_ts_generates_pub_const_new() {
        let ident: Ts2 = "Cfg".parse().expect("4afbe04b");
        let attr: Ts2 = "#[inline]".parse().expect("5cfde4dd");
        let prms: Ts2 = "v:usize".parse().expect("4304ab24");
        let body: Ts2 = "Self{v}".parse().expect("29ac89d5");
        let ts = gen_impl_pub_const_new_for_ident_ts(&ident, &attr, &prms, &body);
        assert_eq!(
            cmpct(&ts.to_string()),
            cmpct("impl Cfg { #[inline] pub const fn new(v:usize) -> Self { Self{v} } }")
        );
    }
}
