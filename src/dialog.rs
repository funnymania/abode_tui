pub struct Dialog {
    pub title: String,
    pub content: String,
    pub options: (String, String),
}
//TODO Dialog needs buttons
impl Dialog {
    pub fn new(title: &str, content: String) -> Dialog {
        Dialog {
            title: title.to_string(),
            content: content,
            options: ("Go back".to_string(), "Yes, please.".to_string()),
        }
    }
}
