pub fn generate_path_from_segments(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> String {
    let mut segments_stringified = segments.iter().fold(String::new(), |mut acc, elem| {
        acc.push_str(&format!("{}::", elem.ident));
        acc
    });
    let _: Option<char> = segments_stringified.pop();
    let _: Option<char> = segments_stringified.pop();
    segments_stringified
}
