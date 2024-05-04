pub enum ContentType {
    TextPlain
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain"
        }
    }
}