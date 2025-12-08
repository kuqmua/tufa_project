#[derive(PartialEq, Eq, Clone)]
pub struct PseudoCssWrapper {
    pub style: std::collections::HashMap<String, String>,
}

impl PseudoCssWrapper {
    pub fn to_string(&self) -> String {
        let mut formated = String::new();
        self.style.iter().for_each(|(k, v)| {
            formated.push_str(&format!("{}: {};", k, v));
        });
        formated
    }
}
