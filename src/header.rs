use std::fs;
use std::io::Read;

pub struct Header {
    content: String,
}

impl Header {
    pub fn new() -> Header {
        let file_path = format!("{}/src/hi.you", env!("CARGO_MANIFEST_DIR"));
        let mut file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)
            .unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content);

        Header { content }
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }
}
