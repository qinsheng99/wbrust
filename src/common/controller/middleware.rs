use {
    actix_web::{
        body::MessageBody,
        dev::{Service, ServiceRequest, ServiceResponse, Transform},
        Error,
    },
    http::HeaderValue,
    std::{
        cell::RefCell,
        future::{ready, Future, Ready},
        pin::Pin,
        rc::Rc,
        task::{Context, Poll},
    },
};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = S::Response;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = S::Response;
    type Error = S::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let s = self.service.clone();

        Box::pin(async move {
            let v = HeaderValue::from_str("").unwrap();
            let auth = req.headers().get("Auth").unwrap_or(&v);

            if auth.len() > 0 {
                Ok(s.call(req).await?)
            } else {
                Err(actix_web::error::ErrorUnauthorized(
                    crate::utils::error::Error::AuthError,
                ))
            }
        })
    }
}
