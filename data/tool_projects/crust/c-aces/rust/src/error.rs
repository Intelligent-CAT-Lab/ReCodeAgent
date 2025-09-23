#[derive(Debug)]
pub enum AcesError {
    GenericError(String),
}
pub type Result<T> = std::result::Result<T, AcesError>;