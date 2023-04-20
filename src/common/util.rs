use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ErrorMsg {
    data: String,
}

pub type Result<T> = std::result::Result<T, ErrorMsg>;

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err is {}", self.data)
    }
}

impl ErrorMsg {
    pub fn new(err: &str) -> Self {
        ErrorMsg {
            data: err.to_string(),
        }
    }
}
