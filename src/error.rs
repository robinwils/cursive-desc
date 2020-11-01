use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    ParseStrError,
    UnknownAttr,
    Other(&'static str),
}

impl From<JsonError> for Error {
    /// Creates a new `Error::DeXml` from the given error
    #[inline]
    fn from(error: serde_json::Error) -> Error {
        Error::Json(error)
    }
}
