use actix_web::{HttpResponse, ResponseError};
use config::ConfigError;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;
use sqlx::types::uuid::Error as SqlxUuidError;
use std::convert::Infallible;
use std::error::Error as libError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error as IoError;
use std::num::{ParseFloatError, ParseIntError};
use std::process;
use std::sync::PoisonError;
use thiserror::Error as ThisError;
use uuid::Error as UuidError;

const RECORD_NOT_EXIST: &'static str = "record_not_exists";
const SYSTEM_ERROR: &'static str = "system_error";

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

    #[error("record not found error")]
    NotFound,

    #[error("failed to poison error. {0}")]
    PoisonError(String),

    #[error("failed to configuration error. {0}")]
    ConfigError(String),

    #[error("failed to convent error. {0}")]
    ConventError(String),

    #[error("failed to uuid param error. {0}")]
    UUIDError(String),

    #[error("failed to parse date time error. {0}")]
    DateError(String),

    #[error("header Auth is empty")]
    AuthError,

    #[error("parse error. {0}")]
    ParseError(String),

    // #[error("failed to param error. {0}")]
    // ParamError(String),
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

impl From<Infallible> for Error {
    fn from(v: Infallible) -> Self {
        Error::ParseError(v.to_string())
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
        match v.as_database_error() {
            Some(err) => Error::DataBaseError(err.to_string()),
            None => match v {
                SqlxError::RowNotFound => Error::NotFound,
                _ => Error::DataBaseError(v.to_string()),
            },
        }
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

impl From<UuidError> for Error {
    fn from(v: UuidError) -> Self {
        Error::UUIDError(v.to_string())
    }
}

impl From<SqlxUuidError> for Error {
    fn from(v: SqlxUuidError) -> Self {
        Error::UUIDError(v.to_string())
    }
}

impl From<&str> for Error {
    fn from(v: &str) -> Self {
        Error::None(v.to_string())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ResponseErr {
    code: &'static str,
    msg: String,
}

// impl Display for ResponseErr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.msg)
//     }
// }

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,

            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Error::NotFound => HttpResponse::build(self.status_code()).json(ResponseErr {
                code: RECORD_NOT_EXIST,
                msg: self.to_string(),
            }),

            _ => HttpResponse::build(self.status_code()).json(ResponseErr {
                code: SYSTEM_ERROR,
                msg: self.to_string(),
            }),
        }
    }
}
