const ALLOWED_VALUES_SEPARATOR: &str = ", ";
#[allow(clippy::single_call_fn)] // extracted for reuse by allowed-values formatter and tests
fn allowed_values_capacity<T>(vrts: &[(&str, T)]) -> usize {
    vrts.iter()
        .map(|(name, _)| name.len())
        .sum::<usize>()
        .saturating_add(
            vrts.len()
                .saturating_sub(1)
                .saturating_mul(ALLOWED_VALUES_SEPARATOR.len()),
        )
}
#[allow(clippy::single_call_fn)] // extracted to keep allowed-values formatting reusable and tested
fn mk_allowed_values<T>(vrts: &[(&str, T)]) -> String {
    let mut allowed_values = String::with_capacity(allowed_values_capacity(vrts));
    for (k_6e44a22d, (name, _)) in vrts.iter().enumerate() {
        if k_6e44a22d != 0 {
            allowed_values.push_str(ALLOWED_VALUES_SEPARATOR);
        }
        allowed_values.push_str(name);
    }
    allowed_values
}
#[allow(clippy::single_call_fn)] // extracted lookup keeps case-insensitive enum-pair search reusable and testable
fn find_case_insensitive_pair<T>(v: &str, vrts: &[(&str, T)]) -> Option<T>
where
    T: Copy,
{
    vrts.iter()
        .find_map(|(str_vrt, enum_vrt)| v.eq_ignore_ascii_case(str_vrt).then_some(*enum_vrt))
}
pub fn impl_from_str_for_enum_helper<T>(v: &str, vrts: &[(&str, T)]) -> Result<T, String>
where
    T: Copy,
{
    find_case_insensitive_pair(v, vrts).ok_or_else(|| {
        let allowed_values = mk_allowed_values(vrts);
        format!("Unknown value: {v}. Allowed values: {allowed_values}")
    })
}
#[cfg(test)]
mod tests {
    use super::{find_case_insensitive_pair, impl_from_str_for_enum_helper, mk_allowed_values};
    const PAIRS: [(&str, V); 2] = [("a", V::A), ("b", V::B)];
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum V {
        A,
        B,
    }
    #[test]
    fn helper_parses_values_case_insensitively() {
        assert_eq!(impl_from_str_for_enum_helper("A", &PAIRS), Ok(V::A));
        assert_eq!(impl_from_str_for_enum_helper("b", &PAIRS), Ok(V::B));
    }
    #[test]
    fn find_case_insensitive_pair_returns_none_for_unknown_value() {
        assert_eq!(find_case_insensitive_pair("x", &PAIRS), None);
    }
    #[test]
    fn helper_error_mentions_allowed_values() {
        let er = impl_from_str_for_enum_helper("x", &PAIRS).expect_err("4d6330e7");
        assert!(er.contains("Unknown value: x"));
        assert!(er.contains("Allowed values: a, b"));
    }
    #[test]
    fn helper_error_keeps_variant_order_in_allowed_values() {
        let pairs = [("first", V::A), ("second", V::B)];
        let er = impl_from_str_for_enum_helper("x", &pairs).expect_err("ee52fc8d");
        assert!(er.contains("Allowed values: first, second"));
    }
    #[test]
    fn helper_error_handles_empty_variants() {
        let pairs: [(&str, V); 0] = [];
        let er = impl_from_str_for_enum_helper("x", &pairs).expect_err("312f79de");
        assert_eq!(er, "Unknown value: x. Allowed values: ");
    }
    #[test]
    fn mk_allowed_values_formats_multiple_variants() {
        assert_eq!(mk_allowed_values(&PAIRS), "a, b");
    }
    #[test]
    fn mk_allowed_values_returns_empty_for_no_variants() {
        let pairs: [(&str, V); 0] = [];
        assert_eq!(mk_allowed_values(&pairs), "");
    }
}
