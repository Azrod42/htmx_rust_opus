pub enum SnackbardColor {
    Error,
    // Warning,
}

impl SnackbardColor {
    pub fn get_status(&self) -> String {
        match self {
            SnackbardColor::Error => String::from("error"),
            // SnackbardColor::Warning => String::from("warning"),
        }
    }
}
