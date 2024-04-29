pub trait VecToStdStringStringIntoVecString {
    fn vec_to_std_string_string_into_vec_string(self) -> Vec<String>;
}

impl<VecElementGeneric> VecToStdStringStringIntoVecString for Vec<VecElementGeneric>
where
    VecElementGeneric: to_std_string_string::ToStdStringString,
{
    fn vec_to_std_string_string_into_vec_string(self) -> Vec<String> {
        self.into_iter().map(|i| i.to_std_string_string()).collect()
    }
}
