use config::ConfigError;
use sqlx::error::Error as SqlxError;
use std::error::Error as libError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error as IoError;
use std::num::{ParseFloatError, ParseIntError};
use std::process;
use std::sync::PoisonError;
use thiserror::Error as ThisError;

#[derive(Debug)]
pub struct ErrorMsg(Box<dyn libError>);

#[derive(Debug)]
struct ErrorStr(String);

pub type Result<T> = std::result::Result<T, Error>;

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err is {}", self.0)
    }
}

// impl Debug for ErrorMsg {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("err_msg").field("err", &self.0).finish()
//     }
// }

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

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("failed to IO error. {0}")]
    IOError(String),

    #[error("failed to database error. {0}")]
    DataBaseError(String),

    #[error("failed to poison error. {0}")]
    PoisonError(String),

    #[error("failed to configuration error. {0}")]
    ConfigError(String),

    #[error("failed to convent error. {0}")]
    ConventError(String),

    #[error("")]
    #[allow(dead_code)]
    None(String),
}

impl Error {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Error::None("".to_string())
    }
}

impl From<IoError> for Error {
    fn from(v: IoError) -> Self {
        Error::IOError(v.to_string())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(v: PoisonError<T>) -> Self {
        Error::PoisonError(v.to_string())
    }
}

impl From<ConfigError> for Error {
    fn from(v: ConfigError) -> Self {
        Error::ConfigError(v.to_string())
    }
}

impl From<SqlxError> for Error {
    fn from(v: SqlxError) -> Self {
        Error::DataBaseError(v.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(v: ParseIntError) -> Self {
        Error::ConventError(v.to_string())
    }
}

impl From<ParseFloatError> for Error {
    fn from(v: ParseFloatError) -> Self {
        Error::ConventError(v.to_string())
    }
}
