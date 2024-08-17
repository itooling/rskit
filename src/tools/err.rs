#[derive(Debug)]
pub enum BaseErr {
    StrError(String),
    EcdhError(String),
}
