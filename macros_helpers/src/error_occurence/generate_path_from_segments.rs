pub fn generate_path_from_segments(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> String {
    let mut segments_stringified = segments.iter().fold(String::new(), |mut acc, elem| {
        use std::fmt::Write as _;
        if let Err(error) = write!(acc, "{}::", elem.ident) {
            panic!("error 9f50a356-2f57-44cd-876e-f1af7e293fd2 {error:#?}");
        }
        acc
    });
    let _: Option<char> = segments_stringified.pop();
    let _: Option<char> = segments_stringified.pop();
    segments_stringified
}
