use actix_web::HttpResponse;
use http::StatusCode;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Result as SerdeResult;

#[derive(Deserialize)]
pub struct Response {
    pub code: u16,
    pub data: String,
    pub msg: String,
}

pub trait ResponseT {
    fn default() -> Self;
    fn new_success(data: &str) -> Self;
    fn new(data: &str, code: u16, msg: &str) -> Self;
    fn response_ok(&self) -> HttpResponse;
}

impl ResponseT for Response {
    #[allow(dead_code)]
    fn default() -> Self {
        Response {
            code: StatusCode::default().as_u16(),
            data: String::new(),
            msg: String::new(),
        }
    }

    #[allow(dead_code)]
    fn new_success(data: &str) -> Self {
        Response {
            code: StatusCode::OK.as_u16(),
            data: data.to_string(),
            msg: String::new(),
        }
    }

    #[allow(dead_code)]
    fn new(data: &str, code: u16, msg: &str) -> Self {
        Response {
            code,
            data: data.to_string(),
            msg: msg.to_string(),
        }
    }

    fn response_ok(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(mime::APPLICATION_JSON)
            .json(self)
    }
}

impl Response {
    #[allow(dead_code)]
    fn json_marshal(&self) -> SerdeResult<String> {
        serde_json::to_string(&self)
    }

    #[allow(dead_code)]
    fn json_unmarshal(s: &[u8]) -> SerdeResult<Self> {
        serde_json::from_slice(s)
    }
}

impl Serialize for Response {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut s = serializer.serialize_struct("Response", 1)?;
        s.serialize_field("code", &self.code)?;
        s.serialize_field("data", &self.data)?;
        s.serialize_field("msg", &self.msg)?;
        s.end()
    }
}
