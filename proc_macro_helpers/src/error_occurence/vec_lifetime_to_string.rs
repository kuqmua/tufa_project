pub fn vec_lifetime_to_string(vec: &[crate::error_occurence::lifetime::Lifetime]) -> std::string::String {
    let mut lifetimes_stringified_handle = vec.iter().fold(std::string::String::new(), |mut acc, path_segment| {
        acc.push_str(&path_segment.to_string());
        acc
    });
    let _: std::option::Option<std::primitive::char> = lifetimes_stringified_handle.pop();
    format!("<{lifetimes_stringified_handle}>")
}
