use regex::Regex;

pub enum RegexPattern {
    Email,
}

impl RegexPattern {
    pub fn pattern(&self) -> &'static str {
        match self {
            RegexPattern::Email => r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$",
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        let re = Regex::new(self.pattern()).unwrap();
        re.is_match(text)
    }
}
