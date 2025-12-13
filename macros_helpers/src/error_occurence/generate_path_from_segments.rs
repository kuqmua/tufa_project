pub fn generate_path_from_segments(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> String {
    let mut segments_stringified = segments.iter().fold(String::new(), |mut acc, elem| {
        use std::fmt::Write as _;
        assert!(write!(acc, "{}::", elem.ident).is_ok(), "error 87a234a5-8086-474d-b780-57e77c4add43");
        acc
    });
    let _: Option<char> = segments_stringified.pop();
    let _: Option<char> = segments_stringified.pop();
    segments_stringified
}
