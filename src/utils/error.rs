use {
    actix_web::{HttpResponse, ResponseError},
    config::ConfigError,
    http::StatusCode,
    redis::RedisError,
    sea_orm::DbErr,
    serde::{Deserialize, Serialize},
    sqlx::{error::Error as SqlxError, types::uuid::Error as SqlxUuidError},
    std::{
        convert::Infallible,
        error::Error as libError,
        fmt::{Debug, Display, Formatter},
        io::Error as IoError,
        num::{ParseFloatError, ParseIntError},
        process,
        sync::PoisonError,
    },
    thiserror::Error as ThisError,
    uuid::Error as UuidError,
};

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

    #[error("failed to new sea-orm database error. {0}")]
    NewDataBaseError(String),

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

    #[error("redis error. {0}")]
    RedisError(String),

    // #[error("failed to param error. {0}")]
    // ParamError(String),
    #[error("")]
    #[allow(dead_code)]
    None(String),

    #[error("fn no implement")]
    #[allow(dead_code)]
    ImplementError,

    #[error("error. {0}")]
    #[allow(dead_code)]
    General(String),
}

impl Error {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Error::None("".to_string())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(v: PoisonError<T>) -> Self {
        Error::PoisonError(v.to_string())
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

#[allow(unused_macros)]
macro_rules! errfrom {
    ($($st:ty),* => $variant:ident) => (
        $(
            impl From<$st> for Error {
                fn from(e: $st) -> Error {
                    Error::$variant(e.to_string())
                }
            }
        )*
    )
}

//General

#[allow(unused_macros)]
macro_rules! generalerrorfrom {
    ($($st:ty),*) => (
        $(
            impl From<$st> for Error {
                fn from(e: $st) -> Error {
                    Error::General(format!({:?}, e))
                }
            }
        )*
    )
}

errfrom!(RedisError => RedisError);
errfrom!(DbErr => NewDataBaseError);
errfrom!(ConfigError => ConfigError);
errfrom!(Infallible => ParseError);
errfrom!(IoError => IOError);
errfrom!(ParseIntError, ParseFloatError => ConventError);
errfrom!(UuidError, SqlxUuidError => UUIDError);

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
            // Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
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
