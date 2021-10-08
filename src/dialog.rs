//TODO: Dialog (actually perhaps any UI Element) should have its own layout associated with it
//TODO: Dialog should possibly be a trait.
pub struct Dialog {
    pub title: String,
    pub content: String,
    pub options: (String, String),
}

impl Dialog {
    pub fn new(title: &str, content: String) -> Dialog {
        Dialog {
            title: title.to_string(),
            content: content,
            options: ("Go back".to_string(), "Yes, please.".to_string()),
        }
    }
}
