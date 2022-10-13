use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct PseudoCssWrapper {
    pub style: HashMap<String, String>,
}

impl PseudoCssWrapper {
    pub fn to_string(&self) -> String {
        let mut formated = String::from("");
        self.style.iter().for_each(|(k, v)| {
            formated.push_str(&format!("{}: {};", k, v));
        });
        formated
    }
}
