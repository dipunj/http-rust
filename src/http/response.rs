use crate::http::content_type::ContentType;
use crate::http::header::ResponseHeader;

struct Response {
    header: ResponseHeader,
    content_type: ContentType,
    content_length: u32,
    content: String
}

impl Response {
    fn to_string(&self) -> String {
        self.header.to_string() + self.content_type.as_str() + &self.content.len().to_string() + &self.content
    }

    fn
}