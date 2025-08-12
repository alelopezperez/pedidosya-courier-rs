pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}
impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

pub enum ContentType {
    Json,
    Pdf,
    Unsoported(String),
    None,
}

impl From<Option<&str>> for ContentType {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some(text) => {
                if text.starts_with("application/json") {
                    ContentType::Json
                } else if text.starts_with("application/pdf") {
                    ContentType::Pdf
                } else {
                    ContentType::Unsoported(text.to_owned())
                }
            }
            None => ContentType::None,
        }
    }
}

mod shippings;
