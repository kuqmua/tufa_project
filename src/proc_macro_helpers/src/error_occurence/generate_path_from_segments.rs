pub fn generate_path_from_segments(
    segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>,
) -> std::string::String {
    let mut segments_stringified = segments.iter().fold(String::from(""), |mut acc, elem| {
        acc.push_str(&format!("{}::", elem.ident));
        acc
    });
    segments_stringified.pop();
    segments_stringified.pop();
    segments_stringified
}
