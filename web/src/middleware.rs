#![allow(clippy::type_complexity)]
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;

use futures::future::{ok, Ready};
use futures::Future;
use actix_web::{dev::Service, dev::Transform, Error, HttpResponse};

use log::info;

use super::web_util;
pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S: Service<Req>, Req> Service<Req> for AuthMiddleware<S>{
    type Response = S::Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: Req) -> Self::Future {
        let mut srv = self.service.clone();
        Box::pin(async move {
            let path = req.path().to_string();
            info!("path:{}", path);
            if path.find("/admin").is_some()
                && web_util::get_username_from_session(&req.get_session()).is_none()
            {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            } else {
                let res_fut = srv.call(req);
                res_fut.await
            }
        })
    }
}

#[derive(Clone)]
pub struct AuthService {}

impl<S: Service<Req>, Req> Transform<S, Req> for AuthService
{
    
    type Response = S::Response;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}


