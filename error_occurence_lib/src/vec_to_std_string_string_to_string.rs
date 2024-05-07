pub trait VecToStdStringStringToString {
    fn vec_to_std_string_string_to_string(&self) -> std::string::String;
}

// impl<VecElementGeneric> VecToStdStringStringToString for Vec<VecElementGeneric>
// where
//     VecElementGeneric: to_std_string_string::ToStdStringString,
// {
//     fn vec_to_std_string_string_to_string(&self) -> std::string::String {
//         crate::helpers::stringified_lines_error_vec(self.iter().fold(
//             std::string::String::from(""),
//             |mut acc, vec_element| {
//                 acc.push_str(&format!(" {}\n", vec_element.to_std_string_string()));
//                 acc
//             },
//         ))
//     }
// }
