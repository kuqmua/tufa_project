use crate::panic_if_err::panic_if_err;
use proc_macro2::TokenStream as Ts2;
use syn::{Attribute, Meta};
const NO_ATTR_ER: &str = "no_attr";
const ATTR_NOT_LIST_ER: &str = "attr_not_list";
#[allow(clippy::single_call_fn)] // helper keeps segment comparison logic isolated and reusable for future attr queries
fn attr_path_matches(attr: &Attribute, attr_path: &str) -> bool {
    let mut attr_segments = attr.path().segments.iter();
    let mut expected_segments = attr_path
        .split("::")
        .map(str::trim)
        .filter(|el| !el.is_empty());
    loop {
        match (attr_segments.next(), expected_segments.next()) {
            (Some(attr_segment), Some(expected_segment)) => {
                if attr_segment.ident != expected_segment {
                    return false;
                }
            }
            (None, None) => {
                return true;
            }
            (Some(_), None) | (None, Some(_)) => {
                return false;
            }
        }
    }
}
#[must_use]
pub fn find_macro_attr<'lt>(attrs: &'lt [Attribute], attr_path: &str) -> Option<&'lt Attribute> {
    attrs.iter().find(|attr| attr_path_matches(attr, attr_path))
}
pub fn try_get_macro_attr<'lt>(
    attrs: &'lt [Attribute],
    attr_path: &str,
) -> Result<&'lt Attribute, &'static str> {
    find_macro_attr(attrs, attr_path).ok_or(NO_ATTR_ER)
}
pub fn try_get_macro_attr_meta_list_ts<'lt>(
    attrs: &'lt [Attribute],
    attr_path: &str,
) -> Result<&'lt Ts2, &'static str> {
    let attr = try_get_macro_attr(attrs, attr_path)?;
    if let Meta::List(v) = &attr.meta {
        Ok(&v.tokens)
    } else {
        Err(ATTR_NOT_LIST_ER)
    }
}
#[must_use]
pub fn get_macro_attr<'lt>(attrs: &'lt [Attribute], attr_path: &str) -> &'lt Attribute {
    panic_if_err(try_get_macro_attr(attrs, attr_path), |er| {
        format!("68acaa15:{er}:{attr_path}")
    })
}
#[must_use]
pub fn get_macro_attr_meta_list_ts<'lt>(attrs: &'lt [Attribute], attr_path: &str) -> &'lt Ts2 {
    panic_if_err(try_get_macro_attr_meta_list_ts(attrs, attr_path), |er| {
        format!("9d057161:{er}:{attr_path}")
    })
}
#[cfg(test)]
mod tests {
    use super::{
        find_macro_attr, get_macro_attr, get_macro_attr_meta_list_ts, try_get_macro_attr,
        try_get_macro_attr_meta_list_ts,
    };
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
    #[test]
    fn find_macro_attr_returns_none_when_path_not_present() {
        let attrs = attrs();
        assert!(find_macro_attr(&attrs, "missing::attr").is_none());
    }
    #[test]
    fn try_get_macro_attr_returns_error_when_attr_not_found() {
        let attrs = attrs();
        assert_eq!(try_get_macro_attr(&attrs, "missing::attr"), Err("no_attr"));
    }
    #[test]
    fn try_get_macro_attr_meta_list_ts_returns_error_for_non_list_attr() {
        let attrs = vec![parse_quote!(#[allow])];
        assert!(matches!(
            try_get_macro_attr_meta_list_ts(&attrs, "allow"),
            Err("attr_not_list")
        ));
    }
    #[test]
    fn find_macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = find_macro_attr(&attrs, "sqlx :: type_name");
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_accepts_leading_colons_in_lookup_path() {
        let attrs = attrs();
        let attr = find_macro_attr(&attrs, " :: sqlx :: type_name ");
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_empty_lookup_path() {
        let attrs = attrs();
        let attr = find_macro_attr(&attrs, " :: ");
        assert!(attr.is_none());
    }
    #[test]
    fn find_macro_attr_ignores_empty_segments_between_path_separators() {
        let attrs = attrs();
        let attr = find_macro_attr(&attrs, "sqlx::::type_name");
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_partial_path_match() {
        let attrs = attrs();
        let attr = find_macro_attr(&attrs, "sqlx");
        assert!(attr.is_none());
    }
}
