pub enum ContentType {
    TextPlain,
    OctetStream
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::OctetStream => "application/octet-stream"
        }
    }
}