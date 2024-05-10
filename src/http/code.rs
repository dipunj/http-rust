/// Represents the Status codes used by HTTP
/// https://en.wikipedia.org/wiki/List_of_HTTP_status_codes


#[derive(PartialEq, Eq)]
pub enum HttpCode {
    OK = 200,
    CREATED = 201,
    BadRequest = 400,
    NotFound = 404,
}

impl HttpCode {
    pub fn to_string(&self) -> &'static str {
        match self {
            HttpCode::OK => "200 OK",
            HttpCode::CREATED => "201 Created",
            HttpCode::BadRequest => "400 Bad Request",
            HttpCode::NotFound => "404 Not Found"
        }
    }
}