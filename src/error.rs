#[derive(Debug)]
pub enum Error {
    Xml(quick_xml::Error),
    ParseStrError,
    UnknownAttr,
    Other(&'static str)
}

impl From<quick_xml::Error> for Error {
    /// Creates a new `Error::DeXml` from the given error
    #[inline]
    fn from(error: quick_xml::Error) -> Error {
        Error::Xml(error)
    }
}
