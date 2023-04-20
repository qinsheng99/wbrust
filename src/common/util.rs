use std::fmt::{Debug, Display, Formatter};

// #[derive(Debug)]
pub struct ErrorMsg(String);

pub type Result<T> = std::result::Result<T, ErrorMsg>;

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err is {}", self.0)
    }
}

impl Debug for ErrorMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("err_msg").field("err", &self.0).finish()
    }
}

impl ErrorMsg {
    #[allow(dead_code)]
    pub fn new(err: &str) -> ErrorMsg {
        ErrorMsg(err.to_string())
    }

    #[allow(dead_code)]
    pub fn set_err(&mut self, data: &str) {
        self.0 = data.to_string()
    }
}
