#[derive(Debug)]
pub enum Error {
    InputError(String),
    InvalidMessage(String),
    NoNodeId,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::InputError(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::InputError(value.to_string())
    }
}
