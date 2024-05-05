
pub enum HttpCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl HttpCode {
    pub fn to_string(&self) -> &'static str {
        match self {
            HttpCode::OK => "200 OK",
            HttpCode::BadRequest => "400 Bad Request",
            HttpCode::NotFound => "404 Not Found"
        }
    }
}