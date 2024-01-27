pub fn vec_lifetime_to_string(vec: &[crate::error_occurence::lifetime::Lifetime]) -> std::string::String {
    let mut lifetimes_stringified_handle =
        vec.iter().fold(String::from(""), |mut acc, path_segment| {
            acc.push_str(&format!("{},", path_segment));
            acc
        });
    lifetimes_stringified_handle.pop();
    format!("<{lifetimes_stringified_handle}>")
}
