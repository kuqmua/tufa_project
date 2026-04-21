pub fn impl_from_str_for_enum_helper<T>(v: &str, vrts: &[(&str, T)]) -> Result<T, String>
where
    T: Copy,
{
    if let Some((_, enum_vrt)) = vrts
        .iter()
        .find(|(str_vrt, _)| v.eq_ignore_ascii_case(str_vrt))
    {
        return Ok(*enum_vrt);
    }
    let allowed_values = vrts
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(", ");
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
}
