use actix_web::{web, HttpResponse, Resource, Responder};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Result as SerdeResult;

#[allow(dead_code)]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(name.into_inner())
}

pub fn score() -> Resource {
    web::resource("/hello/{name}").route(web::get().to(greet))
}

#[derive(Deserialize)]
struct Greet {
    pub data: String,
}

impl Greet {
    #[allow(dead_code)]
    fn default() -> Self {
        Greet {
            data: String::new(),
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
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}
