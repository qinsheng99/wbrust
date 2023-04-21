use std::error::Error as libError;
use std::fmt::{Debug, Display, Formatter};
use std::process;

// #[derive(Debug)]
pub struct ErrorMsg(Box<dyn libError>);

#[derive(Debug)]
struct ErrorStr(String);

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

impl libError for ErrorMsg {
    fn description(&self) -> &str {
        "error msg"
    }
}

impl Display for ErrorStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl libError for ErrorStr {
    fn description(&self) -> &str {
        self.0.as_str()
    }
}

impl ErrorMsg {
    #[allow(dead_code)]
    pub fn new(err: String) -> ErrorMsg {
        ErrorMsg(Box::new(ErrorStr(err)))
    }

    #[allow(dead_code)]
    pub fn set_err(&mut self, data: Box<dyn libError>) {
        self.0 = data
    }
}

#[allow(dead_code)]
pub fn err(msg: &str) {
    println!("{}", msg);

    process::exit(1)
}

#[allow(dead_code)]
pub fn err_msg<T>(msg: &str, err: T)
where
    T: Display + Sized,
{
    println!("{}, err: {}", msg, err);

    process::exit(1)
}
