pub mod middleware;

use actix_web::HttpResponse;
use http::StatusCode;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Result as SerdeResult;

#[derive(Deserialize)]
pub struct Response<T>
where
    T: Serialize,
{
    pub code: u16,
    pub msg: String,
    pub data: T,
}

#[derive(Serialize)]
pub struct Str(String);

impl Str {
    pub fn new(s: String) -> Self {
        Str(s)
    }
}

pub trait ResponseT<T> {
    fn new_success(data: T) -> Self;
    fn new(data: T, code: u16, msg: &str) -> Self;
    fn response_ok(&self) -> HttpResponse;
}

impl<T: Serialize> ResponseT<T> for Response<T> {
    #[allow(dead_code)]
    fn new_success(data: T) -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            msg: String::new(),
            data,
        }
    }

    #[allow(dead_code)]
    fn new(data: T, code: u16, msg: &str) -> Self {
        Response {
            code,
            msg: msg.to_string(),
            data,
        }
    }

    fn response_ok(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(mime::APPLICATION_JSON)
            .json(self)
    }
}

impl<T: Serialize> Response<T> {
    #[allow(dead_code)]
    fn json_marshal(&self) -> SerdeResult<String> {
        serde_json::to_string(&self)
    }

    #[allow(dead_code)]
    fn json_unmarshal<'a, S: Deserialize<'a>>(s: &'a [u8]) -> SerdeResult<S> {
        serde_json::from_slice(s)
    }
}

impl<T: Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Response", 1)?;
        s.serialize_field("code", &self.code)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("msg", &self.msg)?;
        s.end()
    }
}
