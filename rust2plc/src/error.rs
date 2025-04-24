pub enum Rust2PlcError {
    IoError(std::io::Error),
    ParseError(String),
    Other(String),
}

impl Rust2PlcError {
    pub fn parse(msg: String) -> Self {
        Rust2PlcError::ParseError(msg)
    }
}