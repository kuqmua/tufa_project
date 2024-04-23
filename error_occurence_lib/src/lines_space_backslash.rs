pub trait LinesSpaceBackslash {
    fn lines_space_backslash(&self) -> std::string::String;
}

impl<SelfGeneric> LinesSpaceBackslash for SelfGeneric
where
    SelfGeneric: std::fmt::Display,
{
    fn lines_space_backslash(&self) -> std::string::String {
        self.to_string()
            .lines()
            .fold(std::string::String::new(), |mut acc, line| {
                acc.push_str(&format!(" {}\n", line));
                acc
            })
    }
}
