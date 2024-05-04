
pub enum Method {
    GET,
    POST,
}
impl Method {
    pub fn from_string(word: &str) -> Option<Self> {
        match word {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            _ => None,
        }
    }
}
