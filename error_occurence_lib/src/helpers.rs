//todo later - make them private
pub fn stringified_lines_error_vec<T: std::fmt::Display>(stringified_vec: T) -> std::string::String {
    format!("[\n{}]", stringified_vec)
}

pub fn stringified_lines_error_hashmap_element<K: std::fmt::Display, V: std::fmt::Display>(
    key: K, //todo - it can possibly contains more than one line
    value: V,
) -> std::string::String {
    format!("{}: [\n{}]\n", key, lines_space_backslash_addition(value))
}

pub fn lines_space_backslash_addition<T: std::fmt::Display>(value: T) -> std::string::String {
    value
        .to_string()
        .lines()
        .fold(std::string::String::new(), |mut acc, line| {
            acc.push_str(&format!(" {}\n", line));
            acc
        })
}

pub fn source_and_code_occurence_formatter<Source: std::fmt::Display, CodeOccurence: std::fmt::Display>(
    stringified_source: Source,
    stringified_code_occurence: CodeOccurence,
) -> std::string::String {
    format!("{}\n{}", stringified_source, stringified_code_occurence)
}

pub fn error_occurence_hashmap_formatter<T: std::fmt::Display>(inner: T) -> std::string::String {
    format!("{{\n{}}}", lines_space_backslash_addition(inner))
}
