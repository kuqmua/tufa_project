pub fn impl_from_str_for_enum_helper<T>(v: &str, vrts: &[(&str, T)]) -> Result<T, String>
where
    T: Copy,
{
    for &(str_vrt, enum_vrt) in vrts {
        if v.eq_ignore_ascii_case(str_vrt) {
            return Ok(enum_vrt);
        }
    }
    let cap = vrts
        .iter()
        .map(|(name, _)| name.len())
        .sum::<usize>()
        .saturating_add(vrts.len().saturating_sub(1).saturating_mul(2));
    let mut allowed_values = String::with_capacity(cap);
    for (k_6e44a22d, (name, _)) in vrts.iter().enumerate() {
        if k_6e44a22d != 0 {
            allowed_values.push_str(", ");
        }
        allowed_values.push_str(name);
    }
    Err(format!(
        "Unknown value: {v}. Allowed values: {allowed_values}"
    ))
}
#[cfg(test)]
mod tests {
    use super::impl_from_str_for_enum_helper;
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
}
