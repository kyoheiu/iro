#[derive(Debug)]
pub struct IroError(pub String);

impl From<std::num::ParseIntError> for IroError {
    fn from(err: std::num::ParseIntError) -> Self {
        IroError(err.to_string())
    }
}
