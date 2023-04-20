use actix_web::{web, HttpResponse, Resource, Responder};
use http::StatusCode;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Result as SerdeResult;

#[allow(dead_code)]
async fn get_greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.into_inner())
}

#[allow(dead_code)]
async fn post_greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(mime::APPLICATION_JSON)
        .json(Greet::new_success(name.into_inner().as_str()))
}

#[allow(dead_code)]
pub fn score() -> Vec<Resource> {
    let mut r: Vec<Resource> = vec![];

    r.push(
        web::resource("/hello/{name}")
            .route(web::get().to(get_greet))
            .route(web::post().to(post_greet)),
    );

    r
}

#[derive(Deserialize)]
struct Greet {
    pub code: u16,
    pub data: String,
}

impl Greet {
    #[allow(dead_code)]
    fn default() -> Self {
        Greet {
            data: String::new(),
            code: StatusCode::default().as_u16(),
        }
    }

    #[allow(dead_code)]
    fn new_success(data: &str) -> Self {
        Greet {
            data: data.to_string(),
            code: StatusCode::OK.as_u16(),
        }
    }

    #[allow(dead_code)]
    fn new(data: &str, code: u16) -> Self {
        Greet {
            data: data.to_string(),
            code,
        }
    }

    #[allow(dead_code)]
    fn json_marshal(&self) -> SerdeResult<String> {
        serde_json::to_string(&self)
    }

    #[allow(dead_code)]
    fn json_unmarshal(s: &[u8]) -> SerdeResult<Self> {
        serde_json::from_slice(s)
    }
}

impl Serialize for Greet {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut s = serializer.serialize_struct("Greet", 1)?;
        s.serialize_field("code", &self.code)?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}
