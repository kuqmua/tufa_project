pub fn generate_path_from_segments(
    segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
) -> std::string::String {
    let mut segments_stringified = segments.iter().fold(String::from(""), |mut acc, elem| {
        acc.push_str(&format!("{}::", elem.ident));
        acc
    });
    let _ = segments_stringified.pop();
    let _ = segments_stringified.pop();
    segments_stringified
}
