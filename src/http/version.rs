pub enum Version {
    Http1_1,
}

impl Version {
    fn as_str(&self) -> &'static str {
        match &self {
            Version::Http1_1 => "HTTP/1.1",
        }
    }
}