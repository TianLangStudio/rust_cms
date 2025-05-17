#![allow(clippy::type_complexity)]
use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use std::future::{ready, Ready};

use actix_web::{dev, dev::Service, dev::Transform, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;

use log::info;

use super::web_util;
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    //type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        info!("path:{}", path);
        if path.contains("/admin")
            && web_util::get_username_from_session(&req.get_session()).is_none()
        {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();

            Box::pin(async { Ok(ServiceResponse::new(request, response)) })
            //Ok(req.into_response(actix_web::error::ErrorNetworkAuthenticationRequired("Unauthenticated")))
        } else {
            let res = self.service.call(req);
            Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
        }
    }
}

#[derive(Clone)]
pub struct AuthService;

impl<S, B> Transform<S, ServiceRequest> for AuthService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    //type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
