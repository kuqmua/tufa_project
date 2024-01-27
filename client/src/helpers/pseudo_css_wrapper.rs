#[derive(PartialEq, Clone)]
pub struct PseudoCssWrapper {
    pub style: std::collections::HashMap<String, std::string::String>,
}

impl PseudoCssWrapper {
    pub fn to_string(&self) -> std::string::String {
        let mut formated = std::string::String::from("");
        self.style.iter().for_each(|(k, v)| {
            formated.push_str(&format!("{}: {};", k, v));
        });
        formated
    }
}
