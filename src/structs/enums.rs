pub enum SnackbardColor {
    Error,
    Success,
    // Warning,
}

impl SnackbardColor {
    pub fn get_status(&self) -> String {
        match self {
            SnackbardColor::Error => String::from("error"),
            SnackbardColor::Success => String::from("success"),
            // SnackbardColor::Warning => String::from("warning"),
        }
    }
}
