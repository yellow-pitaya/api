pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Redpitaya(u32),
    String(std::str::Utf8Error),
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::Redpitaya(code) => convert_string!(
                crate::rp::rp_GetError(*code as i32)
            ),
            Self::String(error) => format!("{}", error),
        };

        write!(f, "{}", description)
    }
}

impl std::convert::From<u32> for Error {
    fn from(code: u32) -> Self {
        Self::Redpitaya(code)
    }
}

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::String(error)
    }
}
