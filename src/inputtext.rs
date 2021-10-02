pub struct InputText {
    content: String,    
}

impl InputText {
    pub fn new() -> InputText {
        InputText {
            content: "".to_string()
        }
    }
}
