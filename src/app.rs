pub struct App {
    pub title: String,
    pub enhanced_graphics: bool,
}

impl App {
    pub fn new(title: &str, enhanced_graphics: bool) -> App {
        App {
            title: title.to_string(),
            enhanced_graphics,
        }
    }
}
