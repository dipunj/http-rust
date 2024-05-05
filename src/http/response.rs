use crate::http::code::HttpCode;
use crate::http::content_type::ContentType;
use crate::http::header::ResponseHeader;
use crate::http::version::Version;

pub struct Response {
    header: ResponseHeader,
    content_type: ContentType,
    content_length: u32,
    content: String
}

impl Response {
    pub fn to_string(&self) -> String {
        let header_string = self.header.to_string();
        let content_length = self.content.len().to_string();
        let content_type = self.content_type.as_str().to_string();

        // Create a vector containing the sections of the response
        let sections = vec![
            header_string,
            format!("Content-Type: {}", content_type),
            format!("Content-Length: {}", content_length),
            "".to_string(), // Empty line separator
            self.content.to_string(),
        ];

        // Concatenate sections with "\r\n"
        let response_string = sections.join("\r\n");

        response_string
    }

    pub(crate) fn new(protocol: Version,
                      http_code: HttpCode,
                      content_type: ContentType,
                      content: String) -> Self {
        Self {
            header: ResponseHeader::new(protocol, http_code),
            content_type,
            content_length: content.len() as u32,
            content,
        }
    }
}