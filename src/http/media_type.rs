/// Represents the Status codes used by HTTP
/// https://www.iana.org/assignments/media-types/media-types.xhtml

pub enum MediaType {
    TextPlain,
    OctetStream
}

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::TextPlain => "text/plain",
            MediaType::OctetStream => "application/octet-stream"
        }
    }
}