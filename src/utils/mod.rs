pub type Result<T> = std::result::Result<T, EmulatorError>;

#[derive(Debug)]
pub struct EmulatorError;
