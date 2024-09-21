use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    StrError(String),
    EcdhError(String),
}

impl Error {
    pub fn custom<T: Display>(val: T) -> Self {
        Error::StrError(val.to_string())
    }
}
