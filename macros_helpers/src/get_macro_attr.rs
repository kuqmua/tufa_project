use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
use syn::{Attribute, Meta};
fn normalize_path(path: &str) -> String {
    let mut out = path.to_owned();
    out.retain(|ch| !ch.is_whitespace());
    out
}
fn find_attr_by_path<'lt>(attrs: &'lt [Attribute], attr_path: &str) -> Option<&'lt Attribute> {
    let attr_path_norm = normalize_path(attr_path);
    attrs.iter().find(|el| {
        attr_path_norm == normalize_path(&ToTokens::to_token_stream(&el.path()).to_string())
    })
}
#[must_use]
pub fn get_macro_attr<'lt>(attrs: &'lt [Attribute], attr_path: &str) -> &'lt Attribute {
    find_attr_by_path(attrs, attr_path).expect("68acaa15")
}
#[must_use]
pub fn get_macro_attr_meta_list_ts<'lt>(attrs: &'lt [Attribute], attr_path: &str) -> &'lt Ts2 {
    let attr = find_attr_by_path(attrs, attr_path).expect("9d057161");
    if let Meta::List(v) = &attr.meta {
        &v.tokens
    } else {
        panic!("985dc2d5")
    }
}
#[cfg(test)]
mod tests {
    use super::{get_macro_attr, get_macro_attr_meta_list_ts};
    use quote::quote;
    use syn::{Attribute, parse_quote};
    fn attrs() -> Vec<Attribute> {
        vec![
            parse_quote!(#[sqlx::type_name(name = "x")]),
            parse_quote!(#[serde(default)]),
        ]
    }
    #[test]
    fn get_macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = get_macro_attr(&attrs, "sqlx :: type_name");
        assert!(quote! {#attr}.to_string().contains("sqlx :: type_name"));
    }
    #[test]
    fn get_macro_attr_meta_list_ts_returns_list_tokens() {
        let attrs = attrs();
        let ts = get_macro_attr_meta_list_ts(&attrs, "serde");
        assert_eq!(ts.to_string(), "default");
    }
}
