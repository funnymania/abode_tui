pub struct Dialog {
    pub title: String,
    pub content: String,
}

impl Dialog {
    pub fn new(title: &str, content: &str) -> Dialog {
        Dialog {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}
