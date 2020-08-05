#![allow(clippy::type_complexity)]
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use futures::future::{ok, Ready};
use futures::Future;

use actix_service::{Service, Transform};
use actix_session::UserSession;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};

use log::info;

use super::web_util;

#[derive(Clone)]
pub struct AuthService {}

impl<S, B> Transform<S> for AuthService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
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

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
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
