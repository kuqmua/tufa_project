pub fn generate_path_from_segments(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> std::string::String {
    let mut segments_stringified = segments.iter().fold(std::string::String::new(), |mut acc, elem| {
        acc.push_str(&format!("{}::", elem.ident));
        acc
    });
    let _: std::option::Option<std::primitive::char> = segments_stringified.pop();
    let _: std::option::Option<std::primitive::char> = segments_stringified.pop();
    segments_stringified
}
